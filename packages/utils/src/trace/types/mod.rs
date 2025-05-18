pub mod startup;
pub mod network;
pub mod peer;

pub use startup::*;
pub use network::*;
pub use peer::*;

pub trait TraceId: std::fmt::Debug {
    fn id(&self) -> u64;
    fn name(&self) -> &'static str;
    fn message(&self) -> String;
}
