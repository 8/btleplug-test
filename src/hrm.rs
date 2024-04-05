use bitflags::bitflags;

bitflags! {
  // see: HRS_SPEC_V10.pdf
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  struct HrmHeaderFlags : u8 {
  
    // if this bit is set, then the value is 16Bit format, otherwise in 8Bit
    const HeartRateValue16BitFormat = 0b0000_0001;
  
    // set if the sensor is in contact with the user's body
    const SensorContactStatus = 0b0000_0010;

    // set if the server supports the sensor contact status
    const SensorContactSupport = 0b0000_0100;
  
    // set if energy expenditure data is available
    const EnergyExpenditurePresent = 0b0000_1000;
  
    // set if RR interval data is present
    const RRIntervalPresent = 0b0001_0000;
  }
}

pub struct HrmNotification {
  pub heart_rate: u16,
  pub sensor_in_contact: bool,
  
  // in kilo joule
  // pub energy_expended: Option<u32>,

  // pub rr_interval: Option<u32>,
}

impl HrmNotification {
  pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {

    if let Some((header_bytes, data_bytes)) = bytes.split_first_chunk::<1>() {

      let header = HrmHeaderFlags::from_bits_retain(header_bytes[0]);
      let sensor_in_contact = header.contains(HrmHeaderFlags::SensorContactStatus);

      let heart_rate: Option<u16> =
        if header.contains(HrmHeaderFlags::HeartRateValue16BitFormat) { // 16-Bit
          if let Some((heart_rate_bytes, _)) = data_bytes.split_first_chunk::<2>() {
            Some(u16::from_le_bytes(*heart_rate_bytes))
          } else {
            None
          }
        } else { // 8-Bit
          if let Some((heart_rate_byte, _)) = data_bytes.split_first_chunk::<1>() {
            Some(u16::from(u8::from_le(heart_rate_byte[0])))
          } else {
            None
          }
        };

      if let Some(heart_rate) = heart_rate {

        // let rr_interval = None;
        // let energy_expended = None;

        Some(HrmNotification {
          heart_rate,
          sensor_in_contact,
          // rr_interval,
          // energy_expended
        })
      } else {
        None
      }

      } else  {
        None
      }

  }
}

#[cfg(test)] 
mod test {
    use super::{HrmHeaderFlags, HrmNotification};

  #[test]
  pub fn from_bytes_6_49() {

    // arrange
    let bytes : Vec<u8> = Vec::from([6, 49]);

    // act
    let result = HrmNotification::from_bytes(bytes);

    // assert
    assert!(result.is_some());
    if let Some(result) = result {
      assert_eq!(result.heart_rate, 49);
      assert!(result.sensor_in_contact);
    }
  }
  
  #[test]
  pub fn from_bytes_22_50_109_4() {
    // arrange
    let bytes : Vec<u8> = Vec::from([22, 50, 109, 4]);

    // act
    let result = HrmNotification::from_bytes(bytes);

    // assert
    assert!(result.is_some());
    if let Some(result) = result {
      assert!(result.sensor_in_contact);
      assert_eq!(result.heart_rate, 50);
    }
  }

  #[test]
  pub fn from_bytes_22_55_61_4_73_1_168_1() {

    // arrange
    let bytes: Vec<u8> = Vec::from([22, 55, 61, 4, 73, 1, 168, 1]);

    // act
    let result = HrmNotification::from_bytes(bytes);

    // assert
    assert!(result.is_some());
  }

  #[test]
  pub fn HrmHeaderFlags_from_bits() {
    
    // arrange
    let byte : u8 = 22;

    // act
    let flags = HrmHeaderFlags::from_bits_retain(byte);

    println!("{:?}", flags);
  }
}
