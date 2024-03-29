use std::error::Error;
use std::time::Duration;
use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter };
use btleplug::{platform::Manager};
use tokio::time;

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

  // get the first adapter
  let adapter = adapters.into_iter().nth(0).unwrap();

  // start scanning
  adapter.start_scan(ScanFilter::default()).await?;

  // wait sometime
  time::sleep(Duration::from_secs(2)).await;

  // get the found devices
  let peripherals = adapter.peripherals().await?;

  println!("peripherals:");
  for peripheral in peripherals.iter() {
    println!("{}", peripheral.id());
    println!("{}", peripheral.address());
    println!("{:?}", peripheral.characteristics());
  }

  Ok(())
}
