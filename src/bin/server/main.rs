use std::{ops::Deref, str::FromStr, sync::Arc};

use env_logger::{init_from_env, Env};
use oxitrmnl::{
    config::{self, Config, DBType},
    domain::device::service::Service,
    inbound::http,
    outbound,
};
use sqlx::migrate;
use sqlx::{migrate::Migrate, Acquire};
use sqlx::{
    migrate::MigrateError,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use tokio::{net::TcpListener, signal};

#[tokio::main]
async fn main() {
    let config = config::Config::from_env().unwrap();

    init_logger(&config.log_level_var);

    let db_options = SqliteConnectOptions::from_str(&config.database_url)
        .unwrap()
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .test_before_acquire(true)
        .connect_with(db_options)
        .await
        .unwrap();
    run_migrations(&config, &pool).await.unwrap();

    let outbound = outbound::sqlite::SQLite::new(pool);

    let device_service = Service::new(outbound);
    let inbound = http::HttpServer::new(device_service);
    let app = oxitrmnl_openapi::server::new(Arc::new(inbound));

    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    // let mac_address = MacAddress::new("prova").unwrap();
    // let created_device = device_service.create_device(&CreateDeviceRequest::new(
    //     "prova".to_string(),
    //     mac_address.clone(),
    // ));
    // println!("{:?}", created_device);

    // let retrieved_device = device_service.get_device(&GetDeviceRequest::new(mac_address.clone()));
    // println!("{:?}", retrieved_device);
}

fn init_logger(log_level_var: &String) {
    let env = Env::default().filter_or(log_level_var, "INFO");
    init_from_env(env);
}

async fn run_migrations<'a, A>(config: &Config, pool: A) -> Result<(), MigrateError>
where
    A: Acquire<'a>,
    <A::Connection as Deref>::Target: Migrate,
{
    match config.get_db_type() {
        DBType::SQLite => {
            return Ok(migrate!("migrations/sqlite").run(pool).await?);
        }
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
