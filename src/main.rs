use std::error::Error;
use btleplug::api::{Central, Manager as _ };
use btleplug::{platform::Manager};

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

  
  Ok(())
}
