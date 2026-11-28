# Hits

For [Horizon](https://playhorizon.dev) (Minecraft: Java Edition 26.3 in Rust). Hit a mob and the damage rises from it as a number. Everyone near sees it. A mob's health is in [Jade](https://github.com/ameliiom/jade)'s panel when you look at it, a friend's in Team's cards, so Hits shows numbers only. English and Arabic.

## How it plays
- Each hit's damage rises and fades over a second: yellow, a big hit (a quarter of the mob's life) bigger and red. A heal of 2 or more rises green (+4).
- Players show nothing: their health is in Team's cards.
- Within 32 blocks, at most 20 numbers a second per player.
- **Options > Mods > Hits**: numbers on or off.
- A villager's trades show in [Jade](https://github.com/ameliiom/jade)'s panel when you look at it (Hits 0.3 drew them above its head).

## Install
Needs [Kit](https://github.com/ameliiom/kit) 0.1 or later, the shared code of these mods, installed the same way (`kit.hmod`, `mod.kit:true`).

1. Copy `hits.hmod` into `%APPDATA%\horizon\horizon-native-mods\`.
2. Add the line `mod.hits:true` to `%APPDATA%\horizon\horizon-mods.txt`.
3. Build: the launcher's Mods page **Build**, or `horizon-launcher native-build --instance horizon --trust-native-code`. A native mod runs with your rights: install it only from a source you trust.

The server and every player need the mod. Built for Horizon Nightly `f037744` (2026-11-21).

## Licence
MIT, see [LICENSE](LICENSE). Not affiliated with the Horizon project.
