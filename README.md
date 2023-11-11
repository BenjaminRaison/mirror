# mirror
A simple utility for sway / wl-mirror to automatically set up display mirroring in a seemless manner.


## Usage
`mirror`

When only one display is connected, nothing happens.

For two or more, this will:
* mirror eDP-1, or the active output if not present, using `wl-mirror`
* move wl-mirror to an inactive output (HDMI-1 if available)

If mirroring is already happening, executing `mirror` will kill it.


Sample keybinding in sway config:
```
bindsym $mod+p exec mirror

```

