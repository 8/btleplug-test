#[cfg(test)]
use super::HrmNotification;

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
