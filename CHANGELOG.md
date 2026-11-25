# Changelog hazard3-ttp223b

All notable changes to this project are documented here.
This project follows a pragmatic development philosophy: each update aims to
add functionality while minimizing the microcontroller's memory footprint.

---

## [0.2.0]

### Changed
- Renamed the package `embassy-ttp223b` to `hazard3-ttp223b` and the
  library `embassy_ttp223b` to `hazard3_ttp223b`.
- The default time backend is `hazard3-time` via `time-hazard3`.
  The official Embassy backend remains available via `time-upstream` with
  default features disabled; the two options are mutually exclusive.
- Documented migration and runtime configuration.

---

## [0.1.0] — 2026

### Added
- Initial implementation of the asynchronous TTP223B touch sensor driver.
- Configurable 50 ms software debounce to eliminate false positives.
- `Ttp223::new(pin)`: constructor with default timing.
- `Ttp223::new_with_timing(pin, debounce_ms, poll_ms)`: customizable timing.
- `wait_for_change()`: asynchronously wait for a confirmed state change.
- `get_state()`: read the stored state without waiting.
- `is_pressed()`: Boolean shortcut.
- `release()`: release the GPIO resource.
- Public types `TouchState` (Pressed / Released) and `TouchError` (ReadError / SignalUnstable).
- No allocation or `unsafe` code; compatible with `no_std`.