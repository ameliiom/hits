//! What crosses the network: the server tells players near a hurt mob what the hit did, and a
//! player looking at a merchant its trades. Registered (version "1", optional) in `register`.
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

/// An item shown in a trade: its id (`minecraft:emerald`) and count.
#[derive(PayloadCodec, Clone, Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub count: VarInt,
}

/// One trade: what it costs (one or two items), what it gives, and whether it is out of stock.
#[derive(PayloadCodec, Clone, Debug, PartialEq)]
pub struct Trade {
    pub costs: Vec<Item>,
    pub result: Item,
    pub out: bool,
}

/// Server → client, once when a player starts looking at a merchant (its network id, with at
/// most `look::TRADE_ROWS` trades) and once when the look ends (`entity` -1, no trades).
#[derive(PayloadCodec, Clone, Debug, PartialEq)]
pub struct Trades {
    pub entity: VarInt,
    pub trades: Vec<Trade>,
}

pub static HIT: OnceLock<ToClient<Hit>> = OnceLock::new();
pub static TRADES: OnceLock<ToClient<Trades>> = OnceLock::new();
