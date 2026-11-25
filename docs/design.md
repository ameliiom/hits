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
- Nothing to earn.

## 4. Choices

| Question | Picked | Over | Why |
|---|---|---|---|
| Where health comes from | the server, on each hit (`living_damage_post`), sent to players near | the client | measured: a client reads only its own player's health (`living()` is None for others) |
| Where it is drawn | above the mob, in the world (`render_living_post`) | the HUD | it stays with the mob |

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

## 7. Risks and answers

| Risk | Answer |
|---|---|
| A big fight floods the network | at most 20 numbers a second per player, nearest first |
| Clutter | numbers last a second, hearts five; both can be turned off |
