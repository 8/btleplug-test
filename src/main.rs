use std::error::Error;
use std::time::Duration;
use btleplug::api::{CharPropFlags, Characteristic};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, bleuuid::uuid_from_u16 };
use btleplug::platform::Adapter;
use btleplug::{platform::Manager, platform::Peripheral};
use tokio::time;
use uuid::Uuid;

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

    // connect
    hrm.connect().await?;

    // discovers services and characteristics
    hrm.discover_services().await?;

    println!();
    let characteristics = hrm.characteristics();
    
    // HeartRateCharacteristic: 0000180d-0000-1000-8000-00805f9b34fb
    println!("HeartRateCharacteristic: {:?}", HEART_RATE_SERVICE_CHARACTERISTICS_UUID);
    
    // battery service            // 00002a19-0000-1000-8000-00805f9b34fb, 0000180f-0000-1000-8000-00805f9b34fb, CharPropFlags(READ | NOTIFY)
    // device information service // 00002a24-0000-1000-8000-00805f9b34fb, 0000180a-0000-1000-8000-00805f9b34fb, CharPropFlags(READ)
    // device information service // 00002a26-0000-1000-8000-00805f9b34fb, 0000180a-0000-1000-8000-00805f9b34fb, CharPropFlags(READ)
    // device information service // 00002a27-0000-1000-8000-00805f9b34fb, 0000180a-0000-1000-8000-00805f9b34fb, CharPropFlags(READ)
    // device information service // 00002a28-0000-1000-8000-00805f9b34fb, 0000180a-0000-1000-8000-00805f9b34fb, CharPropFlags(READ)
    // device information service // 00002a29-0000-1000-8000-00805f9b34fb, 0000180a-0000-1000-8000-00805f9b34fb, CharPropFlags(READ)
    // heart rate service         // 00002a37-0000-1000-8000-00805f9b34fb, 0000180d-0000-1000-8000-00805f9b34fb, CharPropFlags(NOTIFY)
    // heart rate service         // 00002a38-0000-1000-8000-00805f9b34fb, 0000180d-0000-1000-8000-00805f9b34fb, CharPropFlags(READ)
    // // 8fc3fd09-f21d-11e3-976c-0002a5d5c51b, 8fc3fd00-f21d-11e3-976c-0002a5d5c51b, CharPropFlags(NOTIFY)
    // // 8fc3fd0a-f21d-11e3-976c-0002a5d5c51b, 8fc3fd00-f21d-11e3-976c-0002a5d5c51b, CharPropFlags(WRITE_WITHOUT_RESPONSE)
    // // 8fc3fd15-f21d-11e3-976c-0002a5d5c51b, 8fc3fd00-f21d-11e3-976c-0002a5d5c51b, CharPropFlags(NOTIFY)
    // // 8fc3fd16-f21d-11e3-976c-0002a5d5c51b, 8fc3fd00-f21d-11e3-976c-0002a5d5c51b, CharPropFlags(WRITE_WITHOUT_RESPONSE)

    // dump the characteristics of the HR
    for characteristic in characteristics.iter() {
      println!("{}, {}, {:?}", characteristic.uuid, characteristic.service_uuid, characteristic.properties);
    }

    let heart_rate_service_characteristic = characteristics
      .into_iter().find(|c|
        c.service_uuid == HEART_RATE_SERVICE_CHARACTERISTICS_UUID
        && c.properties & CharPropFlags::READ == CharPropFlags::READ);

    if let Some(hrc) = heart_rate_service_characteristic {
      println!("found heart rate characteristic");

      for _ in 0..20 {
        if let Ok(result) = hrm.read(&hrc).await {
          println!("read heart rate: {:?}", result);
          for b in result {
            print!("{}", b);
            println!();
          }

        }
        time::sleep(Duration::from_millis(1000)).await;
      }
    } else {
      println!("didn't find heart rate characteristic");
    }

  } else {
    println!("Didn't find device");
  }

  Ok(())
}
