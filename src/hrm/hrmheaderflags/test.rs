#[cfg(test)]
use super::HrmHeaderFlags;

#[test]
pub fn from_bits_6() {

  // arrange
  let byte: u8 = 6;

  // act
  let flags = HrmHeaderFlags::from_bits_retain(byte);

  // assert
  assert!(flags.contains(HrmHeaderFlags::SensorContactSupport));
  assert!(flags.contains(HrmHeaderFlags::SensorContactStatus));
}

#[test]
pub fn from_bits_22() {

  // arrange
  let byte: u8 = 22;

  // act
  let flags = HrmHeaderFlags::from_bits_retain(byte);

  // assert
  assert!(flags.contains(HrmHeaderFlags::SensorContactSupport));
  assert!(flags.contains(HrmHeaderFlags::SensorContactStatus));
  assert!(flags.contains(HrmHeaderFlags::RRIntervalPresent));
}
