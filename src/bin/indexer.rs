use std::env;
use anyhow::{Context, Result};
use serde_json::Value;

use minigrep::indexer::fetch_block_results;
use minigrep::indexer::parser::extract_wasm_events;

fn main() -> Result<()> {
    // Uso esperado:
    // cargo run --bin indexer -- <RPC_ENDPOINT> <ALTURA_INICIAL> [ALTURA_FINAL]
    //
    // Exemplo:
    // cargo run --bin indexer -- https://rpc.xion-testnet-2.burnt.com:443 9436125 9436181

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "uso: {} <rpc_endpoint> <altura_inicial> [altura_final]\n\
             ex.: {} https://rpc.xion-testnet-2.burnt.com:443 9436125 9436181",
            args.get(0).map(String::as_str).unwrap_or("indexer"),
            args.get(0).map(String::as_str).unwrap_or("indexer"),
        );
        std::process::exit(2);
    }

    let endpoint = &args[1];
    let start_height: u64 = args[2]
        .parse()
        .context("altura_inicial deve ser número inteiro")?;
    let end_height: u64 = if args.len() >= 4 {
        args[3].parse().context("altura_final deve ser número inteiro")?
    } else {
        start_height
    };

    println!("🔎 Buscando blocos de {} até {}...\n", start_height, end_height);

    for height in start_height..=end_height {
        match process_block(endpoint, height) {
            Ok(_) => (),
            Err(e) => eprintln!("⚠️  Erro ao processar bloco {}: {}\n", height, e),
        }
    }

    println!("✅ Finalizado intervalo {} - {}", start_height, end_height);
    Ok(())
}

fn process_block(endpoint: &str, height: u64) -> Result<()> {
    let json: Value = fetch_block_results(endpoint, height)?;
    let eventos = extract_wasm_events(&json);

    if eventos.is_empty() {
        println!("Bloco {} → nenhum evento wasm encontrado.", height);
        return Ok(());
    }

    println!("🧩 Bloco {} → {} evento(s) wasm encontrados:", height, eventos.len());
    for (tipo, attrs) in &eventos {
        println!("→ tipo: {}", tipo);
        for (k, v) in attrs {
            println!("   {} = {}", k, v);
        }
        println!();
    }

    Ok(())
}
