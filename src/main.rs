use clap::Parser;
use create2crunch::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if config.gpu_device == 255 {
        if let Err(e) = create2crunch::cpu(config) {
            eprintln!("CPU application error: {e}");
            process::exit(1);
        }
    } else if let Err(e) = create2crunch::gpu(config) {
        eprintln!("GPU application error: {e}");
        process::exit(1);
    }
}
