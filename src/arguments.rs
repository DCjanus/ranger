use std::path::PathBuf;

use path_absolutize::Absolutize;
use structopt::StructOpt;

use crate::utils::MyResult;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "range",
    about = "一个简单的字幕下载器",
    version = "1.3"
)]
pub struct Arguments {
    #[structopt(name = "FILE", parse(try_from_str = "parse_exist_file_path"))]
    pub path: PathBuf,
    #[structopt(short = "l", long = "limit", default_value = "10")]
    pub limit: usize,
}

fn parse_exist_file_path(src: &str) -> MyResult<PathBuf> {
    let path = PathBuf::from(src).absolutize()?;
    if !path.exists() {
        Err(failure::err_msg(format!(
            "文件不存在: {}",
            path.display()
        )))
    } else if !path.is_file() {
        Err(failure::err_msg(format!(
            "目标不是文件: {}",
            path.display()
        )))
    } else {
        Ok(path)
    }
}
