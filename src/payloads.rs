//! What crosses the network: the server tells players near a hurt mob what the hit did. Registered
//! (version "1", optional) in `register`.
use mod_api::{PayloadCodec, ToClient, VarInt};
use std::sync::OnceLock;

/// Server → client: entity `entity` (its network id) lost `amount` health (a heal is negative)
/// and has `health` of `max` left; all three in tenths, so half hearts and odd hits survive whole
/// numbers.
#[derive(PayloadCodec, Clone, Debug, PartialEq)]
pub struct Hit {
    pub entity: VarInt,
    pub amount: VarInt,
    pub health: VarInt,
    pub max: VarInt,
}

pub static HIT: OnceLock<ToClient<Hit>> = OnceLock::new();
