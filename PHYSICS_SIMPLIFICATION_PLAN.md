# Physics Simplification Plan

This plan is for simplifying the Avian physics code after the Rapier migration.
The goal is not to rewrite the game, but to make the current behavior easier to
understand, tune, and trust.

## Goals

- Keep the game simple enough for a small pinball project.
- Make collider geometry easy to inspect and reason about.
- Reduce hidden gameplay corrections and position-window hacks.
- Keep the good tuned behavior already found during testing.
- Make future physics tuning happen in one place, not scattered across systems.

## Current Assessment

The game works better now, but the code still carries migration/debugging
complexity. The most important examples are:

- `wall.rs` contains floor, visual assets, outer walls, bottom drain, launcher
  lane, flipper guides, and wall helper functions in one large setup function.
- `launcher.rs` still has manual ball guidance and anti-stuck velocity logic.
- `flipper.rs` is simpler than before and now relies on kinematic flippers with
  compound primitive colliders, but flipper physics is still not fully
  satisfactory. In particular, Avian contact response near the flipper tip can
  feel weak or inconsistent, and the ball does not always get the expected
  forward/diagonal motion from a flipper hit.
- Physics constants are hardcoded across several files.
- Debug rendering is manually toggled in `main.rs`.

## Phase 1: Make Geometry Code Readable

### 1. Split `wall.rs`

Break `spawn_walls` into small private helpers:

- `spawn_floor`
- `spawn_floor_collider`
- `spawn_table_art`
- `spawn_outer_wall_visual`
- `spawn_outer_wall_colliders`
- `spawn_bottom_drain_sensor`
- `spawn_launcher_bottom_stop`
- `spawn_flipper_guides`
- `spawn_launcher_lane_wall`

This should be a pure refactor. The colliders and positions should not change.

Verification:

- `cargo fmt --check`
- `cargo check`
- run with wireframe and compare against the current layout

### 2. Introduce Named Geometry Constants

Move important board geometry values to named constants in the relevant module,
or to a small `geometry.rs` module if they are shared:

- board tilt
- wall thickness
- outer wall left/right x
- bottom y
- arc center
- arc radius
- arc segment count
- launcher x-range
- launcher wall position/height
- ball radius
- flipper positions

Avoid changing values in this step. This is only naming.

Verification:

- no gameplay change expected
- `cargo check`

## Phase 2: Simplify Outer Wall Colliders

### 3. Keep The Current Working Outer Wall Shape

The current outer wall solution should be treated as the baseline:

- straight side segments go to the arc center y
- arc is tangent to side walls
- arc has 24 segments
- segment colliders have no length overhang
- restitution is zero on outer walls
- rounded joints are used only on interior arc points

Do not change this until other cleanup is done.

### 4. Consider A Single Compound Collider For Outer Wall

If Avian supports it cleanly, replace the many outer wall entities with one
compound collider entity. This may reduce boundary-edge behavior between
separate static colliders.

Important: do this only after Phase 1, and compare behavior carefully.

Possible outcome:

- If it behaves better, keep it.
- If it behaves worse or wireframe becomes harder to inspect, revert it.

Verification:

- launcher shot along right wall must glide into the arc without a sharp kick
- ball coming from above must still follow the arc naturally
- no ball disappearing or tunneling

## Phase 3: Reduce Flipper Special Cases

### 5. Keep Current Collider Model

The current flipper collider model is reasonable for pinball:

- large cylinder near base
- small cylinder near tip
- two side boxes between them

Do not return to mesh colliders for flippers unless there is a strong reason.
The compound primitive approach is easier to tune and generally more stable.

### 6. Reassess Flipper Physics

Current behavior:

- flippers are kinematic bodies
- colliders use the current compound primitive shape
- no manual ball kick or active drive assist is currently applied

The current direction is to avoid gameplay assistance and let Avian contacts do
the work. However, testing shows that the flippers still do not behave fully
like pinball flippers, especially near the tip.

Recommended test:

1. Keep the compound primitive collider model.
2. Test whether higher solver/substep/contact settings improve tip contacts.
3. Tune flipper friction/restitution with one change at a time.
4. If pure Avian contact still cannot produce believable shots, decide whether a
   small, well-documented gameplay assist is acceptable.

If an assist is reintroduced, make that explicit in naming:

- `assist_ball_from_active_flippers`

Verification:

- left flipper should be able to send ball up/right
- right flipper should be able to send ball up/left
- ball should not tunnel through flippers at normal speeds

## Phase 4: Simplify Launcher

### 7. Separate Launcher Into Clear Responsibilities

Split launcher logic into:

- input charging
- launch impulse/velocity
- optional lane guidance
- anti-stuck fallback

Each part should have named constants.

### 8. Make Launcher Guidance Explicit

The launcher guide is currently a pragmatic correction. Keep it only if it is
needed, but make the intent obvious:

- guide should only apply while the ball is moving upward inside the launcher
- guide should not affect balls after they have exited the lane
- anti-stuck should be rare and easy to identify

Possible naming:

- `charge_launcher`
- `launch_charged_ball`
- `guide_ball_up_launcher_lane`
- `unstick_ball_from_launcher_lane`

Verification:

- holding space longer increases force up to a cap
- holding space for several seconds does not produce extreme velocity
- ball does not bounce forever in launcher
- ball can leave launcher consistently
- ball can still drain/respawn outside launcher

## Phase 5: Clean Up Debug Toggles

### 9. Add A Simple Debug Flag

Instead of manually editing `main.rs` to show wireframe, add one obvious switch:

```rust
const SHOW_PHYSICS_DEBUG: bool = false;
```

Then conditionally add `PhysicsDebugPlugin`.

This makes testing easier without accidental commits that leave wireframe on.

Verification:

- `false`: normal game
- `true`: collider wireframe visible

## Phase 6: Tune Physics In One Place

### 10. Centralize Physics Tuning

Collect global Avian tuning values:

- gravity
- substep count
- solver config
- ball friction/restitution
- wall restitution/friction
- flipper friction/restitution
- launcher min/max speed

This can be a `physics_config.rs` module or a set of constants near the systems
that own them. Prefer local constants unless values are reused.

## Suggested Order

1. Split `wall.rs` helpers.
2. Name constants without changing values.
3. Add debug flag.
4. Re-test current physics baseline.
5. Simplify launcher structure.
6. Test disabling flipper active drive.
7. Consider compound outer wall collider only if the transition still feels off.

## Do Not Do Yet

- Do not switch flippers back to mesh colliders.
- Do not change many physics constants in one commit.
- Do not change launcher and flippers in the same commit.
- Do not remove CCD/speculative margins until there is a focused test for
  tunneling.

## Commit Strategy

Use small commits:

- `Refactor wall setup helpers`
- `Name board physics constants`
- `Add physics debug flag`
- `Simplify launcher systems`
- `Document flipper assist behavior`

Each commit should pass:

- `cargo fmt --check`
- `cargo check`

For gameplay-sensitive commits, also test manually with wireframe before
pushing.
