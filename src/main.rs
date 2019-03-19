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
extern crate dirs;

mod cli;
mod event;
mod logging;
mod terminal;

use std::io::BufReader;
use std::process::{Command, Stdio};
use std::thread;

use terminal::Terminal;

use xi_rpc::RpcLoop;

fn setup_logger() {
    let logging_path = dirs::home_dir()
        .expect("failed to retrieve the home dir")
        .join(".local/share/vixy/vixi.log");

    logging::setup(&logging_path).expect("failed to set the logger")
}

fn main() {
    let matches = cli::build().get_matches();

    let inputs = matches
        .value_of("file")
        .expect("failed to retrieve cli value");

    setup_logger();

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

    event::start_keyboard_event_loop();

    terminal.clean();
}
