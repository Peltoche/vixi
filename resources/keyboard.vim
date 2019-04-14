
map <Esc> switch_to_normal_mode
imap <Esc> switch_to_normal_mode

map <Up> move_up
imap <Up> move_up

map <Down> move_down
imap <Down> move_down

map <Left> move_left
imap <Left> move_left

map <Right> move_right
imap <Right> move_right

map <PageUp> page_up
imap <PageUp> page_up

map <PageDown> page_down
imap <PageDown> page_down


nmap i switch_to_insert_mode
nmap v switch_to_visual_mode

nmap p paste

nmap o open_line_below
nmap O open_line_above

nmap h move_left
nmap j move_down
nmap k move_up
nmap l move_right

vmap <Esc> switch_to_normal_mode
vmap y yank_selection
vmap d delete_selection
vmap p past

imap <Del> delete_forward
imap <BS> delete_backward
