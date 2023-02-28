# Current date/time for i3blocks

## Why?
- Minimal Rust binary, hardly uses any resources
- Runs constantly, rather than needing to spawn a process every N seconds
- Only sends updates to i3blocks once per second

## Prerequisite
- Rust

## Build

```
$ cargo build --release
```

The binary will be in `target/release`. Move it wherever you want.

## Configure i3blocks

```
# Date Time
#
[time]
command=/path/to/i3blocks-current-dt
interval=persist
```
