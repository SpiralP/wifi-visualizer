use crate::error::*;
use serde_derive::*;

#[derive(Serialize, Debug)]
pub enum Type {
  Management(ManagementSubtype),
  Control(ControlSubtype),
  Data(DataSubtype),
}

#[derive(Serialize, Debug)]
pub enum ManagementSubtype {
  AssociationRequest,
  AssociationResponse,
  ReassociationRequest,
  ReassociationResponse,
  ProbeRequest,
  ProbeResponse,
  // 6-7 Reserved
  Beacon = 8,
  ATIM,
  Disassociation,
  Authentication,
  Deauthentication,
  Action,
  // 14-15 Reserved
}

impl ManagementSubtype {
  pub fn parse(n: u8) -> Result<ManagementSubtype> {
    Ok(match n {
      // maybe use an array lookup? or transmute?
      0 => ManagementSubtype::AssociationRequest,
      1 => ManagementSubtype::AssociationResponse,
      2 => ManagementSubtype::ReassociationRequest,
      3 => ManagementSubtype::ReassociationResponse,
      4 => ManagementSubtype::ProbeRequest,
      5 => ManagementSubtype::ProbeResponse,
      // 6-7 Reserved
      8 => ManagementSubtype::Beacon,
      9 => ManagementSubtype::ATIM,
      10 => ManagementSubtype::Disassociation,
      11 => ManagementSubtype::Authentication,
      12 => ManagementSubtype::Deauthentication,
      13 => ManagementSubtype::Action,
      // 14-15 Reserved
      _ => bail!("invalid Management Subtype"),
    })
  }
}

#[derive(Serialize, Debug)]
pub enum ControlSubtype {
  // 0-7 Reserved
  BlockAckRequest = 8,
  BlockAck,
  PSPoll,
  RTS, // Request To Send
  CTS, // Clear To Send
  ACK,
  CFEnd, // Contention Free
  CFEndCFACK,
}

impl ControlSubtype {
  pub fn parse(n: u8) -> Result<ControlSubtype> {
    Ok(match n {
      // 0-7 Reserved
      8 => ControlSubtype::BlockAckRequest,
      9 => ControlSubtype::BlockAck,
      10 => ControlSubtype::PSPoll,
      11 => ControlSubtype::RTS,
      12 => ControlSubtype::CTS,
      13 => ControlSubtype::ACK,
      14 => ControlSubtype::CFEnd,
      15 => ControlSubtype::CFEndCFACK,
      _ => bail!("invalid Control Subtype"),
    })
  }
}

#[derive(Serialize, Debug)]
pub enum DataSubtype {
  Data,
  DataCFAck,
  DataCFPoll,
  DataCFAckCFPoll,
  Null,
  CFAck,
  CFPoll,
  CFAckCFPoll,
  QoSData,
  QoSDataCFAck,
  QoSDataCFPoll,
  QoSDataCFAckCFPoll,
  QoSNull,
  // 13 Reserved
  QoSCFPoll = 14, // no data
  QoSCFAck,       // no data
}

impl DataSubtype {
  pub fn parse(n: u8) -> Result<DataSubtype> {
    Ok(match n {
      // maybe use an array lookup? or transmute?
      0 => DataSubtype::Data,
      1 => DataSubtype::DataCFAck,
      2 => DataSubtype::DataCFPoll,
      3 => DataSubtype::DataCFAckCFPoll,
      4 => DataSubtype::Null,
      5 => DataSubtype::CFAck,
      6 => DataSubtype::CFPoll,
      7 => DataSubtype::CFAckCFPoll,
      8 => DataSubtype::QoSData,
      9 => DataSubtype::QoSDataCFAck,
      10 => DataSubtype::QoSDataCFPoll,
      11 => DataSubtype::QoSDataCFAckCFPoll,
      12 => DataSubtype::QoSNull,
      // 13 Reserved
      14 => DataSubtype::QoSCFPoll,
      15 => DataSubtype::QoSCFAck,
      _ => bail!("invalid Data Subtype"),
    })
  }
}
