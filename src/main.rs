use std::error::Error;
use std::time::Duration;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter };
use btleplug::platform::Adapter;
use btleplug::{platform::Manager, platform::Peripheral};
use tokio::time;

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

    // dump the HR characteristics
    let characteristics = hrm.characteristics();
    println!("characteristics: {:?}", characteristics);
  } else {
    println!("Didn't find device");
  }

  

  Ok(())
}
