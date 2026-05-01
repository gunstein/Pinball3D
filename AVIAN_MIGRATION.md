# Migrasjon fra bevy_rapier3d til Avian Physics

## Mål

- Bytte fra `bevy_rapier3d` til `avian3d`
- Utnytte Avians ECS-native hierarki-støtte for å:
  - Fjerne `common::board_transform()` — bruk en foreldre-entitet for brettet i stedet
  - Åpne for dynamisk skalering basert på vindusoppløsning

## Nåværende tilstand

- Bevy `0.18`
- `bevy_rapier3d = "0.33"`
- `rand = "0.8"`

## Mål-avhengigheter

```toml
avian3d = "0.6"
```

`avian3d` 0.6.x støtter Bevy 0.18.

---

## API-mapping: Rapier → Avian

### RigidBody

| Rapier | Avian |
|--------|-------|
| `RigidBody::Dynamic` | `RigidBody::Dynamic` |
| `RigidBody::Fixed` | `RigidBody::Static` |
| `RigidBody::KinematicPositionBased` | `RigidBody::Kinematic` |

### Collider

| Rapier | Avian |
|--------|-------|
| `Collider::ball(r)` | `Collider::sphere(r)` |
| `Collider::cuboid(hx, hy, hz)` | `Collider::cuboid(hx*2, hy*2, hz*2)` *(Avian bruker fulle mål, ikke halve)* |
| `Collider::cylinder(hh, r)` | `Collider::cylinder(hh*2, r)` *(samme konvensjon)* |
| `Collider::capsule3d / round_cylinder` | `Collider::capsule(r, h)` — trenger verifisering |
| `Collider::compound(vec)` | `Collider::compound(vec)` — trolig lik struktur |
| `Collider::heightfield(h, r, c, s)` | Trenger verifisering |

> **OBS:** Avian bruker typisk fulle lengder (ikke halve) for kuboid og sylinder. Må verifiseres.

### Bevegelse og krefter

| Rapier | Avian |
|--------|-------|
| `Velocity { linvel, angvel }` | `LinearVelocity(Vec3)` + `AngularVelocity(Vec3)` (separate komponenter) |
| `ExternalForce { force, torque }` | `ExternalForce` + `ExternalTorque` |
| `ExternalImpulse { impulse, torque_impulse }` | `ExternalImpulse` + `ExternalAngularImpulse` |

### Fysikkegenskaper

| Rapier | Avian |
|--------|-------|
| `Friction { coefficient, combine_rule }` | `Friction(f32)` (enklere) |
| `Restitution::coefficient(x)` | `Restitution(x)` |
| `Sleeping::disabled()` | `SleepingDisabled` (marker-komponent) |
| `Ccd::enabled()` | `SweptCcd` eller `SpeculativeCcd` |

### Kollisjonsgrupper

Avian bruker et layer-basert system med `CollisionLayers`:

```rust
// Rapier:
CollisionGroups { memberships: Group::GROUP_1, filters: Group::GROUP_3 }

// Avian — definer layers som enum:
#[derive(PhysicsLayer, Default, Clone, Copy)]
enum GameLayer {
    #[default]
    Default,
    Floor,
    Walls,
    Ball,
    // osv.
}

CollisionLayers::new(GameLayer::Floor, [GameLayer::Ball])
```

> Dette er en større refaktorering. Nåværende GROUP_1–GROUP_5-system må mappes til navngitte layers.

### Kollisjons-events

| Rapier | Avian |
|--------|-------|
| `ActiveEvents::COLLISION_EVENTS` | Ikke nødvendig — Avian sender events automatisk |
| `MessageReader<CollisionEvent>` | `EventReader<CollisionStarted>` / `EventReader<CollisionEnded>` |
| `CollisionEvent::Started(h1, h2, _)` | `CollisionStarted(e1, e2)` — trenger verifisering av struktur |
| `rapier_context.intersection_pair(e1, e2)` | Trenger verifisering — trolig `SpatialQuery` |

### Shape casting

| Rapier | Avian |
|--------|-------|
| `ReadRapierContext` + `.cast_shape(...)` | `SpatialQuery` system-parameter + `.cast_shape(...)` |

Syntaksen er trolig lik, men via `SpatialQuery`:
```rust
fn system(spatial_query: SpatialQuery) {
    spatial_query.cast_shape(&collider, pos, rot, dir, max_dist, true, &filter);
}
```

### Joints

