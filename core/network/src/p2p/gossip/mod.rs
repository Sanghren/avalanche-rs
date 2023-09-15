pub mod gossip;
pub mod handler;

use avalanche_types::ids::Id;

pub trait Gossipable {
    fn get_id(&self) -> Id;
    fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn deserialize(&mut self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}