use crate::error::*;

#[derive(Debug)]
pub enum Type {
  Management(ManagementSubtype),
  Control(ControlSubtype),
  Data,
}

#[derive(Debug)]
pub enum ManagementSubtype {
  AssociationRequest,
  AssociationResponse,
  ReassociationRequest,
  ReassociationResponse,
  ProbeRequest,
  ProbeResponse,
  Beacon = 8,
  ATIM,
  Disassociation,
  Authentication,
  Deauthentication,
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
      //
      8 => ManagementSubtype::Beacon,
      9 => ManagementSubtype::ATIM,
      10 => ManagementSubtype::Disassociation,
      11 => ManagementSubtype::Authentication,
      12 => ManagementSubtype::Deauthentication,
      _ => bail!("invalid Management Subtype"),
    })
  }
}

#[derive(Debug)]
pub enum ControlSubtype {
  PSPoll = 10,
  RTS, // Request To Send
  CTS, // Clear To Send
  ACK,
  CFEnd, // Contention Free
  CFEndCFACK,
}

impl ControlSubtype {
  pub fn parse(n: u8) -> Result<ControlSubtype> {
    Ok(match n {
      // maybe use an array lookup? or transmute?
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
