use bitflags::{bitflags};

bitflags! {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  struct HrmHeaderFlags : u8 {
  
    // if this bit is set, then the value is 16Bit format, otherwise in 8Bit
    const HeartRateValue16BitFormat = 0x01;
  
    // set if the sensor is in contact with the user's body
    const SensorContactStatus = 0x02;
  
    // set if energy expenditure data is available
    const EnergyExpenditurePresent = 0x04;
  
    // set if RR interval data is present
    const RRIntervalPresent = 0x08;
  }
}

pub struct HrmNotification {
  pub heart_rate: u16,
  pub sensor_in_contact: bool,
  pub energy_expended: Option<u32>,
  pub rr_interval: Option<u32>,
}

impl HrmNotification {
  pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {

    if let Some((header_bytes, data_bytes)) = bytes.split_first_chunk::<1>() {

      let header = HrmHeaderFlags::from_bits_retain(header_bytes[0]);
      let sensor_in_contact = header.contains(HrmHeaderFlags::SensorContactStatus);

      let heart_rate: Option<u16> =
        if header.contains(HrmHeaderFlags::HeartRateValue16BitFormat) {
          // 16 Bit
          if let Some((heart_rate_bytes, _)) = data_bytes.split_first_chunk::<2>() {
            Some(u16::from_le_bytes(*heart_rate_bytes))
          } else {
            None
          }
        } else {
          // 8 Bit
          if let Some((heart_rate_byte, _)) = data_bytes.split_first_chunk::<1>() {
            Some(u16::from(u8::from_le(heart_rate_byte[0])))
          } else {
            None
          }
        };

      if let Some(heart_rate) = heart_rate {

        let rr_interval = None;
        let energy_expended = None;

        Some(HrmNotification {
          heart_rate,
          sensor_in_contact,
          rr_interval,
          energy_expended
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
    use super::HrmNotification;

  #[test]
  pub fn from_bytes_6_49() {
    let bytes : Vec<u8> = Vec::from([6, 49]);
    let result = HrmNotification::from_bytes(bytes);
    assert!(result.is_some());
  }
  
  #[test]
  pub fn  from_bytes_22_50_109_4() {
    let bytes : Vec<u8> = Vec::from([22, 50, 109, 4]);
    let result = HrmNotification::from_bytes(bytes);
    assert!(result.is_some());
  }
}
