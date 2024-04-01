use std::error::Error;
use std::time::Duration;
use btleplug::api::{CharPropFlags, Characteristic, Peripheral as _};
use btleplug::api::{Central, Manager as _, ScanFilter, bleuuid::uuid_from_u16 };
use btleplug::platform::Adapter;
use btleplug::{platform::Manager, platform::Peripheral};
use futures::StreamExt;
use tokio::time;
use uuid::Uuid;

use crate::hrm::HrmNotification;

mod hrm;

const HEART_RATE_SERVICE_CHARACTERISTICS_UUID: Uuid = uuid_from_u16(0x180D);

async fn find_hrm(adapter: &Adapter) -> Option<Peripheral> {
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

async fn dump_peripherals(peripherals: Vec<Peripheral>) -> Result<(), Box<dyn Error>> {
  println!("peripherals:");
  for peripheral in peripherals.iter() {
    // println!("{}", peripheral.id());
    println!("  address: {}", peripheral.address());
    
    let properties = peripheral.properties().await?.unwrap();
    println!("  {}", properties.local_name.unwrap_or("<none>".to_string()));

    println!("  services:");
    for service in properties.services {
      println!("    {}", service);
    }
    
    // // connect
    // peripheral.connect().await?;
    
    // // discover services and characteristics
    // peripheral.discover_services().await?;
    // println!("  characteristics: {:?}", peripheral.characteristics());
    
    println!()
  }
  Ok(())
}

// todo: try to use a filter to find all devices based on the heart rate service characteristic


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

  // create a manager
  let manager = Manager::new().await.unwrap();

  // get all adapters
  let adapters = manager.adapters().await?;

  // print adapter info
  println!("adapters:");
  for adapter in adapters.iter() {
    let adapter_info = adapter.adapter_info().await?;
    println!("{}", adapter_info);
  }
  println!();

  // get the first adapter
  let adapter = adapters.into_iter().nth(0).unwrap();

  // start scanning
  adapter.start_scan(ScanFilter::default()).await?;

  // wait sometime
  time::sleep(Duration::from_secs(3)).await;

  // get the found devices
  // let peripherals = adapter.peripherals().await?;

  // dump the found peripherals

  if let Some(hrm) = find_hrm(&adapter).await {
    
    println!("Found device!");

    let local_name = hrm.properties().await.unwrap().unwrap().local_name.unwrap();

    // connect
    hrm.connect().await?;

    // discovers services and characteristics
    hrm.discover_services().await?;

    println!();
    let characteristics = hrm.characteristics();
    
    // HeartRateCharacteristic: 0000180d-0000-1000-8000-00805f9b34fb
    println!("HeartRateServiceCharacteristic: {:?}", HEART_RATE_SERVICE_CHARACTERISTICS_UUID);
    
    // dump the characteristics of the HR
    for characteristic in characteristics.iter() {
      println!("{}, {}, {:?}", characteristic.uuid, characteristic.service_uuid, characteristic.properties);
    }

    let heart_rate_service_notify_characteristic =
      characteristics.into_iter().find(|c|c.service_uuid == HEART_RATE_SERVICE_CHARACTERISTICS_UUID
        && c.properties & CharPropFlags::NOTIFY == CharPropFlags::NOTIFY);

    if let Some(hrc) = heart_rate_service_notify_characteristic {
      println!("found heart rate characteristic");

      //  subscribe to the heart rate
      hrm.subscribe(&hrc).await?;

      let mut notification_stream = hrm.notifications().await?;

      while let Some(data) = notification_stream.next().await {
        println!(
            "Received data from {:?} [{:?}]: {:?}",
            local_name, data.uuid, data.value
        );

        let hrm_notification = HrmNotification::from_bytes(data.value);
        if let Some(hrm_notification) = hrm_notification {
          println!(
            "Heart Rate: {}, Sensor in Contact: {}", 
            hrm_notification.heart_rate,
            hrm_notification.sensor_in_contact
          );
        }
      }

    } else {
      println!("didn't find heart rate characteristic");
    }

  } else {
    println!("Didn't find device");
  }

  Ok(())
}
