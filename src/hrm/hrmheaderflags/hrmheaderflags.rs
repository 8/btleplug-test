use bitflags::bitflags;

bitflags! {
  // see: HRS_SPEC_V10.pdf
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct HrmHeaderFlags : u8 {
  
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