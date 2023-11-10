# Dygma Layer Switcher

## Config

On the first run of `dygma-layer-switcher-service` a `config.yml` will be created.

```yaml
---
# Port of the keyboard for RF and USB, can be seen when selecting the keyboard in bazecor.
comm_port: "COM5"

# Layer to return to when no matching window is found.
base_layer: 1

# The mappings, each mapping has a layer and a choice of `exe_name` and/or `window_title`.
mappings:
  - layer: 5
    window_title: "Blender"
  - layer: 5
    window_title: "Maya"
  - layer: 2
    exe_name: "some_game.exe"

```
