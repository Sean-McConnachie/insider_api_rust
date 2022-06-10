use std::io::Write;
use log::LevelFilter;
use env_logger::{Builder, Env, Target};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Log builder error")]
    LogErr(String)
}


#[derive(Debug)]
pub struct Log {
    pub level: LevelFilter,
    pub target: Target
}

impl Default for Log {
    fn default() -> Self {
        Log {
            level: LevelFilter::Info,
            target: Target::Stderr,
        }
    }
}


pub fn build_default_logger(log_config: Log) -> Result<Builder, LoggerError> {
    let env = Env::default()
        .filter("auto");

    let mut builder = Builder::from_env(env);

    builder
        .format(|buf, record| {
            writeln!(buf, "[{}] [{}] - {}",
                     buf.timestamp_millis(),
                     record.level(),
                     record.args())
        })
        .filter(None, log_config.level)
        .target(log_config.target);

    Ok(builder)
}
