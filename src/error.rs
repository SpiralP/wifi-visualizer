#![allow(deprecated)]

pub use error_chain::{
  bail, error_chain, error_chain_processing, impl_error_chain_kind, impl_error_chain_processed,
  impl_extract_backtrace, ChainedError,
};

error_chain! {
  foreign_links {
    Pcap(::pcap::Error);
  }
}
