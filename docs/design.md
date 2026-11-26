# Hits: design

Hit a mob and the damage rises from it as a number; a hurt mob shows its hearts above its head for
a few seconds. Everyone nearby sees them, in the game's own font and heart sprites.

## 1. Outcome

You and your friend see what each hit did and how close a mob is to dying, without aiming at it
(Jade already shows the health of the one you aim at).

## 2. Prior art (searched 2026-11-25)

| What | Where | Kind | Use |
|---|---|---|---|
| Horizon mods (GitHub, 38 repos) | GitHub | none (Jade shows the aimed entity's health) | build ours |
| ToroHealth, Neat, Damage Indicators | Java | floating numbers, health bars over mobs | design only |

## 3. How it plays

- **Numbers**: each hit's damage rises and fades over a second, yellow; a critical hit is bigger
  and red; healing is green.
- **Hearts**: a mob hurt in the last 5 seconds shows its hearts above its head (the HUD's own heart
  sprites, half hearts included; a big mob a short bar instead).
- Seen by everyone within 32 blocks; settings to turn either off.
- **Trades (0.3.0)**: a villager or wandering trader you look at within 8 blocks shows its first 6
  trades above its head, item icons facing you: cost (one or two) → result, counts beside them, a
  grey arrow when out of stock, in 2 columns read left to right from the top (0.3.1). A setting turns it off.
- Nothing to earn.

## 4. Choices

| Question | Picked | Over | Why |
|---|---|---|---|
| Where health comes from | the server, on each hit (`living_damage_post`), sent to players near | the client | measured: a client reads only its own player's health (`living()` is None for others) |
| Where it is drawn | above the mob, in the world (`render_living_post`) | the HUD | it stays with the mob |
| Who finds the merchant looked at | the server, every 4 ticks: the player's eye ray against merchants' boxes within 8 blocks, nothing solid between | the client's crosshair | the crosshair reaches 3 blocks; offers exist only on the server |
| When trades are sent | once when a look starts (at most 6, ids and counts) and once when it ends, to that player only | every tick | the payload stays small; a trade made while looking shows after looking away and back |

## 5. Measured (owner's PC, Horizon f037744, a spike, 2026-11-25)

| What | Result |
|---|---|
| Reading a cow's or villager's health on the client | None: only the local player resolves, so the server must send it |
| Text drawn above a mob after its render | drawn, in the game's font with an outline |
| Facing the camera | the draw is turned with the body. Of three turns tried, one faced the camera from the south only; from the two views the right turn works out as 180° + body yaw − camera yaw, not yet seen on screen (the skeleton checks it) |

## 6. Built (0.2.0), measured in the game (2026-11-25)

| Case | Result |
|---|---|
| A cow hit for 3 (of 10) | a red, bigger "-3" rising, hearts above it |
| A cow hit for 1.5 | a yellow "-1.5"; hearts 4 and a half |
| A cow at 4 healed by Instant Health | a green "+4", hearts back to 4 of 5 |
| The text facing the camera | read from the south and the east with the turn 180° + body yaw − camera yaw |
| Arabic | "-3" first came out "3-", then a left-to-right mark showed as a box; each character is now drawn on its own, left to right |
| Heal shown (0.2.1) | Instant Health on a sheep at 3 of 8 showed "+4" (3 → 7): the heal shows what health really rose by, not what was asked (Instant Health III on a pig had shown "+32"); a heal the game did not apply shows nothing (it had shown "-0") |
| Empty hearts (0.2.1) | drawn dark, a half heart pale: the font's outline heart had drawn as red as a full one |

## 6b. Trades (0.3.0), measured in the game (2026-11-26)

| Case | Result |
|---|---|
| A farmer with 8 trades set by command (one with two costs, one used up) | 6 rows, first on top; the two-cost rows show both icons; the used-up row's arrow grey |
| Looking at it from the south and from the east | the rows face the camera from both |
| Looking away, then back | gone at once, back within a moment |
| Close and looking up | the top rows went under the Jade panel: rows made smaller (0.26 block icons) and lower |
| Smaller rows, 3.7 blocks from the side (0.3.0) | the top 2 of 6 rows still under the Jade panel: 0.3.1 lays trades in 2 columns of 3 rows, half the height: all 6 clear of the panel from the side (3.7 blocks) and the front (4.7) |

## 7. Risks and answers

| Risk | Answer |
|---|---|
| A big fight floods the network | at most 20 numbers a second per player, nearest first |
| Clutter | numbers last a second, hearts five; both can be turned off |
| Trades flood the network | one message per look start and end, at most 6 trades of 3 item ids |
