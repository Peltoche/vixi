#[macro_use]
extern crate serde_json;
extern crate ncurses;
extern crate xi_core_lib;
extern crate xi_rpc;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate fern;
#[macro_use]
extern crate clap;

mod cli;
mod event;
mod logging;
mod terminal;

use std::io::BufReader;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use terminal::Terminal;

use ncurses::*;
use xi_rpc::RpcLoop;

fn main() {
    let matches = cli::build().get_matches();

    let logging_path = Path::new("/home/peltoche/.local/share/vixy/vixi.log");
    if let Err(e) = logging::setup(logging_path) {
        eprintln!(
            "[ERROR] setup_logging returned error, logging not enabled: {:?}",
            e
        );
    }

    // spawn the core core_process
    let core_process = Command::new("xi-core")
        //.arg("test-file")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .env("RUST_BACKTRACE", "1")
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute core: {}", e));

    // Create the RpcLoop and give him access to the core via the core process
    // stdin.
    let stdin = core_process.stdin.unwrap();
    let mut rpc_loop = RpcLoop::new(stdin);

    let server = rpc_loop.get_peer();

    // Start a thread used to consume the events from the core process.
    let mut server_event_handler = event::server::Handler::new();
    let stdout = core_process.stdout.unwrap();
    thread::spawn(move || {
        rpc_loop.mainloop(|| BufReader::new(stdout), &mut server_event_handler);
    });

    // Setup the terminal.
    let terminal = Terminal::new();

    let params = json!({});
    server.send_rpc_notification("client_started", &params);

    // Create the loop handling the keyboard events.
    let _keyboard_event_handler = event::keyboard::Handler::new();
    loop {
        let ch = getch();

        if ch == KEY_F1 {
            break;
        }
    }

    terminal.clean();
}
