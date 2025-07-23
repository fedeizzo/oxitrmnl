use log::{debug, info};

use super::{
    models::device::{CreateDeviceRequest, Device, DeviceError, GetDeviceRequest},
    ports::{DeviceRepository, DeviceService},
};

pub struct Service<R>
where
    R: DeviceRepository,
{
    repo: R,
}

impl<R> Service<R>
where
    R: DeviceRepository,
{
    pub fn new(repo: R) -> Service<R> {
        Service { repo }
    }
}

impl<R> DeviceService for Service<R>
where
    R: DeviceRepository + std::marker::Sync,
{
    async fn create_device(&self, request: &CreateDeviceRequest) -> Result<Device, DeviceError> {
        let result = self.repo.create_device(request).await?;

        info!("new device created, mac address: {}", request.mac_address);

        Ok(result)
    }

    async fn get_device(&self, request: &GetDeviceRequest) -> Result<Device, DeviceError> {
        let result = self.repo.get_device(request).await?;

        debug!(
            "got device from repository, mac address: {}",
            request.mac_address
        );

        Ok(result)
    }
}
