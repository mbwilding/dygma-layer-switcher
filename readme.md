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
# All setting values are case-insensitive.

# `window` is part of the title of the window.
# `process` is part of the name of the process.
# `parent` has a `process` and an optional `excludes` (array) section.

# You can do any combination of the above, mix and match all you want under the layer's `app` section.
mappings:
  - layer: 4
    apps:
      - parent:
          process: "EpicGamesLauncher.exe"
      - parent:
          process: "steam.exe"
          excludes:
            - "steamwebhelper.exe"
            - "blender.exe"
  - layer: 5
    apps:
      - window: "Autodesk MAYA"
      - process: "blender.exe"

```
