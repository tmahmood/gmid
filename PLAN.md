# gmid — Implementation Plan & Design Document

> A small top-down arena shooter built with **Rust + SDL3 (`sdl3-rs`) + egui**
> (`egui-sdl3`, Canvas backend).
>
> Spec source: [SPEC.md](SPEC.md)

---

## 1. Goal

Build a playable, self-contained game where the player can:

1. Move a character around the arena.
2. Press **Space** to fire a gun (aimed with the mouse).
3. Get targeted by **projectile launchers** placed in the environment that
   fire **homing missiles** at the player.

Constraints from the spec:

- Rust, SDL3 with egui integration.
- Keep it simple.
- No tests required.
- Descriptive function names, no single-letter variable names.

---

## 2. Technology choices

| Concern | Choice | Why |
| --- | --- | --- |
| Language | Rust 2021 | Required by spec. |
| Window / input / 2D renderer | `sdl3` crate (sdl3-rs, `vhspace/sdl3-rs`) | Required by spec. Links against the system SDL3 via `pkg-config`. |
| UI / HUD | `egui-sdl3` (Canvas backend) | Provides event translation plus a `Canvas<Window>` we can draw the game world with, and egui on top for HUD/overlays. |
| Shapes / art | SDL renderer primitives (rects, lines, circles built from scanlines) | Zero assets, zero extra dependencies, simple to build. |
| Game loop | Single-threaded, delta-time based | Simple and adequate for a small arcade game. |

Environment check performed:

- `rustc` / `cargo` available.
- SDL3 3.4.18 installed (`pkg-config --modversion sdl3`).
- SDL3 shared libraries present in `/usr/lib`.
- Rust bindings compiled with SDL3's `use-pkg-config` feature.

No test suite is added, intentionally, per the spec.

---

## 3. Game design

### 3.1 Fantasy

A neon arena. The player is a mobile gunner. Static turrets ("launchers")
track the player and launch slow, homing missiles. The player dodges and shoots
both the launchers and the incoming missiles.

### 3.2 Controls

| Input | Action |
| --- | --- |
| `W` `A` `S` `D` or arrow keys | Move |
| Mouse | Aim the gun |
| `Space` (hold) | Fire bullets (rate limited) |
| `R` | Restart after game over |
| `Esc` / window close | Quit |

### 3.3 Entities

- **Player**
  - Circular body, radius 16, 100 HP.
  - Movement speed 260 px/s, clamped inside the arena.
  - Gun angle follows the mouse.
  - Fire cooldown 0.12 s; bullets travel at 900 px/s.
  - Brief invulnerability window and hit flash after taking damage.

- **Bullet** (player projectile)
  - Radius 4, lifetime 1.2 s, 900 px/s.
  - Damages missiles and launchers.
  - Removed on impact or when leaving the arena.

- **Launcher** (environment turret)
  - Static, 4 placed around the arena at fixed positions.
  - Tracks the player, fires a missile every 3.0 s (staggered start).
  - 3 bullet hits to destroy; respawns after 7 s.
  - Grants score when destroyed.

- **Missile** (launcher projectile)
  - Radius 7, initial speed 170 px/s accelerating to 260 px/s.
  - Homes toward the player with a limited turn rate (2.2 rad/s), so it is
    dodgeable.
  - Lifetime 8 s; explodes on contact with the player or when shot down.
  - Deals 16 damage on player contact.
  - Grants score when shot down.

- **Particle**
  - Short-lived visual sparks for firing, impacts, explosions, and damage.
  - Purely cosmetic.

### 3.4 Game rules

- Score increases for shooting missiles and destroying launchers.
- Health reaches 0 → **game over** state. World simulation stops.
- Restart resets the world (score, health, entities, launchers).
- Difficulty stays constant (no waves) to keep the implementation simple.

### 3.5 Rendering

- Dark navy background with a subtle grid and a bright arena border.
- Neon, high-contrast entity colors:
  - Player: cyan with a white gun barrel.
  - Bullets: yellow with a soft glow.
  - Launchers: orange/red with a health strip.
  - Missiles: red-pink with a fading trail.
  - Particles: fading warm/cool sparks.
- Circles are filled using horizontal scanlines over SDL `fill_rect` calls;
  lines use SDL `draw_line`. Everything is procedurally drawn — no assets.
