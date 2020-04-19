use once_cell::sync::Lazy;
use std::{
    num::{NonZeroU32, NonZeroUsize},
    path::PathBuf,
};
use structopt::StructOpt;

pub static OPTIONS: Lazy<Options> = Lazy::new(Options::from_args);

#[derive(Debug, StructOpt)]
#[structopt(
    name = "range",
    about = "一个简单的字幕下载器",
    author = "DCjanus <DCjanus@dcjanus.com>",
    after_help = "源码地址: https://github.com/dcjanus/ranger"
)]
pub struct Options {
    /// 电影文件目录或所在文件夹
    #[structopt(default_value = ".")]
    pub path: PathBuf,
    /// 输出更多的调试信息
    #[structopt(long)]
    pub verbose: bool,
    /// 当path参数为文件夹时递归查找的深度，为1时表示不查找子目录
    #[structopt(long, default_value = "1")]
    pub depth: NonZeroU32,
    /// 并发下载的任务数
    #[structopt(long, default_value = "10")]
    pub concurrent: NonZeroUsize,
}
