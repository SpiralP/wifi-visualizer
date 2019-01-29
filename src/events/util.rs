use ieee80211::MacAddress;

pub fn hash_macs(mac1: MacAddress, mac2: MacAddress) -> String {
  if mac1 >= mac2 {
    format!("{}{}", mac1, mac2).to_string()
  } else {
    format!("{}{}", mac2, mac1).to_string()
  }
}

pub fn is_broadcast(mac: MacAddress) -> bool {
  // multicast
  (mac.as_bytes()[0] & 0b01) != 0
}
