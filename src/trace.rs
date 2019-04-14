use std::fs::OpenOptions;
use std::io::Write;

use xi_trace::chrome_trace_dump;

pub fn start_tracer() {
    xi_trace::enable_tracing();
    if xi_trace::is_enabled() {
        info!("tracing started")
    }
}

pub fn write_trace_dump_into(file_path: &str) {
    let samples = xi_trace::samples_cloned_unsorted();
    let mut serialized = Vec::<u8>::new();
    chrome_trace_dump::serialize(&samples, &mut serialized).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    file.write_all(&serialized).unwrap();
}
