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

impl Clone for Log {
    fn clone(&self) -> Self {
        let level = match self.level {
            LevelFilter::Off => LevelFilter::Off,
            LevelFilter::Error => LevelFilter::Error,
            LevelFilter::Warn => LevelFilter::Warn,
            LevelFilter::Info => LevelFilter::Info,
            LevelFilter::Debug => LevelFilter::Debug,
            LevelFilter::Trace => LevelFilter::Trace
        };
        let target = match self.target {
            Target::Stdout => Target::Stdout,
            Target::Stderr => Target::Stderr,
            Target::Pipe(_) => panic!("Target::Pipe is not implemented for Clone trait."),
            _ => panic!("Invalid Target.")
        };
        Log {
            level,
            target
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
