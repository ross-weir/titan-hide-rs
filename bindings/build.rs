use bindgen::builder;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/TitanHide.h");

    let bindings = builder()
        .header("src/TitanHide.h")
        .allowlist_type("HIDE_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate TitanHide bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write TitanHide bindings to file");
}
