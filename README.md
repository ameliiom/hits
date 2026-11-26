# Hits

For [Horizon](https://playhorizon.dev) (Minecraft: Java Edition 26.3 in Rust). Hit a mob and the damage rises from it as a number; a hurt mob shows its hearts above its head for a few seconds. Everyone near sees them. Look at a villager or wandering trader and its trades show above its head. English and Arabic.

## How it plays
- Each hit's damage rises and fades over a second: yellow, a big hit (a quarter of the mob's life) bigger and red. A heal of 2 or more rises green (+4).
- A mob hurt or healed in the last 5 seconds shows its hearts above its head; a mob with more than ten hearts shows its health as a number.
- Within 32 blocks, at most 20 numbers a second per player.
- **Trades**: look at a villager or wandering trader within 8 blocks and its first 6 trades show above its head in 2 columns, as item icons: cost (one or two) → result, the count beside each; a grey arrow is out of stock. Only you see the ones you look at. A villager nobody has traded with has no trades yet (the game rolls them on the first trade).
- **Options > Mods > Hits**: numbers, hearts and trades each on or off.

## Install
1. Copy `hits.hmod` into `%APPDATA%\horizon\horizon-native-mods\`.
2. Add the line `mod.hits:true` to `%APPDATA%\horizon\horizon-mods.txt`.
3. Build: the launcher's Mods page **Build**, or `horizon-launcher native-build --instance horizon --trust-native-code`. A native mod runs with your rights: install it only from a source you trust.

The server and every player need the mod. Built for Horizon Nightly `f037744` (2026-11-21).

## Licence
MIT, see [LICENSE](LICENSE). Not affiliated with the Horizon project.
