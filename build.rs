use std::{env, fs, path::Path};

use const_units_code_gen::generate;
static DATA_DIR: &str = "data";

fn main() {
    println!("cargo:rerun-if-changed={}/data", DATA_DIR);
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");

    let code = &format!("{}", generate());
    let code_file = syn::parse_file(code).unwrap();

    fs::write(&dest_path, prettyplease::unparse(&code_file)).unwrap();
    // for system in fs::read_dir(data_folder).expect("cannot read datafolder: {}", data_folder) {}
}
