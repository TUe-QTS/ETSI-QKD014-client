extern crate cbindgen;

use cbindgen::Config;
use std::env;

fn main() {
    if env::var("E14_NO_BINDINGS").is_ok() {
        return;
    }
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(Config::from_file("./c/cbindgen.toml").unwrap())
        .generate()
        .map_or_else(
            |error| match error {
                cbindgen::Error::ParseSyntaxError { .. } => {}
                e => panic!("{:?}", e),
            },
            |bindings| {
                bindings.write_to_file("./c/etsi014-client.h");
            },
        );
}
