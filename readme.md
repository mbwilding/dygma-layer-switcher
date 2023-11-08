# Dygma Layer Switcher

## Running

### Service
```
cargo run --bin dygma-layer-switcher-service
```

### Config UI
```
cargo run --bin dygma-layer-switcher-ui
```

## Building

```
cargo build --release
```

Then check the target folder

## Config

Use the `dygma-layer-switcher-ui` to generate or edit a config file.

```yaml
# Port of the keyboard for RF and USB, can be seen when selecting the keyboard in bazecor.
comm_port: "COM5"

# Layer to return to when no matching window is found.
base_layer: 1

# The mappings, each mapping has a layer and an option `exe_name` and/or `window_title`.
mappings:
  - layer: 5
    window_title: "Blender"
  - layer: 5
    window_title: "Maya"
  - layer: 2
    exe_name: "game.exe"
```