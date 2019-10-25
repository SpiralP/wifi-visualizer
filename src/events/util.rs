use ieee80211::MacAddress;

pub fn hash_macs(mac1: MacAddress, mac2: MacAddress) -> String {
  if mac1 >= mac2 {
    format!("{}{}", mac1, mac2)
  } else {
    format!("{}{}", mac2, mac1)
  }
}

pub fn is_broadcast(mac: MacAddress) -> bool {
  // multicast
  (mac.as_bytes()[0] & 0b01) != 0
}
