use std::{fs::File, io::Write};

use structopt::StructOpt;

use crate::{
    arguments::Arguments,
    fetch::{SubInfo, TaskRunner},
    utils::{calc_cid_hash, calc_target_path},
};

pub mod arguments;
pub mod fetch;
pub mod utils;

fn main() -> Result<(), ::failure::Error> {
    let args: Arguments = Arguments::from_args();

    let path = &args.path;
    let limit = args.limit;

    let cid_hash = calc_cid_hash(path)?;
    let sub_info_list = SubInfo::all(&cid_hash, limit)?;

    let mut downloader = TaskRunner::default();
    sub_info_list
        .into_iter()
        .enumerate()
        .map(|(index, sub_info)| (calc_target_path(path, index, &sub_info), sub_info))
        .for_each(|(target_path, sub_info)| downloader.execute(sub_info, target_path)); // UGLY CODE

    for i in downloader.results {
        let download_result = i.recv().expect("Receiver download result failed");
        let target_path = &download_result.target_path;
        let content = download_result.response?;

        File::create(target_path)?.write_all(&content)?;
        println!(
            "下载成功: {}",
            target_path.file_name().unwrap().to_str().unwrap()
        );
    }

    Ok(())
}
