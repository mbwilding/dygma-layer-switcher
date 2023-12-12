# Dygma Layer Switcher

## Config

On the first run of `dygma-layer-switcher` the `config.yml` will be generated.

```yaml
---
# Toggle logging.
logging: false

# Port of the keyboard for RF and USB, can be seen when selecting the keyboard in bazecor.
comm_port: "COM4"

# Layer to return to when no matching window is found.
base_layer: 1

# The mappings, each mapping has a layer and a choice of `window`, `process`, and `parent`, ordered in respect to performance.
# The settings aren't mutually exclusive nor are the layers.
# All setting's values are case insensitive.

# `window` is part of the title of the window.
# `process` is part of the name of the process.
# `parent` has a `process` and an optional `excludes` (array) section.
mappings:
  # Using `window` only.
  - layer: 5
    window: "Maya"
  # Using `process` only.
  - layer: 2
    process: "some_thing.exe"
  # Using `window` and `process` together.
  - layer: 3
    window: "Parsec"
    process: "parsecd.exe"
  # Using `parent` only.
  - layer: 4
    parent:
      process: "EpicGamesLauncher.exe"
  # Using `parent` only, with excludes.
  - layer: 4
    parent:
      process: "steam.exe"
      excludes:
        - "blender.exe"
        - "Quake_x64_steam.exe"
        - "bg3.exe"
        - "bg3_dx11.exe"

```
