use crate::{
    dto::{CreateMeterDeviceInput, UpdateMeterDeviceInput},
    error::{Error, Result},
    model::{
        db::{CreateMeterDeviceData, DbDevice, DbMeterDevice, UpdateMeterDeviceData},
        Identifier,
    },
    PgPool,
};

pub struct MeterDeviceService;

impl MeterDeviceService {
    pub(crate) async fn get_by_identifiers(
        meter_identifier: Identifier,
        meterdevice_identifier: Identifier,
        pool: &PgPool,
    ) -> Result<DbMeterDevice> {
        let (meter_id, meterdevice_id) = match (meter_identifier, meterdevice_identifier) {
            (Identifier::Id(meter_id), Identifier::Id(device_id)) => (meter_id, device_id),
            _ => return Err(Error::InvalidIdentifier),
        };

        DbMeterDevice::get_by_meter_device_id(meter_id, meterdevice_id, pool).await
    }

    pub(crate) async fn create(
        meter_identifier: Identifier,
        input: CreateMeterDeviceInput,
        pool: &PgPool,
    ) -> Result<DbMeterDevice> {
        let meter_id = match meter_identifier {
            Identifier::Id(id) => id,
            Identifier::Name(_) => return Err(Error::InvalidIdentifier),
        };

        let device = DbDevice::get_by_identifer(input.device, pool).await?;

        let data = CreateMeterDeviceData {
            meter: meter_id,
            device: device.id,
            turned_on: input.on,
            duration: device.duration,
        };

        let id = DbMeterDevice::create(data, pool).await?;

        DbMeterDevice::get_by_id(id, pool).await
    }

    pub(crate) async fn update(
        meter_identifier: Identifier,
        meterdevice_identifier: Identifier,
        input: UpdateMeterDeviceInput,
        pool: &PgPool,
    ) -> Result<DbMeterDevice> {
        let (meter_id, meterdevice_id) = match (meter_identifier, meterdevice_identifier) {
            (Identifier::Id(meter_id), Identifier::Id(device_id)) => (meter_id, device_id),
            _ => return Err(Error::InvalidIdentifier),
        };

        //let meterdevice_id = DbMeterDevice::get_by_meter_device_id(meter_id, device_id, pool)
        //    .await?
        //    .id;

        let data = UpdateMeterDeviceData {
            meter: None,
            device: None,
            turned_on: input.on,
            duration: input.duration,
        };

        let id = DbMeterDevice::update(Identifier::Id(meterdevice_id), data, pool).await?;

        DbMeterDevice::get_by_id(id, pool).await
    }
}
