use log::error;
use once_cell::sync::Lazy;

pub mod constants;
pub mod fetch;
pub mod options;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Lazy::force(&crate::options::OPTIONS);
    init_logger()?;

    let mut handlers = vec![];
    let path = crate::options::OPTIONS.path.clone();
    let depth = crate::options::OPTIONS.depth;
    crate::fetch::found_videos(path, depth.into(), &mut handlers)?;

    for i in handlers {
        if let Err(error) = i.await? {
            error!("发生错误: {}", error);
        }
    }

    Ok(())
}

fn init_logger() -> anyhow::Result<()> {
    if std::env::var("RUST_BACKTRACE").is_ok() {
        // work with anyhow::Error
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    let level = if crate::options::OPTIONS.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            let color_config = fern::colors::ColoredLevelConfig::new()
                .info(fern::colors::Color::Green)
                .debug(fern::colors::Color::Magenta);
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%F %H:%M:%S %:z"),
                color_config.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for(module_path!().splitn(2, "::").next().unwrap(), level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
