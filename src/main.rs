use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

fn main() -> Result<(), SetLoggerError> {
    let file_path = "logs.log";

    let p = PatternEncoder::new("{d(%H:%M:%S)} - {l} - {m}\n");
    let stderr = ConsoleAppender::builder().encoder(Box::new(p)).build();

    let trigger = SizeTrigger::new(10 * 1024 * 1024); // 10 MB

    let roller = FixedWindowRoller::builder()
        .build("logs.{}.log", 10)
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build(file_path, Box::new(policy))
        .unwrap();

    let level = LevelFilter::Info; // Trace < Debug < Info < Warn < Error
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(level),
        )
        .unwrap();

    if let Err(err) = log4rs::init_config(config) {
        println!("Error: {}", err);
    }

    error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");
    debug!("Goes to file only");
    trace!("Goes to file only");

    Ok(())
}
