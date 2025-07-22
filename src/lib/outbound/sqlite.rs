use log::error;
use rusqlite::{types::FromSqlError, Connection};

use crate::domain::device::{
    models::device::{CreateDeviceRequest, Device, DeviceError, GetDeviceRequest, MacAddress},
    ports::DeviceRepository,
};

pub struct SQLite {
    connection: Connection,
}

impl SQLite {
    pub fn new(connection: Connection) -> SQLite {
        SQLite { connection }
    }
}

impl DeviceRepository for SQLite {
    fn create_device(&self, request: &CreateDeviceRequest) -> Result<Device, DeviceError> {
        if let Err(err) = self.connection.execute(
            "INSERT INTO devices(api_key, mac_address) VALUES (?1, ?2)",
            (&request.api_key, &request.mac_address.inner_value()),
        ) {
            error!("cannot create a device: {}", err);
            return Err(DeviceError::Unkwnown);
        }

        Ok(Device::new(
            request.api_key.clone(),
            request.mac_address.clone(),
        ))
    }

    fn get_device(&self, request: &GetDeviceRequest) -> Result<Device, DeviceError> {
        match self.connection.query_row(
            "SELECT api_key, mac_address, friendly_id FROM devices WHERE mac_address = $1",
            [request.mac_address.inner_value()],
            |row| {
                let mac_address: String = row.get(1)?;
                let parsed_mac_address = match MacAddress::new(&mac_address) {
                    Ok(it) => Ok(it),
                    Err(err) => {
                        error!("cannot parse the mac address {}", err);
                        Err(FromSqlError::InvalidType)
                    }
                };
                let friendly_name = match row.get(2) {
                    Ok(val) => val,
                    Err(_) => "".to_string(),
                };
                Ok(Device::get(row.get(0)?, parsed_mac_address?, friendly_name))
            },
        ) {
            Ok(device) => return Ok(device),
            Err(err) => {
                error!("cannot retrieve the device {}", err);
                return Err(DeviceError::Unkwnown);
            }
        }
    }
}
