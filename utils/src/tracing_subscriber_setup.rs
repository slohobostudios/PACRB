use std::{env, fs::OpenOptions, io::prelude::*};
use tracing_appender;
use tracing_subscriber::{fmt, layer::SubscriberExt};

pub const TRACING_SUB_FAILURE_MESSAGE: &str =
    "Unable to setup logging subscriber! No logging will be generated.";

pub fn setup_tracing_subscriber_with_no_logging() {
    if try_setup_tracing_subscriber(&["--no-logging".to_string()]).is_err() {
        eprintln!("{}", TRACING_SUB_FAILURE_MESSAGE)
    }
}

pub fn setup_tracing_subscriber() {
    if try_setup_tracing_subscriber(&env::args().collect::<Vec<_>>()).is_err() {
        eprintln!("{}", TRACING_SUB_FAILURE_MESSAGE)
    }
}

pub fn try_setup_tracing_subscriber(
    args: &[String],
) -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    let mut logging_enabled = true;
    for arg in args {
        if arg == "--no-logging" {
            logging_enabled = false;
        }
    }

    if logging_enabled {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .append(true)
            .open("runtime.log")
        {
            let _ = writeln!(
                file,
                "
\n
\n
***********************************************
*                                             *
*          APPLICATION INTILIALIZED           *
*                                             *
***********************************************

            "
            );
        }

        let appender = tracing_appender::rolling::never("./", "runtime.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(appender);
        std::mem::forget(guard); // Gotta keep the guard alive. Rust goes doofus mode and deletes the guard
        let subscriber = tracing_subscriber::registry()
            .with(
                fmt::Layer::new()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_line_number(true),
            )
            .with(
                fmt::Layer::new()
                    .with_writer(std::io::stdout)
                    .with_line_number(true),
            );
        tracing::subscriber::set_global_default(subscriber)
    } else {
        let subscriber = tracing_subscriber::registry().with(
            fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_line_number(true),
        );
        tracing::subscriber::set_global_default(subscriber)
    }
}
