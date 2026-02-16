use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = minigrep::Config::new(&args).unwrap_or_else(|err| {
        // eprintln!("Problem parsing arguments: {}", err);
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    match minigrep::run(cfg) {
        Ok(_) => {}
        Err(e) => {
            // eprintln!("Application error: {}", e);
            println!("Application error: {}", e);
        }
    }
}

