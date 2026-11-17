use chrono::{DateTime, Local};
use iocore::Path;
use log::{Level, debug, error, info, log_enabled, trace, warn};

pub fn setup_logger(
    log_level: log::LevelFilter,
    output_file: Option<Path>,
) -> Result<(), fern::InitError> {
    let mut dsp = fern::Dispatch::new()
        .format(|out, message, record| {
            let now = Local::now();
            let timestamp = now.format("%Y/%m/%dT%H:%M:%S.%f");
            let level = record.level();
            let target = record.target();
            out.finish(format_args!("[{level}] [{timestamp}] {target}: {message}"))
        })
        .level(log_level)
        .chain(std::io::stderr());
    if let Some(output_file) = output_file {
        dsp = dsp.chain(match fern::log_file(&output_file){
            Ok(output) => output,
            Err(error) => {
                log::error!("failed to setup log output to {output_file}: {error}");
                std::process::exit(1);
            }
        })
    }
    dsp.apply()?;
    Ok(())
}
