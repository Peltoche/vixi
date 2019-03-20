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
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod cli;
mod controller;
mod event_handler;
mod logging;
mod terminal;

use std::io::BufReader;
use std::process::{Command, Stdio};
use std::thread;

use controller::Controller;
use event_handler::EventHandler;
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

    let file_path = matches
        .value_of("file")
        .expect("failed to retrieve cli value");

    setup_logger();

    // spawn the core core_process
    let core_process = Command::new("xi-core")
        //.arg("test-file")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::null())
        .env("RUST_BACKTRACE", "1")
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute core: {}", e));

    // Create the RpcLoop and give him access to the core via the core process
    // stdin.
    let stdin = core_process.stdin.unwrap();
    let mut rpc_loop = RpcLoop::new(stdin);

    let terminal = Terminal::new();
    let mut controller = Controller::default();
    let mut event_handler = EventHandler::default();
    let core_client = rpc_loop.get_peer();

    // Start a thread used to consume the events from the core process.
    let stdout = core_process.stdout.unwrap();
    thread::spawn(move || {
        rpc_loop.mainloop(|| BufReader::new(stdout), &mut event_handler);
    });

    controller.open_file(&core_client, file_path);
    controller.start_keyboard_event_loop(&core_client);

    terminal.clean();
}
