# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`dwf` is a Rust wrapper around the Digilent WaveForms SDK (`libdwf`), the C library that drives Analog Discovery-style USB instruments. It covers only part of the API. So far it has device open/enumerate/info, analog-in (oscilloscope) and digital-in (logic analyzer). Only macOS has been tested upstream.

## Build prerequisites

`build.rs` runs `bindgen` on `wrapper.h` (`#include <dwf.h>`) and links against the installed SDK, so nothing builds without it:

- **Linux:** headers in `/usr/include/digilent/waveforms`, links `-ldwf`
- **macOS:** `/Library/Frameworks/dwf.framework` (also sets an rpath)
- **Windows:** `C:\Program Files (x86)\Digilent\WaveFormsSDK\inc`, links `dwf.dll`

`bindgen` also needs libclang. If the SDK or libclang is missing, the build fails inside `build.rs`, before any crate code is compiled.

## Commands

```sh
cargo build
cargo test                          # unit tests call the real libdwf
cargo test check_version            # run a single test by name
cargo run --example enumerate_devices
cargo run --example analog_in_acquire   # requires a connected device
cargo fmt && cargo clippy
```

The tests in `src/lib.rs` call into the installed library, so they need the SDK at runtime. Most examples also need hardware attached. `gnuplot` is a dev-dependency used by `analog_in_acquisition`.

## Architecture

- **`src/dwf.rs`**: the raw FFI layer. It `include!`s the bindgen output (`$OUT_DIR/bindings.rs`), so every `FDwf*` function and C constant (`trigsrc*`, `acqmode*`, `dwferc*`, …) lives in `crate::dwf`. It also holds the C-bool helpers `to_c_bool`/`from_c_bool` and maps `DWFERC` to `DwfError`.
- **`src/lib.rs`**: the crate's `Error`/`Result`, plus `check_call(BOOL) -> Result<()>`. Every SDK call goes through `check_call`. When a call fails, it fetches `FDwfGetLastError` and turns it into `Error::FailWithErrorCode`. It also has the free functions `get_version` and `enumerate_devices`.
- **`src/device.rs`**: `Device` owns an `HDWF` handle. It closes the handle in `Drop` and panics if the close fails. `Device::default()` opens the first available device (`open(-1, 0)`). Instruments are borrowed views tied to the device's lifetime: `device.analog_in()` → `AnalogIn<'_>`, `analog_in.channel(ix)` → `AnalogInChannel<'_>`, `device.digital_in()` → `DigitalIn<'_>`.
- **`src/device_info.rs`**: the `DeviceId`/`DeviceInfo` traits. They wrap the `FDwfEnum*` queries, which take a device *index*, not a handle. `Device` derives the index as `handle - 1`.
- **`src/analog/analog_in.rs`, `src/digital/digital_in.rs`**: one module per instrument, behind the `analog.rs`/`digital.rs` module files (no `mod.rs`). Each module defines its own Rust enums (`TriggerSource`, `TriggerSlope`, `*AcquisitionStatus`, …) and maps them to bindgen constants with `match`. These enums are separate per module, not shared.

### Patterns for adding SDK functions

New wrappers follow a fixed shape. Copy a nearby method rather than inventing a new one:

- **Setters:** `unsafe { check_call(dwf::FDwfXxxSet(self.device.get_handle(), ...))?; } Ok(())`
- **Getters:** pass `mem::MaybeUninit::uninit()` as an out-param, call `check_call`, then return `assume_init()`.
- **Strings:** read into a fixed `[c_char; N]` buffer, then convert with `CStr::from_ptr(...).to_str()`.
- **Sample fetches (`fetch_samples`):** append to the caller's `Vec`. They `reserve`, write into `spare_capacity_mut()`, then `set_len`.
- **Enum-typed parameters:** add a Rust enum and map it to the `dwf::` constant with a `match`. Don't expose the raw constant.

Each instrument has examples in `examples/`. Add one when you add an instrument or an acquisition mode.
