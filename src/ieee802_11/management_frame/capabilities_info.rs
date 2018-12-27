use super::*;

#[derive(Debug)]
pub struct CapabilitiesInfo {
  // pub ESS_capabilities: bool,             // 1: Transmitter is an AP
// pub IBSS_status: bool,                  // 0: Transmitter belongs to a BSS
// pub CFP_partitipation_capabilities: u8, // 0: No point coordinator at AP
// pub wep_supported: bool,
// pub short_preamble: bool,
// pub PBCC: bool,
// pub channel_agility: bool,     // 0: Not in use
// pub spectrum_management: bool, // 0: Not Implemented
// pub short_slot_time: bool,     // 1: In use
// pub automatic_power_save_delivery: bool,
// pub radio_measurement: bool,
// pub DSSS_OFDM: bool,
// pub delayed_block_ack: bool,
// pub immediate_block_ack: bool,
}

impl CapabilitiesInfo {
  pub fn parse(bytes: &mut Cursor<Vec<u8>>) -> Result<CapabilitiesInfo> {
    bytes.read_u16::<LE>().unwrap();

    Ok(CapabilitiesInfo {})
  }
}
