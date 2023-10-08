use std::fs::File;
use std::io::{Error, Write};
use std::sync::Mutex;

use log::{LevelFilter, Log, Metadata, Record, set_boxed_logger, set_max_level, SetLoggerError};
use termcolor::{BufferedStandardStream, ColorChoice, ColorSpec, WriteColor};

pub use crate::config::*;

mod config;

pub struct TinyLogger {
    level: LevelFilter,
    config: Config,
    term: Option<Mutex<BufferedStandardStream>>,
    file: Option<Mutex<File>>,
}

impl TinyLogger {
    pub fn init(level: LevelFilter, config: Config, term: Option<ColorChoice>, path: Option<&str>) -> Result<(), SetLoggerError> {
        let this = Self {
            level,
            config,
            term: term.map(BufferedStandardStream::stdout).map(Mutex::new),
            file: path.map(|path| File::create(path).unwrap()).map(Mutex::new),
        };
        set_max_level(this.level);
        set_boxed_logger(Box::new(this))
    }

    fn try_log_term(&self, record: &Record, stream: &mut BufferedStandardStream) -> Result<(), Error> {
        let color = self.config.level_color[record.level() as usize];

        if self.config.level <= record.level() && self.config.level != LevelFilter::Off {
            if !self.config.write_log_enable_colors {
                stream.set_color(ColorSpec::new().set_fg(color))?;
            }

            write_level(record, stream, &self.config)?;

            if !self.config.write_log_enable_colors {
                stream.reset()?;
            }
        }

        if self.config.location <= record.level() && self.config.location != LevelFilter::Off {
            write_location(record, stream)?;
        }

        write_args(record, stream)?;

        stream.flush()
    }

    fn try_log_file(&self, record: &Record, write: &mut File) -> Result<(), Error> {
        if self.config.level <= record.level() && self.config.level != LevelFilter::Off {
            write_level(record, write, &self.config)?;
        }

        if self.config.location <= record.level() && self.config.location != LevelFilter::Off {
            write_location(record, write)?;
        }

        write_args(record, write)
    }
}

impl Log for TinyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(term) = &self.term {
                let mut stream = term.lock().unwrap();
                let _ = self.try_log_term(record, &mut stream);
            }
            if let Some(file) = &self.file {
                let mut writer = file.lock().unwrap();
                let _ = self.try_log_file(record, &mut *writer);
            }
        }
    }

    fn flush(&self) {
        if let Some(term) = &self.term {
            let _ = term.lock().unwrap().flush();
        }
        if let Some(file) = &self.file {
            let _ = file.lock().unwrap().flush();
        }
    }
}


#[inline(always)]
fn write_level<W: Write>(record: &Record, write: &mut W, config: &Config) -> Result<(), Error> {
    let level = match config.level_padding {
        LevelPadding::Left => format!("[{: >5}]", record.level()),
        LevelPadding::Right => format!("[{: <5}]", record.level()),
        LevelPadding::Off => format!("[{}]", record.level()),
    };

    write!(write, "{} ", level)?;

    Ok(())
}

#[inline(always)]
fn write_location<W: Write>(record: &Record, write: &mut W) -> Result<(), Error> {
    let file = record.file().unwrap_or("<unknown>");
    if let Some(line) = record.line() {
        write!(write, "[{}:{}] ", file, line)?;
    } else {
        write!(write, "[{}:<unknown>] ", file)?;
    }
    Ok(())
}


#[inline(always)]
fn write_args<W: Write>(record: &Record, write: &mut W) -> Result<(), Error> {
    writeln!(write, "{}", record.args())?;
    Ok(())
}
