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
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate termion;
extern crate toml;

mod cli;
mod core;
mod event_controller;
mod input_controller;
mod logging;

use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::rc::Rc;
use std::thread;
use std::time;

use event_controller::style::TermionStyles;
use event_controller::window::TermionLayout;
use event_controller::{EventController, Styles};
use input_controller::keyboard::TermionKeyboard;
use input_controller::{Config, InputController};

use failure::Error;
use xi_rpc::{Peer, RpcLoop};

fn setup_logger() {
    let logging_path = dirs::home_dir()
        .expect("failed to retrieve the home dir")
        .join(".local/share/vixy/vixi.log");

    logging::setup(&logging_path).expect("failed to set the logger")
}

fn setup_config(core: &dyn Peer) -> Result<Config, Error> {
    let mut xi_config_dir =
        dirs::config_dir().ok_or_else(|| format_err!("config dir not found"))?;
    xi_config_dir.push("xi");

    let mut vixi_config_file = xi_config_dir.clone();
    vixi_config_file.push("vixi.toml");

    let mut file = File::open(&vixi_config_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;

    core.send_rpc_notification(
        "client_started",
        &json!({ "config_dir": xi_config_dir.to_str().unwrap(), }),
    );

    Ok(config)
}

fn main() {
    let matches = cli::build().get_matches();

    let file_path = matches
        .value_of("file")
        .expect("failed to retrieve cli value");

    setup_logger();

    let (client_to_core_writer, core_to_client_reader, client_to_client_writer) =
        core::start_xi_core();
    let mut front_event_loop = RpcLoop::new(client_to_core_writer);

    let raw_peer = front_event_loop.get_raw_peer();
    thread::spawn(move || {
        let layout = TermionLayout::new();

        let styles: Rc<RefCell<Box<dyn Styles>>> = Rc::new(RefCell::new(Box::new(
            TermionStyles::new(layout.get_writer()),
        )));

        let mut event_handler = EventController::new(Box::new(layout), styles.clone());
        front_event_loop
            .mainloop(|| core_to_client_reader, &mut event_handler)
            .unwrap();
    });

    let exit_res = setup_config(&raw_peer)
        .and_then(|config| {
            Ok(InputController::new(
                Box::new(TermionKeyboard::default()),
                client_to_client_writer,
                &config,
            ))
        })
        .and_then(|mut controller| {
            controller.open_file(&raw_peer, file_path)?;
            controller.start_keyboard_event_loop(&raw_peer)
        });

    thread::sleep(time::Duration::from_millis(50));

    match exit_res {
        Ok(_) => exit(0),
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
}
