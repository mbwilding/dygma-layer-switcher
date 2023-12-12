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

# `window` is part of the title of the window, case insensitive.
# `process` is part of the name of the process, case insensitive.
# `parent` is part of the name of any parent process, case insensitive.
mappings:
  - layer: 5
    window: "Blender"
    process: "blender.exe"
  - layer: 5
    window: "Maya"
  - layer: 2
    process: "some_thing.exe"
  - layer: 4
    parent: "steam.exe"

```
