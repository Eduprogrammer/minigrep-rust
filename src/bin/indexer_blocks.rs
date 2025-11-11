use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde_json::Value;
use std::{thread, time::Duration};

const API_BASE: &str = "https://api.xion-testnet-2.burnt.com";

fn main() -> Result<()> {
    println!("🚀 Indexador Xion — modo automático (últimos 50 blocos)\n");

    // 1️⃣ Banco SQLite
    let conn = Connection::open("events.db")?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS events (
            height TEXT,
            txhash TEXT,
            contract_address TEXT,
            action TEXT,
            sender TEXT,
            recipient TEXT,
            amount TEXT,
            timestamp TEXT,
            PRIMARY KEY (height, txhash)
        );",
    )?;

    // 2️⃣ Buscar altura atual
    let latest_url = format!("{}/cosmos/base/tendermint/v1beta1/blocks/latest", API_BASE);
    let latest_resp = reqwest::blocking::get(&latest_url)?;
    let latest_json: Value = latest_resp.json()?;
    let latest_height = latest_json["block"]["header"]["height"]
        .as_str()
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0);

    println!("📦 Altura mais recente: {}\n", latest_height);

    // 3️⃣ Percorre os últimos 50 blocos
    for height in (latest_height.saturating_sub(50))..=latest_height {
        let url = format!("{}/cosmos/tx/v1beta1/txs/block/{}", API_BASE, height);
        let resp = reqwest::blocking::get(&url);

        match resp {
            Ok(r) if r.status().is_success() => {
                let json: Value = r.json().unwrap_or_default();
                if let Some(txs) = json["tx_responses"].as_array() {
                    let mut wasm_found = 0;
                    for tx in txs {
                        if let Some(hash) = tx["txhash"].as_str() {
                            wasm_found += extract_and_store(&conn, tx, hash, height)?;
                        }
                    }

                    if wasm_found > 0 {
                        println!("✅ Bloco {} → {} evento(s) wasm encontrado(s)", height, wasm_found);
                    } else {
                        println!("⬜ Bloco {} → nenhum evento wasm", height);
                    }
                } else {
                    println!("⚠️ Bloco {} sem transações.", height);
                }
            }
            Ok(r) => println!("⚠️ Bloco {} → erro HTTP {}", height, r.status()),
            Err(e) => println!("❌ Erro ao buscar bloco {}: {}", height, e),
        }

        thread::sleep(Duration::from_millis(400));
    }

    println!("\n🎯 Indexação concluída — resultados salvos em events.db");
    Ok(())
}

fn extract_and_store(conn: &Connection, tx: &Value, hash: &str, height: u64) -> Result<u32> {
    let timestamp = tx["timestamp"].as_str().unwrap_or_default();

    let empty_events = Vec::new();
    let events = tx["events"].as_array().unwrap_or(&empty_events);
    let mut count = 0;

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
                "_contract_address" => {
                    contract_address = attr["value"].as_str().unwrap_or_default().to_string()
                }
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
                    height.to_string(),
                    hash,
                    contract_address,
                    action,
                    sender,
                    recipient,
                    amount,
                    timestamp
                ],
            )?;
            count += 1;
        }
    }

    Ok(count)
}
