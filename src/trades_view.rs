//! The client's side of trades: the ones the server sent for the merchant looked at, kept until
//! the look ends, drawn above its head facing the camera: per row the cost icons, an arrow and
//! the result (grey arrow when out of stock), the first trade on top.
use crate::look;
use crate::payloads::{Item, Trade, Trades};
use mod_api::{ClientHost, Identifier, ItemDisplayContext, ItemStack, RenderLivingPostEvent, Text};
#[allow(unused_imports)]
use mod_api::prelude::*;
use std::sync::Mutex;

static SHOWN: Mutex<Option<(u32, Vec<Trade>)>> = Mutex::new(None);

/// The server told of a look's start (or end, as entity -1).
pub fn trades(t: Trades) {
    let shown = u32::try_from(t.entity.0).ok().filter(|_| !t.trades.is_empty()).map(|id| (id, t.trades));
    *SHOWN.lock().unwrap_or_else(|e| e.into_inner()) = shown;
}

fn stack(host: &dyn mod_api::Host, item: &Item) -> Option<ItemStack> {
    let key = host.item_key(&Identifier::parse(&item.id).ok()?)?;
    ItemStack::new(key, item.count.0.max(1) as u32)
}

/// Draws one icon at `x` with its count beside it when more than one.
fn icon(host: &dyn mod_api::Host, d: &mut dyn mod_api::BlockEntityDrawContext, item: &Item, x: f32) {
    let Some(s) = stack(host, item) else { return };
    d.push();
    d.translate([x, 0.0, 0.0]);
    d.push();
    d.scale([look::ICON * 0.8, look::ICON * 0.8, 0.02]);
    d.item(&s, ItemDisplayContext::Gui, None, 0);
    d.pop();
    if item.count.0 > 1 {
        d.translate([look::ICON * 0.45, -look::ICON * 0.2, 0.01]);
        d.scale([0.012, -0.012, 0.012]);
        let t = Text::literal(item.count.0.to_string());
        let w = d.text_width(&t).unwrap_or(0.0);
        let _ = d.text(&t, [-w, -4.0], 0xFFFFFF, false, Some(0));
    }
    d.pop();
}

/// Draws the trades above `event`'s mob when it is the one looked at; true when it drew.
pub fn draw(host: &dyn ClientHost, event: &mut RenderLivingPostEvent<'_>, height: f32) {
    let Some(rows) = SHOWN.lock().unwrap_or_else(|e| e.into_inner()).as_ref().filter(|s| s.0 == event.entity.id).map(|s| s.1.clone())
    else {
        return;
    };
    let read: &dyn mod_api::Host = host;
    let camera = host.camera().map_or(0.0, |c| c.yaw);
    let d = &mut *event.draw;
    d.push();
    d.translate([0.0, height + 0.65, 0.0]);
    d.rotate_about([0.0, 1.0, 0.0], look::facing(event.entity.body_yaw, camera), [0.0, 0.0, 0.0]);
    for (i, t) in rows.iter().enumerate() {
        let at = look::row_layout(t.costs.len() > 1);
        d.push();
        d.translate([0.0, look::row_y(i, rows.len()), 0.0]);
        if let Some(a) = t.costs.first() {
            icon(read, d, a, at.cost_a);
        }
        if let (Some(b), Some(x)) = (t.costs.get(1), at.cost_b) {
            icon(read, d, b, x);
        }
        d.push();
        d.translate([at.arrow, 0.0, 0.0]);
        d.scale([0.02, -0.02, 0.02]);
        let arrow = Text::literal("→");
        let w = d.text_width(&arrow).unwrap_or(0.0);
        let _ = d.text(&arrow, [-w / 2.0, -4.0], if t.out { 0x707070 } else { 0xFFFFFF }, false, Some(0));
        d.pop();
        icon(read, d, &t.result, at.result);
        d.pop();
    }
    d.pop();
}