- egui draws the HUD on top of the SDL-rendered world:
  - Health bar, score, launcher counter (top-left).
  - Controls hint (bottom).
  - Game-over card with final score and a Restart button.

---

## 4. Architecture

Small modules with one responsibility each:

```
src/
├── main.rs        # SDL/egui bootstrap, event pump, game loop, frame pacing
├── game.rs        # GameState, update rules, spawning, collisions, score
├── entities.rs    # Player, Bullet, Launcher, Missile, Particle structs
├── geometry.rs    # Vec2 math, distance/collision helpers, clamping
├── render.rs      # All SDL Canvas drawing for the game world
├── hud.rs         # egui HUD and game-over overlay
└── theme.rs       # Colors and gameplay tuning constants
```

### Frame flow

```
clear canvas
  → render_world(canvas, game)
  → egui.run(hud + overlay)
  → egui.paint()
  → egui.present()
  → sleep to target frame time
```

Input events feed both the game (keys/mouse) and egui. Events consumed by egui
are not forwarded to game controls.

### Update flow

```
read movement keys + mouse
  → update player (move, aim, fire)
  → update launchers (cooldown, spawn missiles)
  → update missiles (homing, movement, player collision)
  → update bullets (movement, missile/launcher collision)
  → update particles
  → cleanup dead entities
  → check game over
```

Timing uses `Instant` deltas clamped to a maximum step so a stalled frame does
not tunnel projectiles through entities.

---

## 5. Milestones

| # | Milestone | Done when |
| --- | --- | --- |
| 1 | Project scaffold + SDL3/egui plumbing | Window opens, dark canvas renders, clean shutdown. |
| 2 | Player movement, mouse aim, shooting | WASD moves, aim follows mouse, Space fires rate-limited bullets. |
| 3 | Launchers + homing missiles + damage | Turrets fire missiles that track the player and reduce HP on hit. |
| 4 | Combat loop | Bullets destroy missiles/launchers, score updates, respawns work. |
| 5 | HUD + game over + restart | Health/score shown, death overlay, `R`/button restarts. |
| 6 | Polish + docs | Grid, glow, particles, trails, README, plan updated, pushed to GitHub. |

---

## 6. Repository

- GitHub repository: `https://github.com/tmahmood/gmid`
- Commits land incrementally while the milestones are implemented.
- `README.md` documents prerequisites, build/run instructions, controls, and
  the project layout.

---

## 7. Risks & mitigations

| Risk | Mitigation |
| --- | --- |
| `egui-sdl3` is a young crate; API may differ from docs. | Pin `0.4`, compile early with a minimal frame, adjust against compiler errors. |
| SDL3 discovery at build time. | Enable `sdl3/use-pkg-config`; verified SDL3 3.4.18 is installed. |
| HiDPI coordinate mismatch between SDL mouse coords and canvas pixels. | Convert mouse coordinates using window size vs. renderer output size. |
| egui repaint overwriting the game scene. | Draw the world first, then run/paint egui, then present once per frame. |
| Frame-rate dependent movement. | Delta-time updates with a clamped max step. |

---

## 8. Status

- [x] Read and analyze `SPEC.md`
- [x] Define plan and architecture (this document)
- [x] Scaffold project and verify SDL3 + egui build
- [x] Player movement, aim, firing
- [x] Launchers and homing missiles
- [x] Collisions, score, health, restart
- [x] HUD and polish
- [x] README and final push

---

## 9. Verification log

There is no automated test suite (explicitly out of scope), but the build and
the game loop were exercised directly:

1. `cargo build` — clean compile against SDL3 3.4.18 via `pkg-config`.
2. `cargo fmt` — source formatted.
3. `cargo clippy --all-targets` — no remaining warnings.
4. Scripted runtime check under `Xvfb` with `xdotool`:
   - Window opens, world and HUD render.
   - Mouse aim and Space firing produce bullets.
   - Launchers fire homing missiles that damage the player.
   - Standing still leads to game over (HUD + overlay confirm).
   - The game-over **Restart** button and the `R` key both reset the run.
   - Movement keys move the player and bullets destroy launchers/missiles,
     updating the score.

The screenshot at `docs/screenshot.png` was captured from a live run.

