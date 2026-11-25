//! Trades above a merchant: each few ticks the server finds the villager or wandering trader each
//! player looks at within `look::TRADE_RANGE` blocks; when that changes, it reads its offers once
//! and sends at most `look::TRADE_ROWS` of them (or the look's end) to that player only.
use crate::look;
use crate::payloads::{self, Item, Trade, Trades};
use mod_api::{ItemStack, MerchantOffer, PlayerRef, ServerHost, VarInt};
#[allow(unused_imports)]
use mod_api::prelude::*;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// How often looks are checked, in ticks.
const EVERY: u32 = 4;

/// The merchant each player was last told of (its network id).
static LOOKING: Mutex<BTreeMap<u128, u32>> = Mutex::new(BTreeMap::new());
static TICKS: Mutex<u32> = Mutex::new(0);

fn looking<R>(f: impl FnOnce(&mut BTreeMap<u128, u32>) -> R) -> R {
    f(&mut LOOKING.lock().unwrap_or_else(|e| e.into_inner()))
}

/// The merchant `p` looks at, nearest first, with nothing solid between.
fn looked_at(host: &dyn ServerHost, p: PlayerRef) -> Option<mod_api::MerchantRef> {
    let me = p.entity();
    let (eye, dir, level) = (me.eye_position(host)?, me.view_vector(host)?, me.level(host)?);
    let r = look::TRADE_RANGE;
    let around = level.entities_in(host, eye.map(|v| v - r), eye.map(|v| v + r))?;
    let (t, merchant) = around
        .into_iter()
        .filter(|e| e.id() != me.id())
        .filter_map(|e| {
            let m = host.merchant(e)?;
            let [min, max] = e.bounding_box(host)?;
            let t = look::ray_box(eye, dir, min, max)?;
            (t <= r).then_some((t, m))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))?;
    let to = [0, 1, 2].map(|i| eye[i] + dir[i] * t);
    level.line_of_sight(host, eye, to).unwrap_or(true).then_some(merchant)
}

fn item(host: &dyn ServerHost, stack: &ItemStack) -> Item {
    Item { id: stack.item().id(host), count: VarInt(stack.count() as i32) }
}

fn trade(host: &dyn ServerHost, offer: &MerchantOffer) -> Trade {
    let costs = std::iter::once(&offer.base_cost_a).chain(offer.cost_b.as_ref()).map(|c| item(host, &c.stack)).collect();
    Trade { costs, result: item(host, &offer.result), out: offer.uses >= offer.max_uses }
}

pub fn tick(host: &mut dyn ServerHost) {
    let due = {
        let mut t = TICKS.lock().unwrap_or_else(|e| e.into_inner());
        *t = t.wrapping_add(1);
        *t % EVERY == 0
    };
    let Some(token) = payloads::TRADES.get() else { return };
    if !due {
        return;
    }
    for p in host.players().server_players() {
        let now = looked_at(&*host, p);
        let id = now.map(|m| m.entity().id());
        if looking(|l| l.get(&p.uuid()).copied()) == id {
            continue;
        }
        let message = match now {
            Some(m) => {
                let offers = m.offers(&*host).unwrap_or_default();
                Trades {
                    entity: VarInt(m.entity().id() as i32),
                    trades: look::capped(offers.iter().map(|o| trade(&*host, o))),
                }
            }
            None => Trades { entity: VarInt(-1), trades: Vec::new() },
        };
        // Remembered only once sent, so a failed send is tried again on the next check.
        match token.send(host, p, &message) {
            Ok(_) => looking(|l| match id {
                Some(id) => l.insert(p.uuid(), id),
                None => l.remove(&p.uuid()),
            }),
            Err(e) => {
                eprintln!("[hits] sending trades: {e:?}");
                None
            }
        };
    }
}

pub fn forget(player: u128) {
    looking(|l| l.remove(&player));
}
