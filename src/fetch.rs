use log::{debug, info, warn};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use std::{
    io::SeekFrom,
    path::{Path, PathBuf},
};
use tokio::{fs::File, io::AsyncReadExt, sync::Semaphore, task::JoinHandle};

use crate::options::OPTIONS;

static CONCURRENT_DOWNLOAD_LIMIT: Lazy<Semaphore> =
    Lazy::new(|| Semaphore::new(OPTIONS.concurrent.into()));

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:75.0) Gecko/20100101 Firefox/75.0")
        .build()
        .unwrap()
});

pub fn found_videos(
    path: PathBuf,
    depth: u32,
    handlers: &mut Vec<JoinHandle<anyhow::Result<()>>>,
) -> anyhow::Result<()> {
    if path.is_file()
        && path
            .extension()
            .and_then(|x| x.to_str())
            .map(|x| crate::constants::VIDEO_FORMATS.contains(x))
            == Some(true)
    {
        let handler = tokio::spawn(download_video(path));
        handlers.push(handler);
        return Ok(());
    }

    if depth == 0 {
        return Ok(());
    }

    for i in path.read_dir()? {
        found_videos(i?.path(), depth - 1, handlers)?;
    }

    Ok(())
}

async fn download_video(path: PathBuf) -> anyhow::Result<()> {
    #[derive(Debug, Deserialize, Clone)]
    struct SubInfo {
        surl: String,
        language: String,
        rate: String,
        svote: i64,
    }

    assert!(path.is_file());

    let permit = CONCURRENT_DOWNLOAD_LIMIT.acquire().await;

    debug!("正在搜索 {}", path.display());
    let cid_hash = calc_cid_hash(&path).await?;
    let url = format!(
        "http://sub.xmp.sandai.net:8000/subxl/{cid_hash}.json",
        cid_hash = cid_hash
    );

    debug!("获取详情 {}", url);
    let content = CLIENT.get(&url).send().await?.bytes().await?;
    let text = String::from_utf8_lossy(&content);
    let sub_info = match serde_json::from_str::<serde_json::Value>(&text)?
        .get("sublist")
        .and_then(|x| x.as_array())
        .and_then(|x| x.iter().next())
        .and_then(|x| serde_json::from_value::<SubInfo>(x.clone()).ok())
    {
        None => {
            warn!(
                "not found subtitle for {}",
                path.file_name().unwrap().to_string_lossy()
            );
            return Ok(());
        }
        Some(x) => x,
    };

    let surl = url::Url::parse(&sub_info.surl)?;
    let extension = PathBuf::from(surl.path())
        .extension()
        .and_then(|x| x.to_str())
        .unwrap_or(".srt")
        .to_owned();
    let target_path = path.with_extension(extension);

    debug!("正在下载 {}", surl);
    let content = CLIENT.get(surl).send().await?.bytes().await?;

    debug!("文件写入 {}", target_path.display());
    tokio::fs::write(target_path, content).await?;

    drop(permit);
    info!(
        "下载完成: {}",
        path.file_name()
            .and_then(|x| x.to_str())
            .unwrap_or("<UNKNOWN>")
    );

    Ok(())
}

async fn calc_cid_hash(path: &Path) -> anyhow::Result<String> {
    let mut file = File::open(path).await?;
    let file_size = file.metadata().await?.len();

    let mut context = ::sha1::Sha1::new();
    if file_size < 0xf000 {
        let mut buffer: Vec<u8> = Vec::with_capacity(0xf000);
        file.seek(SeekFrom::Start(0)).await?;
        file.read_to_end(&mut buffer).await?;
        context.update(&buffer);
    } else {
        let mut buffer = vec![0u8; 0x5000];

        file.seek(SeekFrom::Start(0)).await?;
        file.read_exact(&mut buffer).await?;
        context.update(&buffer);

        file.seek(SeekFrom::Start(file_size / 3)).await?;
        file.read_exact(&mut buffer).await?;
        context.update(&buffer);

        file.seek(SeekFrom::End(-0x5000)).await?;
        file.read_exact(&mut buffer).await?;
        context.update(&buffer);
    }
    Ok(context.digest().to_string())
}
