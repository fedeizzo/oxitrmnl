use std::error::Error;

use env_logger::{init_from_env, Env};
use log::info;
use oxitrmnl::{
    config,
    domain::device::{
        models::device::{CreateDeviceRequest, GetDeviceRequest, MacAddress},
        ports::DeviceService,
        service::Service,
    },
    outbound,
};
use rusqlite::Connection;

refinery::embed_migrations!("migrations");

#[tokio::main]
async fn main() {
    let config = config::Config::from_env().unwrap();

    init_logger(config.log_level_var);

    run_migrations(&config.database_url).unwrap();

    let conn = Connection::open(config.database_url).unwrap();
    conn.pragma_update(None, "foreign_keys", "ON").unwrap();

    let outbound = outbound::sqlite::SQLite::new(conn);

    let device_service = Service::new(outbound);
    let mac_address = MacAddress::new("prova").unwrap();
    let created_device = device_service.create_device(&CreateDeviceRequest::new(
        "prova".to_string(),
        mac_address.clone(),
    ));
    println!("{:?}", created_device);

    let retrieved_device = device_service.get_device(&GetDeviceRequest::new(mac_address.clone()));
    println!("{:?}", retrieved_device);
}

fn init_logger(log_level_var: String) {
    let env = Env::default().filter_or(log_level_var, "INFO");
    init_from_env(env);
}

fn run_migrations(database_url: &String) -> Result<(), Box<dyn Error + 'static>> {
    let mut conn = Connection::open(database_url)?;
    let report = migrations::runner().run(&mut conn)?;

    for migration in report.applied_migrations() {
        info!(
            "migration {} applied, version {}",
            migration.name(),
            migration.version()
        )
    }

    conn.close().unwrap();

    Ok(())
}
