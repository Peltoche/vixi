use std::io::{self, BufRead, Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use serde_json::Value;
use xi_core_lib::XiCore;
use xi_rpc::RpcLoop;

/// Wraps an instance of `mpsc::Sender`, implementing `Write`.
///
/// This lets the tx side of an mpsc::channel serve as the destination
/// stream for an RPC loop.
pub struct Writer(Sender<String>);

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = String::from_utf8(buf.to_vec()).unwrap();
        self.0
            .send(s)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))
            .map(|_| buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Wraps an instance of `mpsc::Receiver`, providing convenience methods
/// for parsing received messages.
pub struct Reader(Receiver<String>);

impl Read for Reader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        unreachable!("didn't expect xi-rpc to call read");
    }
}

// Note: we don't properly implement BufRead, only the stylized call patterns
// used by xi-rpc.
impl BufRead for Reader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        unreachable!("didn't expect xi-rpc to call fill_buf");
    }

    fn consume(&mut self, _amt: usize) {
        unreachable!("didn't expect xi-rpc to call consume");
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        match self.0.recv() {
            Ok(s) => {
                buf.push_str(&s);
                Ok(s.len())
            }
            Err(_) => Ok(0),
        }
    }
}

pub struct ClientToClientWriter(Writer);

impl ClientToClientWriter {
    pub fn send_rpc_notification(&mut self, method: &str, params: &Value) {
        let raw_content = match serde_json::to_vec(&json!({"method": method, "params": params})) {
            Ok(raw) => raw,
            Err(err) => {
                error!("failed to create the notification {}: {}", method, err);
                return;
            }
        };

        match self.0.write(&raw_content) {
            Ok(_) => (),
            Err(err) => error!("failed to send the notification {}: {}", method, err),
        };
    }
}

pub fn start_xi_core() -> (Writer, Reader, ClientToClientWriter) {
    let mut core = XiCore::new();

    let (to_core_tx, to_core_rx) = channel();
    let client_to_core_writer = Writer(to_core_tx);
    let client_to_core_reader = Reader(to_core_rx);

    let (from_core_tx, from_core_rx) = channel();
    let core_to_client_writer = Writer(from_core_tx.clone());
    let core_to_client_reader = Reader(from_core_rx);

    let client_to_client_writer = ClientToClientWriter(Writer(from_core_tx));

    let mut core_event_loop = RpcLoop::new(core_to_client_writer);
    thread::spawn(move || core_event_loop.mainloop(|| client_to_core_reader, &mut core));

    (
        client_to_core_writer,
        core_to_client_reader,
        client_to_client_writer,
    )
}
