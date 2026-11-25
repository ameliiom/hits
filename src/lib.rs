//! Hits: each hit's damage rises from the mob it hurt, and a hurt mob shows its hearts above its
//! head for a few seconds, for everyone near. See `docs/design.md`.
//!
//! - **The look** (`look.rs`): text, colours, rise, hearts and the turn to the camera, on plain values.
//! - **The server** (`server.rs`): hits heard, the new health sent to players near.
//! - **The client** (`client.rs`): drawn above each mob after its render.
mod client;
pub mod look;
mod payloads;
mod server;

pub(crate) const MOD_ID: &str = "hits";

/// Settings (Options > Mods > Hits), for each player's own screen.
pub(crate) mod setting {
    pub const NUMBERS: &str = "numbers";
    pub const HEARTS: &str = "hearts";
}

use mod_api::{
    ClientHost, Flow, IPayloadContext, LevelRandomHost, LivingDamagePostEvent, LivingHealEvent, LoopContext, LoopSide, PayloadId,
    PlayerLoggedOutEvent, Registrar, RegistrationContext, RenderLivingPostEvent, ServerHost, ServerTickEvent, ServerTickPhase, SettingSpec,
};
#[allow(unused_imports)]
use mod_api::prelude::*;

fn on_hit(hit: payloads::Hit, _: &mut dyn IPayloadContext) -> Result<(), String> {
    let t = |v: mod_api::VarInt| v.0 as f32 / 10.0;
    client::hit(hit.entity.0.max(0) as u32, t(hit.amount), t(hit.health), t(hit.max));
    Ok(())
}

#[derive(Default)]
pub struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    fn register(&self, r: &mut dyn Registrar, _cx: &RegistrationContext) {
        for (key, label) in [(setting::NUMBERS, "hits.config.numbers"), (setting::HEARTS, "hits.config.hearts")] {
            if let Err(e) = r.register_setting(SettingSpec::toggle(key, label, true)) {
                eprintln!("[hits] setting {key}: {e:?}");
            }
        }
        let mut handlers = r.payload_handlers();
        let mut v1 = handlers.registrar("1").optional();
        let _ = payloads::HIT.set(v1.play_to_client_typed(PayloadId("hits:hit".into()), on_hit));
    }

    /// A living entity took a hit (its final damage, after armour and the rest).
    fn living_damage_post(&self, _: &mut dyn LevelRandomHost, event: &mut LivingDamagePostEvent<'_>) {
        server::changed(event.entity, event.container.new_damage());
    }

    /// A living entity heals (on the server's tick: the client's copy isn't told).
    fn living_heal(&self, context: &dyn LoopContext, event: &mut LivingHealEvent) -> Flow {
        if matches!(context.side(), LoopSide::Server)
            && let Ok(id) = i32::try_from(event.entity)
        {
            server::changed(id, -event.amount);
        }
        Flow::Continue
    }

    fn server_tick(&self, host: &mut dyn ServerHost, event: &mut ServerTickEvent<'_>) {
        if matches!(event.phase, ServerTickPhase::ServerTickEnd { .. }) {
            server::tick(host);
        }
    }

    fn player_logged_out(&self, _: &mut dyn ServerHost, event: &mut PlayerLoggedOutEvent<'_>) {
        server::forget(event.player);
    }

    fn render_living_post(&self, host: &dyn ClientHost, event: &mut RenderLivingPostEvent<'_>) {
        client::draw(host, event);
    }
}
