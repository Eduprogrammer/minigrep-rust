use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problema ao analisar os argumentos: {}", err);
        process::exit(1);
    });

    println!("Procurando por: {}", config.query);
    println!("No arquivo: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Erro da aplicação: {}", e);
        process::exit(1);
    }
}

 