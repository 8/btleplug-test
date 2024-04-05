mod hrmheaderflags;
mod hrmnotification;

use uuid::Uuid;
use btleplug::{api::{bleuuid::uuid_from_u16, Central as _, Peripheral as _}, platform::{Adapter, Peripheral}};
pub use hrmnotification::HrmNotification;

pub const HEART_RATE_SERVICE_CHARACTERISTICS_UUID: Uuid = uuid_from_u16(0x180D);

pub async fn find_hrm(adapter: &Adapter) -> Option<Peripheral> {
  if let Ok(peripherials) = adapter.peripherals().await {
    for peripheral in peripherials {
      if let Ok(properties) = peripheral.properties().await {
        if let Some(properties) = properties {
          if let Some(local_name) = properties.local_name {
            if local_name.contains("HR-70EC8EA6") {
              return Some(peripheral);
            }
          }
        }
      }
    }
  }
  None
}

