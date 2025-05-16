use super::models::device::{CreateDeviceRequest, Device, DeviceError, GetDeviceRequest};

pub trait DeviceRepository {
    fn create_device(&self, request: &CreateDeviceRequest) -> Result<Device, DeviceError>;
    fn get_device(&self, request: &GetDeviceRequest) -> Result<Device, DeviceError>;
}

pub trait DeviceService {
    fn create_device(&self, request: &CreateDeviceRequest) -> Result<Device, DeviceError>;
    fn get_device(&self, request: &GetDeviceRequest) -> Result<Device, DeviceError>;
}
