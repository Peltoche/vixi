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
#[macro_use]
extern crate lazy_static;

mod cli;
mod devices;
mod event_controller;
mod input_controller;
mod logging;

use std::io::{BufRead, BufReader};
use std::process::{ChildStderr, Command, Stdio};
use std::thread;

use devices::keyboard::Keyboard;
use devices::terminal::Terminal;
use event_controller::EventController;
use input_controller::key_map::DEFAULT_CONFIG;
use input_controller::Controller;

use ncurses::*;
use xi_rpc::RpcLoop;

fn setup_logger() {
    let logging_path = dirs::home_dir()
        .expect("failed to retrieve the home dir")
        .join(".local/share/vixy/vixi.log");

    logging::setup(&logging_path).expect("failed to set the logger")
}

fn handle_core_stderr(stderr: ChildStderr) {
    let buf_reader = BufReader::new(stderr);
    for line in buf_reader.lines() {
        if let Ok(line) = line {
            if let Some(idx) = line.find("[INFO] ") {
                info!("[CORE] {}", line.split_at(idx + 7).1)
            } else if let Some(idx) = line.find("[WARN] ") {
                warn!("[CORE] {}", line.split_at(idx + 7).1)
            } else if let Some(idx) = line.find("[ERROR] ") {
                error!("[CORE] {}", line.split_at(idx + 8).1)
            } else {
                error!("[CORE] {}", line);
            }
        }
    }
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
        .stderr(Stdio::piped())
        .env("RUST_BACKTRACE", "1")
        .spawn()
        .unwrap_or_else(|e| panic!("failed to execute core: {}", e));

    // Create the RpcLoop and give him access to the core via the core process
    // stdin.
    let stdin = core_process.stdin.unwrap();
    let mut rpc_loop = RpcLoop::new(stdin);

    // Load the devices
    let terminal = Terminal::new();
    let keyboard = Keyboard::default();

    let mut controller = Controller::new(terminal.clone(), keyboard);
    let mut event_handler = EventController::new(terminal);
    let raw_peer = rpc_loop.get_raw_peer();

    let stderr = core_process.stderr.unwrap();
    thread::spawn(move || handle_core_stderr(stderr));

    // Start a thread used to consume the events from the core process.
    let stdout = core_process.stdout.unwrap();
    thread::spawn(move || {
        rpc_loop
            .mainloop(|| BufReader::new(stdout), &mut event_handler)
            .unwrap();
    });

    controller.open_file(&raw_peer, file_path);
    controller.start_keyboard_event_loop(&raw_peer, &DEFAULT_CONFIG);

    endwin();
}
