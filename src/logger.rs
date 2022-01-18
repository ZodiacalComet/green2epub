use std::{fmt::Arguments, io::Write, sync::Mutex, time::SystemTime};

use console::{style, Term};
use log::{Level, Log, Metadata, Record, SetLoggerError};

fn indent_args(args: &Arguments<'_>) -> String {
    args.to_string()
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if index == 0 {
                line.into()
            } else {
                format!("  {}", line)
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

struct Logger {
    level: Level,
    term: Mutex<Term>,
}

impl Logger {
    fn new(level: Level) -> Self {
        Self {
            level,
            term: Mutex::new(Term::buffered_stderr()),
        }
    }

    fn write<S>(&self, content: S)
    where
        S: ToString,
    {
        let mut term = self.term.lock().unwrap();
        term.write_all(content.to_string().as_bytes())
            .expect("failed to write to term's internal buffer");
    }

    fn send_buffered_content(&self) {
        let mut term = self.term.lock().unwrap();
        term.write("\n".as_bytes())
            .expect("failed to write a newline to term's internal buffer");
        term.flush()
            .expect("failed to flush term's internal buffers");
    }

    fn is_verbose(&self) -> bool {
        Level::Debug <= self.level
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let level = record.level();

            if self.is_verbose() {
                let prefix = format!(
                    "[{}] [{:^7}] ",
                    humantime::format_rfc3339_seconds(SystemTime::now()),
                    level,
                );

                match level {
                    Level::Trace | Level::Debug => self.write(style(prefix).dim()),
                    _ => self.write(prefix),
                };
            } else {
                match level {
                    Level::Error => self.write(style("ERROR: ").red().bold()),
                    Level::Warn => self.write(style("WARN: ").yellow().bold()),
                    _ => {}
                }
            }

            let args = indent_args(record.args());
            match level {
                Level::Error => self.write(style(args).red()),
                Level::Warn => self.write(style(args).yellow()),
                Level::Trace | Level::Debug => self.write(style(args).dim()),
                _ => self.write(args),
            };

            self.send_buffered_content();
        }
    }

    fn flush(&self) {}
}

pub fn init(verbose: usize) -> Result<(), SetLoggerError> {
    let level = match verbose {
        0 => Level::Info,
        1 => Level::Debug,
        _ => Level::Trace,
    };

    log::set_boxed_logger(Box::new(Logger::new(level)))
        .map(|()| log::set_max_level(level.to_level_filter()))
}
