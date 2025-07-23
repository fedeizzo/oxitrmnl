use std::fmt::Display;

use rand::{distr::Alphanumeric, Rng};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Device {
    pub api_key: String,
    pub mac_address: MacAddress,
    pub friendly_name: String,
}

impl Device {
    pub fn new(api_key: String, mac_address: MacAddress) -> Device {
        let mut api_key = api_key;
        if api_key.is_empty() {
            api_key = Device::generate_api_key();
        }

        return Device {
            api_key,
            mac_address,
            friendly_name: "".to_string(),
        };
    }

    pub fn get(api_key: String, mac_address: MacAddress, friendly_name: String) -> Device {
        return Device {
            api_key,
            mac_address,
            friendly_name,
        };
    }

    pub fn update_friendly_name(&mut self, friendly_name: String) {
        self.friendly_name = friendly_name
    }

    pub fn generate_api_key() -> String {
        let mut rng = rand::rng();
        (0..20).map(|_| rng.sample(Alphanumeric) as char).collect()
    }
}

pub struct CreateDeviceRequest {
    pub api_key: String,
    pub mac_address: MacAddress,
}

impl CreateDeviceRequest {
    pub fn new(api_key: String, mac_address: MacAddress) -> CreateDeviceRequest {
        CreateDeviceRequest {
            api_key,
            mac_address,
        }
    }
}

pub struct GetDeviceRequest {
    pub mac_address: MacAddress,
}

impl GetDeviceRequest {
    pub fn new(mac_address: MacAddress) -> GetDeviceRequest {
        GetDeviceRequest { mac_address }
    }
}

#[derive(Debug)]
pub enum DeviceError {
    Duplicated(String),
    NotExisting(String),
    Unkwnown,
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceError::Duplicated(id) => write!(f, "duplicated device {}", id),
            DeviceError::NotExisting(id) => write!(f, "device does not exist {}", id),
            DeviceError::Unkwnown => write!(f, "unknown error"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MacAddress(String);

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl MacAddress {
    pub fn inner_value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug)]
pub enum MacAddressError {
    Duplicate,
    Malformed,
}

impl Display for MacAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacAddressError::Duplicate => write!(f, "duplicate mac address"),
            MacAddressError::Malformed => write!(
                f,
                "malformed mac address, allowed format: XX-XX-XX-XX-XX-XX"
            ),
        }
    }
}

impl MacAddress {
    pub fn new(addr: &str) -> Result<MacAddress, MacAddressError> {
        // TODO add regex to check the format
        Ok(MacAddress(addr.to_string()))
    }
}
