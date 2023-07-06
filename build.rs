use log::LevelFilter;
use simple_logging;
static DATA_DIR: &str = "data";

fn main() {
    println!("cargo:rerun-if-changed={}/data", DATA_DIR);
    simple_logging::log_to_file("build.log", LevelFilter::Info).unwrap();

    // for system in fs::read_dir(data_folder).expect("cannot read datafolder: {}", data_folder) {}
}
