use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde_json::Value;
use std::{thread, time::Duration};

fn main() -> Result<()> {
    // 1️⃣ Cria/abre o banco de dados
    let conn = Connection::open("events.db")?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS events (
            height TEXT,
            txhash TEXT PRIMARY KEY,
            contract_address TEXT,
            action TEXT,
            sender TEXT,
            recipient TEXT,
            amount TEXT,
            timestamp TEXT
        );",
    )?;

    println!("🚀 Indexador iniciado — buscando últimas transações CosmWasm...\n");

    // 2️⃣ Buscar últimas transações
    let url = "https://api.xion-testnet-2.burnt.com/cosmos/tx/v1beta1/txs?limit=10";
    let resp = reqwest::blocking::get(url)
        .with_context(|| format!("Falha ao requisitar {}", url))?;

    if !resp.status().is_success() {
        anyhow::bail!("Status HTTP não-sucesso: {}", resp.status());
    }

    let json: Value = resp.json().context("Erro ao decodificar JSON")?;
    let Some(txs) = json["tx_responses"].as_array() else {
        println!("⚠️ Nenhuma transação encontrada.");
        return Ok(());
    };

    // 3️⃣ Itera sobre transações e extrai os hashes
    for tx in txs {
        if let Some(hash) = tx["txhash"].as_str() {
            // 4️⃣ Busca os detalhes da transação individual
            if let Err(e) = process_tx(&conn, hash) {
                eprintln!("Erro processando {}: {}", hash, e);
            }

            // pequeno intervalo entre requisições pra não sobrecarregar o endpoint
            thread::sleep(Duration::from_millis(500));
        }
    }

    println!("\n✅ Indexação finalizada!");
    Ok(())
}

fn process_tx(conn: &Connection, hash: &str) -> Result<()> {
    let url = format!(
        "https://api.xion-testnet-2.burnt.com/cosmos/tx/v1beta1/txs/{}",
        hash
    );

    let resp = reqwest::blocking::get(&url)?;
    if !resp.status().is_success() {
        anyhow::bail!("Status HTTP {} para {}", resp.status(), hash);
    }

    let json: Value = resp.json().context("Erro ao interpretar JSON")?;
    let txr = &json["tx_response"];

    let height = txr["height"].as_str().unwrap_or_default();
    let timestamp = txr["timestamp"].as_str().unwrap_or_default();

    // Extrai eventos do tipo wasm
    let empty_events = Vec::new();
let events = txr["events"].as_array().unwrap_or(&empty_events);

    for ev in events {
        if ev["type"] != "wasm" {
            continue;
        }
        let empty_attrs = Vec::new();
        let attrs = ev["attributes"].as_array().unwrap_or(&empty_attrs);


        let mut contract_address = String::new();
        let mut action = String::new();
        let mut sender = String::new();
        let mut recipient = String::new();
        let mut amount = String::new();

        for attr in attrs {
            match attr["key"].as_str().unwrap_or_default() {
                "_contract_address" => contract_address = attr["value"].as_str().unwrap_or_default().to_string(),
                "action" => action = attr["value"].as_str().unwrap_or_default().to_string(),
                "from" => sender = attr["value"].as_str().unwrap_or_default().to_string(),
                "to" => recipient = attr["value"].as_str().unwrap_or_default().to_string(),
                "amount" => amount = attr["value"].as_str().unwrap_or_default().to_string(),
                _ => {}
            }
        }

        if !contract_address.is_empty() {
            conn.execute(
                "INSERT OR REPLACE INTO events 
                 (height, txhash, contract_address, action, sender, recipient, amount, timestamp)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
                params![
                    height,
                    hash,
                    contract_address,
                    action,
                    sender,
                    recipient,
                    amount,
                    timestamp
                ],
            )?;
            println!("💾 Salvo evento wasm de {}", contract_address);
        }
    }

    Ok(())
}
