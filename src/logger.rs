use std::{fmt::Arguments, io::Write, sync::Mutex, time::SystemTime};

use console::{set_colors_enabled, style, Term};
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

use crate::args::Color;

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
    level: LevelFilter,
    term: Mutex<Term>,
}

impl Logger {
    fn new(level: LevelFilter) -> Self {
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
        term.write_all("\n".as_bytes())
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

pub fn init(verbose: usize, quiet: bool, color: Color) -> Result<(), SetLoggerError> {
    let level_filter = match (verbose, quiet) {
        (_, true) => LevelFilter::Off,
        (0, false) => LevelFilter::Info,
        (1, false) => LevelFilter::Debug,
        (_, false) => LevelFilter::Trace,
    };

    // `console::style()`, which is used everywhere on the application, doesn't initialize with
    // `for_stderr` to true, so it checks for color in stdout with `console::colors_enabled()`.
    // Knowing that, instead of making an alias or setting it on each instance, the following is
    // done. All of the output goes to stderr anyways.
    match color {
        Color::Always => set_colors_enabled(true),
        Color::Never => set_colors_enabled(false),
        _ => {}
    };

    log::set_boxed_logger(Box::new(Logger::new(level_filter)))
        .map(|()| log::set_max_level(level_filter))
}
