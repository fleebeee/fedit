# fedit

`fedit` is a simple terminal-based text editor written in Rust with convenience in mind. It tries to work around the limitations of the terminal and maintain familiar controls from GUI editors.

## Installation

```sh
git clone git@github.com:fleebeee/fedit.git
cd fedit
# to just build the binary
cargo build --release
# or to install it with cargo
cargo install --path .
```

## Compatibility

To enjoy all the features of `fedit` you need a terminal emulator that implements the [kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/). I've found that some terminals don't render graphemes such as 👍🏻 correctly. `fedit` has been developed and tested on [WezTerm](https://wezterm.org/).

## Controls

| Key | Action |
|-----|--------|
| Ctrl+Q | Quit |
| Ctrl+S | Save file |
| Ctrl+Z | Undo |
| Ctrl+Y | Redo |
| Ctrl+C | Copy |
| Ctrl+V | Paste |
| ↑↓←→ | Move cursor (Modifiers: Shift, Super) |
| Left mouse | Move cursor |

## Performance

`fedit` is not blazing fast. It's intended for quick edits on small files.

Text is internally handled as a list of lines, and lines are lists of graphemes. There is no gap buffer.
