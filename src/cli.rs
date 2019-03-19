use clap::{App, Arg};

pub fn build() -> App<'static, 'static> {
    App::new("vixi")
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file"))
}
