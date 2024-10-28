use chrono::Local;
use env_logger::Env;
use std::io::Write;

pub struct Logger;

impl Logger {
    pub fn init(log_level: &str) {
        env_logger::Builder::from_env(Env::default().default_filter_or(log_level))
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} [{}] - {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .init();
    }
}
