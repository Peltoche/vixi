# Vixi

A "vim like" frontend for [xi-editor](https://github.com/xi-editor/xi-editor).

## Warning

This frontend is under heavy development. A lot of stuff can break and change.


## Current State

- Commands
  - Insert Mode (trigger with `i`/`o`/`O` in normal mode)
    - [x] All the basic movements (arrow/pageUp/pageDown)
    - [x] Write some stuff
  - Visual Mode (trigger with `v` in normal mode)
    - [x] All the basic movements (arrow/hjkl/pageUp/pageDown)
    - [x] Delete a region (`d`)
    - [x] Yank a region (`y`)
    - [x] Delete a region then past (`p`)
  - Normal Mode (mode by default)
    - [x] All the basic movements (arrow/hjkl/pageUp/pageDown)
    - [x] Switch to other modes
    - [x] Paste (`p`)
    - [x] Insert line below (`o`)
    - [x] Insert line above (`O`)
    - [x] Handle an action with
  - Action Mode (trigger with `<space>` in normal mode)
    - [ ] Write into a file (`w`) (WIP)
    - [x] Quite (`q`)
    - [ ] Find
- Status Bar
  - [x] Print the mode name
  - [ ] Print the file path
  - [ ] Print the cursor position
