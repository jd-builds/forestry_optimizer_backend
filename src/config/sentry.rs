use super::environment::Environment;
use ::sentry::ClientInitGuard as SentryGuard;
use log::{info, warn};

pub fn init(dsn: &Option<String>, environment: &Environment) -> Option<SentryGuard> {
    match dsn {
        Some(dsn) => {
            let guard = sentry::init((
                dsn.clone(),
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    environment: Some(environment.to_string().into()),
                    ..Default::default()
                },
            ));
            info!("Sentry initialized successfully");
            Some(guard)
        }
        None => {
            warn!("Sentry DSN not configured - error tracking disabled");
            None
        }
    }
}
