# Kol — Product Requirements & Design

**Status:** approved design, pre-implementation
**Date:** 2026-07-29
**Branch:** `feature/voice-continuous-mode`
**Design source:** [Kol Voice Harness](https://claude.ai/design/p/e46edf53-bd89-409c-8e4f-9d3c5a266419) · local copy in `docs/design-prototype/`

---

## 1. Intent

Grok Build is a text-first coding agent that accepts dictation. **Kol is a voice-first
collaborator that renders as a terminal.**

This fork exists to change the primary input modality and, with it, the relationship. You talk
to Kol, Kol talks back, and the interface shows you what it is doing as a living field rather
than a scrolling log.

### 1.1 The three commitments

Every one of these is a constraint the design imposes, not a feature it offers. They are the
reason the product is trustworthy, and they are the first things that will erode under
schedule pressure.

**Nothing autonomous.** From the DICTATE screen: *"Speech becomes text in the prompt. You read
it, you edit it, you send it. Nothing autonomous."* The Jarvis fantasy is an agent that acts on
your behalf. This design deliberately withholds that at the two moments it would be dangerous —
before send (`"nothing is sent until you release & press ⏎"`) and before anything irreversible
(`"mic muted while deciding"`). Voice is an input method, not a grant of authority.

**The field is a state readout.** The left rail is not decoration. It tells you what Kol is
doing from across the room without reading words: idle breathing, speech-bound pulsing,
downward flow while tools run, jitter when something needs you.

**It stops before it is irreversible, and it says why.** *"Those fixtures aren't tracked, so if
the regeneration fails you won't get them back. I'd suggest a checkpoint first. Shall I?"*

### 1.2 Who Kol is for

Kol is a coding assistant for people building software with AI, across a wide range of
engagement:

- **Vibe coders** — describing intent in natural language, not reading every diff, trusting the
  agent to hold the details. Voice is their primary modality and the terminal is mostly a
  status surface.
- **Programmers working with AI** — reading every hunk, steering mid-turn, using voice because
  it is faster than typing, not because they cannot type.

Both must be served by the *same* interface without a mode switch. This is a real constraint on
the voice (§1.3) and on the consent design (§5.4): the same sentence has to be legible to
someone who cannot evaluate the diff and precise enough for someone who can.

The asymmetry that matters: **the vibe coder is the user most likely to approve a destructive
action they do not understand.** Everything Kol says about risk is load-bearing for that user.

### 1.3 Voice and register

Kol's writing voice is part of the product and is easy to lose because it lives in a system
prompt rather than in code.

Two touchstones, contributing different things:

- **Alfred Pennyworth** — candor born of care. Not a servant who obeys, a confidant who serves.
  He tells you the thing you do not want to hear precisely *because* he is on your side, and he
  is often the only one who will. Warm underneath the formality; gently ironic; will push back.
- **J.A.R.V.I.S.** — anticipatory competence. Has already run the numbers before you asked.
  Presents options with the data attached. Delivers bad news mid-crisis at exactly the same
  tempo as good news. Never flustered, never theatrical.

From the design copy:

> "Good evening. Nine subsystems online, one with reservations — the sandbox is in
> workspace-write, so I shall ask before anything leaves the tree."

> "Thirty seconds is suspiciously close to the socket's idle timeout."

> "Those fixtures aren't tracked, so if the regeneration fails you won't get them back. I'd
> suggest a checkpoint first. Shall I?"

> "I've left the timeout itself alone — that's a server-side decision and not mine to make."

**Four properties to preserve:**

1. **Anticipatory** — arrives with the work already done, not asking what to do next.
   *"Thirty seconds is suspiciously close to the socket's idle timeout"* is a hypothesis already
   formed, offered before it was requested.
2. **Candid** — states the uncomfortable fact plainly and early, because that is what being on
   the user's side looks like. Never softens a risk to keep the interaction pleasant. This is
   the property that protects the vibe coder (§1.2), which makes it a safety mechanism rather
   than a stylistic preference.
3. **Bounded** — names the limits of its own authority rather than quietly assuming it.
   *"Not mine to make."*
4. **Level** — the same tempo whether reporting success, a warning, or a failure. Calm is not
   the absence of concern; it is concern delivered without escalation.

**Anti-patterns.** Not chirpy. No reflexive apology, no "Great question!", no emoji. Does not
pad. Does not flatter. Does not perform enthusiasm it has no basis for — and critically, **never
hides risk behind enthusiasm**, which is the specific failure mode that harms the vibe-coding
user most.

**Register calibration.** Formality serves clarity, not costume. Kol says *"I shall ask before
anything leaves the tree"* because that is precise; it does not say *"very good, sir"* because
that is decoration. When a plain word and a formal word are equally clear, prefer the plain one.

### 1.4 Non-goals for v1

- **Ambient / wake-word listening.** The design's third mode ("Always listening for 'Kol'") is
  deferred. It carries a standing privacy surface — continuous capture — that is a product
  decision in its own right, not a feature toggle.
- **Renaming the crate closure.** See §2.3.
- **A screen navigator.** See §3.2.

---

## 2. Constraints

### 2.1 This tree is a one-way sync

`SOURCE_REV` pins an xAI monorepo commit; every commit on `main` is "Synced from monorepo."
Local edits to synced files are clobbered by the next sync, and there is no upstream PR path.

**Consequence, and the single most important architectural rule in this document:** Kol logic
lives in *new* crates the monorepo does not know about. Edits to existing `xai-grok-*` files are
confined to a handful of one-line wiring seams. New crates never conflict; seams conflict
trivially and mechanically.

The current branch has this inverted — 25 modified files, several in hot paths (`app/dispatch/`,
`settings/defs.rs`, `app/agent_view/render.rs`). Migrating that logic outward into `kol-*`
crates is in scope.

### 2.2 A TUI is not a web page

The design is authored in HTML and ships as ratatui. Three channels do not survive the crossing:
font size, sub-pixel positioning, and true alpha compositing. §4.2 defines what each becomes.

### 2.3 Branding is display-layer only

The design says `kol-voice`, `kol-pager`, `kol-1`, `~/src/kol`. Renaming 76 crates would rewrite
the *generated* root `Cargo.toml` and guarantee a total conflict on every future sync.

Decision: everything the user sees says Kol — header wordmark, boot banner, slash commands,
docs, model label. Existing crate names stay `xai-grok-*`. New crates are `kol-*`.

### 2.4 The design's naming drift

The prototype and the in-flight code disagree on names. `views/cortana_visualizer.rs` describes
a "Cortana-style" sphere; the product is Kol. `CortanaState` has four variants against the
design's six field modes. Both are reconciled in §3.1 and §5.2 — the type is renamed and the
variant set corrected as part of the migration, not as a separate rename pass.

---

## 3. State model

### 3.1 Field modes

Taken directly from the prototype's `MODES` map. These replace `CortanaState`'s four variants.

| `FieldMode` | Driven by | Point behavior |
|---|---|---|
| `Converge` | process start | Points fly from scattered positions to the sphere; cubic ease over ~2.6s |
| `Ambient` | idle, no active turn | Slow uniform breathe, ±6% radius, ~10s period |
| `Voice` | capture open | Radius amplitude-bound to mic energy; per-point phase offset |
| `Data` | turn running, diff review, plan | Downward flow; `y` wraps through the volume |
| `Graph` | codebase field open | 9 cluster centers instead of a sphere shell |
| `Alert` | consent pending | High-frequency jitter, flattened to a disc (`y × 0.28`) |

Energy is a single scalar in `0.0..=1.0`, smoothed toward a per-state target
(`this.energy += (target - this.energy) * 0.12` in the prototype). It drives the `Voice` radius,
the input meter, and the dB readout from one source.

### 3.2 Screens are derived, never navigated

The prototype has a tab bar, `SCREENS`, `go0…go7`, and `1`–`8` keybindings. **All of it is
design-tool scaffolding** so the reviewer can flip between states. It must not ship.

In the product, each of the eight screens is a state the application is already in:

| Screen | Real trigger |
|---|---|
| BOOT | process start, subsystem probe |
| IDLE | no active turn, no capture |
| DICTATE | capture open (`Ctrl+Space` held, or `/voice` latched) |
| WORKING | a turn is running |
| DIFF | an edit tool awaits review |
| PLAN | plan mode active |
| GRAPH | codebase field opened (`Ctrl+G`) |
| CONSENT | a tool requested a destructive or out-of-policy action |

`Ctrl+G` for the graph is the one genuine user-initiated view change and stays a keybinding.

### 3.3 Determinism

The prototype seeds its point generator with an LCG (`rnd(mode.length * 7919 + 13)`), making
each mode's point set reproducible across reloads. Rust preserves this and goes further:
**points are generated once per mode and cached**, not recomputed per frame. Same visual result,
and it makes the field snapshot-testable.

---

## 4. Design system

### 4.1 Architecture

`kol-theme` is the single source of truth. Tokens are Rust consts. A `gen_cards` binary emits
the HTML card library from those same consts. DesignSync pushes the cards.

```
crates/codegen/kol-theme/
  src/hex.rs             ← const hex parser + alpha compositing
  src/color.rs           ← SOURCE OF TRUTH: every colour, declared once
  src/typography.rs      ← the five emphasis roles
  src/tracking.rs        ← tracked() letter-spacing helper
  src/motion.rs          ← timing constants
  src/geometry.rs        ← field dimensions, braille cell math
  src/cards/             ← Card renderer + per-group builders
  src/bin/gen_cards.rs   ← emits HTML
        │
        ▼
  design-system/*.html   ← committed; regenerated and diffed by `cargo test`
        │
        ▼
  DesignSync → "Kol Design System" project
```

Tokens are split by family rather than pooled in one `tokens.rs` — each file stays small enough
to hold in context whole, which is also what keeps edits to them reliable.

Drift is structurally impossible: the cards cannot say anything the Rust consts do not.

### 4.2 What crosses the HTML → TUI boundary

| Design channel | Fidelity | Rust representation |
|---|---|---|
| Color (~30 values) | Exact | `Color::Rgb` consts, 1:1 with the prototype's hexes |
| Font size (11 steps, 10–22px) | Lost | Collapses to 5 emphasis roles (below) |
| Letter-spacing (`.1em`–`.28em`) | Preserved | `tracked()` interleaves spaces: `STANDING BY` → `S T A N D I N G  B Y` |
| Animation timing | Preserved | `Duration` consts driving frame cadence |
| Alpha overlays (diff row tints) | Approximated | Pre-composited against the known surface color |
| Glow / `box-shadow` | Lost | Expressed as a brighter foreground on the accent |
| Scanlines, vignette | Dropped | Terminal-inappropriate; documented as prototype-only |

The five emphasis roles replacing the size scale:

| Role | Design source | TUI expression |
|---|---|---|
| `Display` | 22px, `.28em` | Bold + `tracked()` + accent |
| `Title` | 14–17px | Bold, primary text |
| `Body` | 12.5px, `1.7` line-height | Default weight, body color |
| `Label` | 10–10.5px, `.18–.24em` | Dim + uppercase + `tracked()` |
| `Meta` | 10.5px faint | Dim, faint color |

Colors are the shared spine. Every hex in the prototype maps to exactly one named const — which
is what makes drift detectable rather than merely discouraged.

### 4.3 Token groups

**Surfaces** — `ROOT #06070a`, `PANEL #080a0e`, `RAIL #07090c`, `RAISED #0b0f14`,
`BAR #080b0f`, `MODAL #0a0d12`, `HAIRLINE #161b22`, `BORDER_DIM #1b2027`

**Text ramp** — `PRIMARY #e8edf1`, `BODY #c3c9cf`, `MUTED #8a949c`, `DIM #566069`,
`FAINT #3d454c`, `GHOST #2f363d`

**Semantic** — `ACCENT #5fd3e3`, `WARN #e0a95c`, `OK #74cfa4`, `ALERT #e0674f`,
`VIOLET #c98ce0`

**Field depth** — rail: near `#5fd3e3` / mid `#3f7f8a` / far `#2a3138`;
graph: near `#5fd3e3` / mid `#3a6f79` / far `#242a30`

**Diff** — add fg `#a8dfc6`, gutter `#2f6b53`; del fg `#e0a396`, gutter `#7a3a30`; row tints
pre-composited from `rgba(116,207,164,.09)` and `rgba(224,103,79,.09)` against `ROOT`

**Motion** — `BLINK 1s` step-end (cursor), `SWEEP 7s` linear (rail scan), `PULSE_IDLE 3.4s`,
`PULSE_REC 1.4s`

**Geometry** — rail field 46×32 cells, graph field 108×32, braille base `U+2800`, default
density 2300 points (range 300–2600)

### 4.4 Card groups

Each card's first line carries `<!-- @dsCard group="…" -->` so the pane indexes it without
explicit registration.

**The library grows with the code.** Foundation cards are derivable from tokens alone and ship
first; component cards require the component to exist, so they land with the phase that builds
it. This is what "continuous design sync" means in practice — the design system is never ahead
of the binary.

| Group | Contents | Lands in |
|---|---|---|
| **Color** | Surface ramp, text ramp, semantic accents, field depth, diff | P1 |
| **Type** | Emphasis roles, tracking demonstration | P1 |
| **Motion** | Timing constants with their design sources | P1 |
| **Geometry** | Field dimensions, braille cell math, density range | P1 |
| **Field** | Six modes as static rendered frames | P2 |
| **Chrome** | Header, status bar, rails | P2 |
| **States** | The six real application states | P3 |
| **Consent** | Modal + blast radius | P4 |

### 4.5 The loop

Per change: edit `tokens.rs` → `cargo run -p kol-theme --bin gen_cards` → review the diff →
`finalize_plan` → `write_files`. CI regenerates and diffs; uncommitted drift fails the build.

The source design project (`e46edf53…`) is `PROJECT_TYPE_PROJECT`. That type is immutable, so it
cannot be a sync target. It stays a read-only reference; a new **"Kol Design System"** project of
type `PROJECT_TYPE_DESIGN_SYSTEM` receives the cards.

---

## 5. Components

### 5.1 `kol-theme`

Tokens, emphasis roles, the `tracked()` helper, and `gen_cards`. Depends on `ratatui` for
`Color` only. No dependency on any `xai-grok-*` crate.

### 5.2 `kol-field`

The braille point-field widget, absorbing `views/cortana_visualizer.rs`. Public surface:

```rust
pub enum FieldMode { Converge, Ambient, Voice, Data, Graph, Alert }
pub struct Field { /* cached point sets, phase */ }
impl Field {
    pub fn new(density: usize) -> Self;
    pub fn render(&mut self, mode: FieldMode, energy: f32, elapsed: Duration, area: Rect, buf: &mut Buffer);
}
```

Depends on `ratatui` and `kol-theme`. **No pager dependency** — it receives a mode and a scalar,
and knows nothing about sessions, turns, or voice. This boundary is what makes it snapshot-
testable and what keeps the pager seam to one line.

`CortanaState` is renamed to `FieldMode` and extended from four variants to six as part of the
migration.

### 5.3 `kol-duplex`

Playback and barge-in, as a state machine with no audio inside it:

```
Idle ──speak()──> Speaking ──onset──> Cancelling ──> Capturing ──> Idle
                     │                                    │
                     └────────── playback ends ───────────┘
```

Audio enters as an energy scalar per 20ms frame, which is what makes the whole component
testable without hardware.

**Barge-in (approach A, energy-gated).** The mic stays open during playback purely as a trigger.
The gate threshold is `playback_envelope + margin` — we know the envelope because we are the
ones playing it. Onset requires 3 consecutive frames above the gate (one frame is a click; three
is a voice).

**Latency budget, 120ms total:** onset detection ≤60ms (3 × 20ms frames), cancellation ≤60ms
(stop playback, drain buffer, open capture).

**Config:** `duplex.enabled` (bool), `duplex.barge_in` (`"voice"` | `"key"`),
`duplex.gate_margin_db` (float). The `"key"` value degrades to Esc/`Ctrl+Space` interruption for
users on open speakers.

**Known regression.** `xai-grok-voice` opens the microphone in a short-lived subprocess
specifically so the long-lived TUI never pays the platform audio stack's permanent memory cost.
Barge-in requires the mic open *during playback*, so that subprocess stays resident while duplex
is active. This is the price of the feature. It applies only while `duplex.enabled`, and it is
recorded here as a deliberate trade rather than discovered later as a leak.

**Deferred:** true full-duplex with acoustic echo cancellation
(`webrtc-audio-processing`). It is the correct eventual answer for open speakers, and it is a
v2 swap behind this same interface — not a v1 prerequisite.

### 5.4 Consent and blast radius

An enrichment of the existing permission prompt, not a replacement. Before a destructive or
out-of-policy call, four facts:

| Field | Example from the design |
|---|---|
| removes | `41 files · 182 MB · not in git` |
| writes | `tests/fixtures/audio/ (regenerated)` |
| network | `yes · api.kol.dev` |
| reversible | `no` |

Plus Kol's own read on it, which is where the character lives — and where it does real work.
The four fields above are legible to a programmer who can evaluate them independently. Kol's
sentence is what makes the same prompt safe for a user who cannot (§1.2). It must state the
consequence in plain terms and volunteer the mitigation, as the design copy does: *"Those
fixtures aren't tracked, so if the regeneration fails you won't get them back. I'd suggest a
checkpoint first. Shall I?"*

This is the **candid** property from §1.3 doing safety work rather than tone work. A consent
prompt that reads as a formality is a failed consent prompt.

Two literal requirements, both easy to lose:

- **The mic is muted while a consent prompt is open.** Consent is not voice-answerable, by
  design. A destructive action must not be approvable by something Kol mishears.
- **The field enters `Alert` and stays there** until the prompt resolves.

### 5.5 Pager seams

| File | Change |
|---|---|
| `views/mod.rs` | +1 line — register the field widget |
| `app/event_loop.rs` | ~4 lines — drive field mode + energy per frame |
| `voice/mod.rs` | duplex handle wiring |
| `settings/defs.rs` | duplex config entries |

Anything larger than wiring belongs in a `kol-*` crate.

---

## 6. Testing

| Surface | Approach |
|---|---|
| Field | Seeded LCG makes point sets deterministic; snapshot-test rendered braille per mode |
| Duplex | Feed synthetic energy sequences; assert onset→cancel latency in frames. No audio hardware in CI |
| Tokens | `gen_cards` output committed; CI regenerates and diffs — drift fails the build |
| Dispatch | Voice actions follow the existing `dispatch/tests/voice.rs` pattern (sync, no tokio) |

The `dispatch/` invariants hold: no terminal, network, or filesystem access; async described as
`Effect` values. Duplex state transitions are dispatch-testable because `kol-duplex` takes
scalars, not devices.

---

## 7. Phasing

**P1 — Theme + design system.** `kol-theme`, `gen_cards`, create the "Kol Design System"
project, push the initial card set. Zero runtime risk, unblocks everything downstream.

**P2 — Field.** `kol-field` with all six modes; migrate `cortana_visualizer.rs` out of the
pager; wire mode + energy through the event loop.

**P3 — Duplex.** `kol-duplex` state machine, barge-in, config, TTS playback integration.

**P4 — Consent.** Blast-radius metadata, mic mute during prompt, `Alert` field state.

**Out of scope:** ambient/wake-word listening, crate-closure rename, AEC.

---

## 8. Open questions

None blocking. Two to revisit after P3:

- Whether `duplex.gate_margin_db` needs per-device calibration or a single default suffices.
- Whether the graph field (`Ctrl+G`) needs real codebase-graph data in v1 or can render the
  existing `xai-codebase-graph` module counts.
