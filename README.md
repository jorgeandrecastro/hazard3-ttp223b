# hazard3-ttp223b

An asynchronous `no_std` GPIO driver for the TTP223B touch sensor, part of the
independent [Hazard3 Async](https://github.com/jorgeandrecastro/hazard3-async)
project for the RP2350 Hazard3 RISC-V cores. It uses `embedded-hal` 1.0,
with no allocation or `unsafe` code.

[![Crates.io](https://img.shields.io/crates/v/hazard3-ttp223b.svg)](https://crates.io/crates/hazard3-ttp223b)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![docs.rs](https://docs.rs/hazard3-ttp223b/badge.svg)](https://docs.rs/hazard3-ttp223b)


- Software filtering of state changes: 50 ms debounce and 10 ms polling
  by default, configurable with `new_with_timing`.
- Read the confirmed state with `get_state` or `is_pressed`.
- Release the GPIO with `release`.

## Installation

```toml
[dependencies]
hazard3-ttp223b = "0.3.0"
```

The `time-hazard3` feature is enabled by default and uses `hazard3-time`.
The application must provide a compatible time driver, executor, and HAL
configuration. Use the `hazard3-*` runtime crates together without mixing
their time drivers with upstream Embassy:
see the [Hazard3 Async guide](https://github.com/jorgeandrecastro/hazard3-async#quick-start).

### Alternative: upstream Embassy

To use the official Embassy time backend instead of Hazard3:

```toml
[dependencies]
hazard3-ttp223b = { version = "0.3.0", default-features = false, features = ["time-upstream"] }
```

This configuration requires a compatible Embassy time driver and runtime.
Exactly one of the `time-hazard3` and `time-upstream` features must be enabled.

## Migration from `embassy-ttp223b`

The former `embassy-ttp223b` package was yanked to avoid confusion with
upstream Embassy. An existing lockfile can still resolve a yanked version,
but new dependency resolution normally cannot select it. Replace the
dependency and Rust import:

```rust
use hazard3_ttp223b::{Ttp223, TouchState};
```

The driver's types and methods remain unchanged. The default time backend is
now `hazard3-time`; to keep Embassy, use the `time-upstream` configuration above.

## Usage

Configure the GPIO as an input using your HAL. A high signal indicates a
touch; the initial stored state is `Released` until a change is confirmed.

```rust,no_run
use embedded_hal::digital::InputPin;
use hazard3_ttp223b::{Ttp223, TouchState};

async fn watch<P: InputPin>(pin: P) {
    let mut touch = Ttp223::new(pin);
    loop {
        match touch.wait_for_change().await {
            Ok(TouchState::Pressed) => { /* touch */ }
            Ok(TouchState::Released) => { /* release */ }
            Err(_) => { /* GPIO read error */ }
        }
    }
}
```

## 🛡️ License

Licensed under either of:

    MIT license (http://opensource.org/licenses/MIT)

    Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## 🦅 About

Developed by Jorge Andre Castro.
