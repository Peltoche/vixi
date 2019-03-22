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
extern crate ansi_term;
#[macro_use]
extern crate lazy_static;

mod cli;
mod controller;
mod event_handler;
mod logging;

use std::io::{BufRead, BufReader};
use std::panic;
use std::process::{exit, ChildStderr, Command, Stdio};
use std::sync::{Once, ONCE_INIT};
use std::thread;

use controller::config_map::DEFAULT_CONFIG_MAP;
use controller::Controller;
use event_handler::EventHandler;

use ncurses::*;
use xi_rpc::RpcLoop;

static HANDLER: Once = ONCE_INIT;

fn install_custom_panic_handler() {
    HANDLER.call_once(|| {
        let default_handler = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            // Clean the terminal.
            endwin();

            // Run the default panic handler.
            default_handler(info);

            // Exit with the status '1'.
            exit(1);
        }));
    })
}

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

    let mut controller = Controller::default();
    let mut event_handler = EventHandler::default();
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

    /* Setup ncurses. */
    initscr();
    raw();
    keypad(stdscr(), true); // Allow for extended keyboard (like F1).
    noecho();

    install_custom_panic_handler();

    controller.open_file(Box::new(raw_peer.clone()), file_path);
    controller.start_keyboard_event_loop(Box::new(raw_peer), &DEFAULT_CONFIG_MAP);

    endwin();
}
