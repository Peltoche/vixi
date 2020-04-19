# Vixi

A "Vim like" frontend for [xi-editor](https://github.com/xi-editor/xi-editor).

## Warning

This frontend is under heavy development. A lot of stuff can break and change.


## The "Vim like" approach

The idea behind Vixi is to use the powerful keybinding of Vim without keeping
all the complexities and inconsistencies. In other words, keeping the 20% of
Vim used by 80% of users.


## Configuration

All the keybinding presented below can be overriden via the `keyboard.toml`
configuration file. You can find an [example of keyboard.toml config](./resources/keyboard.toml)
which will override all the default values with the exact same values.

Those configuration files should live inside a `vixi` folder placed into the
OS specific config directory.

|  **OS** |     **Vixi configuration folder**     |
|:-------:|:-------------------------------------:|
|  Linux  |        /home/alice/.config/vixi       |
| Windows |      C:\Users\Alice\AppData\vixi      |
|  MacOS  | /Users/Alice/Library/Preferences/vixi |


## How to use it

### Modes

Vixi behavior is split in "modes" (like Vim). Those are:
- **Normal** (the default one) used to move and manipulate stuff
- **Insert** used to insert stuff
- **Visual** used to select stuff and manipulate it
- **Action** used to make generic action (save/quit/etc)

At any time you can return to the **Normal** mode by pressing the `Esc` key.
You can note that the **Action** mode doesn't exists on Vim. This somewhat
corresponds to the Vim `mapLeader` key.


#### Normal mode

The Normal mode is the default one. It is mainly used to move the cursor around
and switch into the other modes.

|  **Key** |               **Description**               |
|:--------:|:-------------------------------------------:|
|   ←↑→↓   |               Move the cursor               |
|   hjkl   |               Move the cursor               |
|     w    |       Move the cursor to the next word      |
|     W    |     Move the cursor to the previous word    |
|  \<PgUp> |        Move the cursor one page above       |
|  \<PgDn> |        Move the cursor one page below       |
|     v    |            Switch to Visual Mode            |
|     i    |            Switch to Insert Mode            |
| \<Space> |            Switch to Action Mode            |
|     o    | Insert line below and switch to Insert Mode |
|     O    | Insert line above and switch to Insert Mode |
|     p    |       Paste the previously cut content      |
|     x    |             Delete text forward             |
|     X    |            Delete text backwards            |
|     q    |                    Quit                     |
|     >    |                   Indent                    |
|     <    |                  Outdent                    |


#### Insert mode

The Insert mode is used to insert some content.

|    **Key**   |         **Description**        |
|:------------:|:------------------------------:|
|     ←↑→↓     |         Move the cursor        |
|    \<PgUp>   | Move the cursor one page above |
|    \<PgDn>   | Move the cursor one page below |
|    \<Esc>    |    Switch to the normal mode   |
| \<Backspace> |   Remove a character backward  |
|    \<Del>    |   Remove a character forward   |


#### Visual mode

The Visual mode is used to select some text and manipulate it.

|  **Key** |                   **Description**                   |
|:--------:|:---------------------------------------------------:|
|   ←↑→↓   |                   Move the cursor                   |
|   hjkl   |                   Move the cursor                   |
|     w    |           Move the cursor to the next word          |
|     W    |         Move the cursor to the previous word        |
|  \<Esc>  |                Switch to Normal Mode                |
|     q    |                Switch to Normal Mode                |
| \<Space> |                Switch to Action Mode                |
|     y    |                  Yank the selection                 |
|     d    |                 Delete the selection                |
|     p    | Delete the selection and past the previous deletion |


#### Action mode

The Action mode is used to make some generic action like saving or exiting.

| **Key** |   **Description**   |
|:-------:|:-------------------:|
|    q    |         Quit        |
|    w    | Write into the file |

