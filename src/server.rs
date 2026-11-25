//! The server: each hit a living entity takes is queued (the damage hook's host can't send), and
//! at the tick's end its new health goes to the players within `NEAR` blocks, at most
//! `PER_SECOND` hits a second each, nearest first.
use crate::payloads::{self, Hit};
use mod_api::{ServerHost, VarInt};
#[allow(unused_imports)]
use mod_api::prelude::*;
use std::collections::BTreeMap;
use std::sync::Mutex;

const NEAR: f64 = 32.0;
const PER_SECOND: u32 = 20;

struct State {
    /// Hits this tick: entity id and damage.
    hits: Vec<(i32, f32)>,
    /// Hits sent to each player in the current second, and which second.
    sent: BTreeMap<u128, (u32, u32)>,
    ticks: u32,
}

static STATE: Mutex<State> = Mutex::new(State { hits: Vec::new(), sent: BTreeMap::new(), ticks: 0 });

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    f(&mut STATE.lock().unwrap_or_else(|e| e.into_inner()))
}

pub fn hurt(entity: i32, amount: f32) {
    if amount > 0.0 {
        with(|s| s.hits.push((entity, amount)));
    }
}

fn tenths(v: f32) -> VarInt {
    VarInt((v * 10.0).round() as i32)
}

pub fn tick(host: &mut dyn ServerHost) {
    let (hits, second) = with(|s| {
        s.ticks = s.ticks.wrapping_add(1);
        (std::mem::take(&mut s.hits), s.ticks / 20)
    });
    if hits.is_empty() {
        return;
    }
    let Some(token) = payloads::HIT.get() else { return };
    let players = host.players().server_players();
    for (id, amount) in hits {
        let Ok(id_u) = u32::try_from(id) else { continue };
        let Some(entity) = host.entity(id_u) else { continue };
        let Some(living) = host.living(entity) else { continue };
        let (Some(health), Some(max), Some(at)) = (living.health(&*host), living.max_health(&*host), entity.position(&*host)) else { continue };
        let hit = Hit { entity: VarInt(id), amount: tenths(amount), health: tenths(health), max: tenths(max) };
        let mut near: Vec<(f64, mod_api::PlayerRef)> = players
            .iter()
            .filter_map(|p| {
                let pos = p.entity().position(&*host)?;
                let d = (0..3).map(|i| (pos[i] - at[i]).powi(2)).sum::<f64>().sqrt();
                (d <= NEAR).then_some((d, *p))
            })
            .collect();
        near.sort_by(|a, b| a.0.total_cmp(&b.0));
        for (_, p) in near {
            let allowed = with(|s| {
                let e = s.sent.entry(p.uuid()).or_insert((second, 0));
                if e.0 != second {
                    *e = (second, 0);
                }
                e.1 += 1;
                e.1 <= PER_SECOND
            });
            if allowed && let Err(e) = token.send(host, p, &hit) {
                eprintln!("[hits] sending a hit: {e:?}");
            }
        }
    }
}

pub fn forget(player: u128) {
    with(|s| s.sent.remove(&player));
}
