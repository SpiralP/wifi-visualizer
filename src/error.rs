pub use error_chain::*;

error_chain! {
  foreign_links {
    Pcap(::pcap::Error);
  }
}
