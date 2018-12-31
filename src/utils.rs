use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    str::FromStr,
};

use url::Url;

use crate::fetch::SubInfo;

pub type MyResult<T> = ::std::result::Result<T, ::failure::Error>;

pub fn calc_cid_hash(path: &Path) -> MyResult<String> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();

    let mut context = ::sha1::Sha1::new();
    if file_size < 0xf000 {
        let mut buffer: Vec<u8> = Vec::with_capacity(0xf000);
        file.read_to_end(&mut buffer)?;
        context.update(&buffer);
    } else {
        let mut buffer = vec![0u8; 0x5000];

        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut buffer)?;
        context.update(&buffer);

        file.seek(SeekFrom::Start(file_size / 3))?;
        file.read_exact(&mut buffer)?;
        context.update(&buffer);

        file.seek(SeekFrom::End(-0x5000))?;
        file.read_exact(&mut buffer)?;
        context.update(&buffer);
    }
    Ok(context.hexdigest())
}

pub fn calc_target_path(video_path: &Path, index: usize, sub_info: &SubInfo) -> PathBuf {
    let parent_dir = video_path
        .parent()
        .expect("Can't get parent dir for video path");
    let movie_name = video_path
        .file_stem()
        .expect("Failed to get file stem")
        .to_str()
        .expect("OsStr to str failed");
    let sub_download_url = Url::from_str(sub_info.url.as_ref())
        .unwrap_or_else(|_| panic!(format!("Wrong sub download URL:{}", sub_info.url)));
    let sub_extension = Path::new(sub_download_url.path())
        .extension()
        .map(|x| x.to_str().expect("OsStr to str failed for sub extension"))
        .unwrap_or("srt");

    let target_file_name = format!(
        "{movie_name}_{index}_{language}.{sub_extension}",
        movie_name = movie_name,
        index = index,
        sub_extension = sub_extension,
        language = sub_info.language,
    );
    parent_dir.join(Path::new(&target_file_name))
}