| Rapier | Avian |
|--------|-------|
| `RevoluteJointBuilder::new(axis).limits(...).local_anchor1(...).local_anchor2(...)` | `RevoluteJoint::new(entity1, entity2).with_angle_limits(...).with_local_anchor_1(...)` |
| `ImpulseJoint::new(anchor, joint)` | Avian bruker komponenten direkte på entiteten |

### Gravitasjon og konfigurasjon

| Rapier | Avian |
|--------|-------|
| `RapierPhysicsPlugin::<NoUserData>::default()` | `PhysicsPlugins::default()` |
| `TimestepMode::Variable { ... }` | `PhysicsPlugins::new(fixed_hz)` eller `Time<Physics>` |
| `RapierConfiguration { gravity }` | `Gravity(Vec3::new(...))` resource |

---

## Brett-hierarki (det store gevinsten)

Avian støtter collider-hierarkier, men dynamiske rigid bodies og joints ble ustabile når hele
brettet lå under en rotert parent. Denne migreringen bruker derfor samme modell som den opprinnelige
Rapier-versjonen: objektene får ferdig rotert world-space transform via `common::board_transform()`.

```rust
commands.spawn((
    RigidBody::Static,
    Collider::cuboid(...),
    common::board_transform(Transform::from_xyz(0.0, -0.3, 0.0)),
));
```

`common.rs` beholdes for `board_transform()`, `GameLayer`, `EndGame` og `DespawnInEndGame`.

---

## Migrasjonssteg

### Steg 1 — Avhengigheter ✅
- [x] Bytt `bevy_rapier3d` med `avian3d = "0.6"` i Cargo.toml
- [x] Fjern `bevy_rapier3d`-import fra alle filer

### Steg 2 — Plugin og gravitasjon (`main.rs`) ✅
- [x] Bytt `RapierPhysicsPlugin` med `PhysicsPlugins`
- [x] Bytt `TimestepMode` med Avian-ekvivalent
- [x] Bytt `RapierConfiguration` med `Gravity`-resource
- [x] Fjern `DefaultRapierContext`-query

### Steg 3 — Brett-transform
- [x] Behold `common::board_transform()` for world-space fysikktransforms
- [x] Ikke parent dynamiske rigid bodies eller joints under en rotert `Board`
- [ ] Vurder en egen, ren visuell board-parent senere hvis det trengs for skalering

### Steg 4-8 — Fysikk-komponenter, events, shape casting, joints ✅ (gjort i steg 1-2)
- [x] GameLayer enum i common.rs (GROUP_1–GROUP_5 → navngitte layers)
- [x] Alle filer: RigidBody, Collider, Friction, Restitution, Sleeping, Ccd
- [x] Kollisjonsevent: MessageReader<CollisionEvent> → CollisionStart/CollisionEnd
- [x] ball.rs: intersection_pair → CollisionStart events
- [x] ball.rs: push_ball_to_floor → SpatialQuery
- [x] launcher.rs: RevoluteJointBuilder/ImpulseJoint → RevoluteJoint

---

## Usikkerhetspunkter (må verifiseres)

1. **Collider-mål:** Avian halvmål vs. fullmål for cuboid/cylinder — sjekk docs
2. **`round_cylinder`:** Tilgjengelig i Avian? Alternativ?
3. **`intersection_pair`:** Avian-ekvivalent for å sjekke om to entiteter overlapper
4. **Kollisjonsevent-struktur:** Nøyaktig type og felt for `CollisionStarted`/`CollisionEnded`
5. **Kinematisk bevegelse:** Avian kinematic + direkte Transform-sett — fungerer på samme måte?
6. **`SleepingDisabled`:** Nøyaktig komponentnavn i avian3d 0.6
7. **Parent-child fysikk:** Bekreft at Avian propagerer transforms korrekt til fysikk-barn

---

## Oppløsningsstøtte (fremtidig, etter migrering)

Når brett-hierarkiet er på plass:
1. Les vindusstørrelse ved oppstart
2. Beregn skalefaktor basert på måloppløsning (360×640)
3. Sett `scale` på `Board`-entiteten
4. Skaler fysikk-konstanter (impulser, krefter, gravitasjon) proporsjonalt
5. Juster kamera-FOV eller -posisjon

Fysikk-konstantene som må skaleres:
- Gravitasjon: `Vec3::new(0.0, -0.3, -0.5)`
- Ball-kraft: `0.0001`
- Bumper-impuls: `0.000003`
- Target-impuls: `0.000013`
- Launcher-bevegelse: `0.03` / `0.02` per frame
