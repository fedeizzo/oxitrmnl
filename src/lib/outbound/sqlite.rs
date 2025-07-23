use log::error;
use sqlx::{query_as, SqlitePool};

use crate::domain::device::{
    models::device::{CreateDeviceRequest, Device, DeviceError, GetDeviceRequest, MacAddress},
    ports::DeviceRepository,
};

pub struct SQLite {
    connection: SqlitePool,
}

impl SQLite {
    pub fn new(connection: SqlitePool) -> SQLite {
        SQLite { connection }
    }
}

struct SQLiteDevice {
    api_key: Option<String>,
    mac_address: String,
    friendly_id: Option<String>,
}

impl TryInto<Device> for SQLiteDevice {
    type Error = DeviceError;

    fn try_into(self) -> Result<Device, Self::Error> {
        match MacAddress::new(&self.mac_address) {
            Ok(mac_address) => Ok(Device {
                api_key: self.api_key.unwrap_or("".to_string()),
                mac_address,
                friendly_name: self.friendly_id.unwrap_or("".to_string()),
            }),
            Err(err) => {
                error!("cannot parse the mac address {}", err);
                Err(DeviceError::Unkwnown)
            }
        }
    }
}

impl DeviceRepository for SQLite {
    async fn create_device(&self, request: &CreateDeviceRequest) -> Result<Device, DeviceError> {
        let api_key = if request.api_key.is_empty() {
            Device::generate_api_key()
        } else {
            request.api_key.clone()
        };
        let mac_address = request.mac_address.inner_value();

        match query_as!(
            SQLiteDevice,
            "INSERT INTO devices(api_key, mac_address) VALUES (?, ?) RETURNING api_key, mac_address, '' as friendly_id",
            api_key,
            mac_address,
        )
        .fetch_one(&self.connection)
        .await
        {
            Ok(device) => device.try_into(),
            Err(err) => {
                error!("cannot retrieve the device {}", err);
                return Err(DeviceError::Unkwnown);
            }
        }
    }

    async fn get_device(&self, request: &GetDeviceRequest) -> Result<Device, DeviceError> {
        let mac_address = request.mac_address.inner_value();
        match query_as!(
            SQLiteDevice,
            "SELECT api_key, mac_address, friendly_id FROM devices WHERE mac_address = ?",
            mac_address,
        )
        .fetch_one(&self.connection)
        .await
        {
            Ok(device) => device.try_into(),
            Err(err) => {
                error!("cannot retrieve the device {}", err);
                return Err(DeviceError::Unkwnown);
            }
        }
    }
}
