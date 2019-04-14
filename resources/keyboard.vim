" This is a sample of vixi keyboard configuration.
"
" It redefine all the default configurations with the same values. This
" content should be set into:
" - Linux => /etc/vixi/keyboard.vim or ~/.config/vixi/keyboard.vim
" - Windows => C:\Users\Alice\AppData\vixi\keyboard.vim
" - MacOS => /Users/Alice/Library/Preferences/vixi/keyboard.vim
"
"
" The configuration format is: {cmd} {lhs} {rhs}
" where:
" {cmd}  is one of 'map', 'nmap', 'vmap', 'imap'
" {lhs}  left hand side, is a sequence of one or more keys that you will use
"        in your new shortcut.
" {rhs}  right hand side, is the sequence of keys that the {lhs} shortcut keys
"        will execute when entered. A shortcut starting with ':' indicate a
"        command.
"
"
" {cmd} defines the mode in which you keymap will be available:
"
" | Command |      Modes     |
" |:-------:|:--------------:|
" |   map   | Normal, Visual |
" |   nmap  |     Normal     |
" |   vmap  |     Visual     |
" |   imap  |     Insert     |


" Leader key definition
set leader <Space>

" List of key available the Normal and Visual mode
map <Esc> :switch_to_normal_mode
map <Up> :move_up
map <Down> :move_down
map <Left> :move_left
map <Right> :move_right
map <PageUp> :page_up
map <PageDown> :page_down
map <Leader>q :quit
map <Leader>w :write_to_file

" Normal mode
nmap h :move_left
nmap j :move_down
nmap k :move_up
nmap l :move_right
nmap i :switch_to_insert_mode
nmap v :switch_to_visual_mode
nmap p :paste
nmap o :open_line_below
nmap O :open_line_above

" Visual mode
vmap <Esc> :switch_to_normal_mode
vmap y :yank_selection
vmap d :delete_selection
vmap p :past

" Insert mode mode
imap <Del> :delete_forward
imap <BS> :delete_backward
imap <Esc> :switch_to_normal_mode
imap <Up> :move_up
imap <Down> :move_down
imap <Left> :move_left
imap <Right> :move_right
imap <PageUp> :page_up
imap <PageDown> :page_down
