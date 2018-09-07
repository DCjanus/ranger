extern crate clap;
extern crate failure;
extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate threadpool;
extern crate url;

use clap::{App, Arg};
use fetch::{SubInfo, TaskRunner};
use std::{fs::File, io::Write};
use utils::{calc_cid_hash, calc_target_path, calc_video_path};
use validators::{is_exists_file, is_positive_integer};

pub mod fetch;
pub mod utils;
pub mod validators;

pub const DEFAULT_LIMIT: &str = "10";

fn main() -> Result<(), ::failure::Error> {
    let matches = App::new("DC字幕下载器")
        .version("1.0")
        .about("一个简单的字幕下载器")
        .author("DCjanus <DCjanus@dcjanus.com>")
        .arg(
            Arg::with_name("FILE")
                .required(true)
                .validator(is_exists_file)
                .help("电影文件路径"),
        ).arg(
            Arg::with_name("LIMIT")
                .validator(is_positive_integer)
                .short("l")
                .long("limit")
                .default_value(DEFAULT_LIMIT)
                .help("最大下载字幕数"),
        ).get_matches();

    let path = &calc_video_path(matches.value_of("FILE").unwrap())?;
    let limit = matches.value_of("LIMIT").unwrap().parse::<usize>()?;

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

        File::create(target_path)?.write_all(content.as_bytes())?;
        println!(
            "下载成功: {}",
            target_path.file_name().unwrap().to_str().unwrap()
        );
    }

    Ok(())
}
