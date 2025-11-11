use anyhow::Result;
use rusqlite::{params, Connection};
use serde_json::Value;
use std::{thread, time::Duration};

const API_REST: &str = "https://api.xion-mainnet-1.burnt.com";
const API_RPC: &str = "https://rpc.xion-mainnet-1.burnt.com";

fn main() -> Result<()> {
    println!("🚀 Indexador Xion (MAINNET híbrido REST + RPC) — últimos 50 blocos\n");

    let conn = Connection::open("events_mainnet.db")?;
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

    // 1️⃣ pega altura atual
    let latest_url = format!("{}/cosmos/base/tendermint/v1beta1/blocks/latest", API_REST);
    let latest_resp = reqwest::blocking::get(&latest_url)?;
    let latest_json: Value = latest_resp.json()?;
    let latest_height = latest_json["block"]["header"]["height"]
        .as_str()
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0);

    println!("📦 Altura mais recente: {}\n", latest_height);

    for height in (latest_height.saturating_sub(50))..=latest_height {
        let mut wasm_found = 0;

        // 2️⃣ tenta REST
        let rest_url = format!("{}/cosmos/tx/v1beta1/txs/block/{}", API_REST, height);
        let rest_resp = reqwest::blocking::get(&rest_url);

        if let Ok(r) = rest_resp {
            if r.status().is_success() {
                let json: Value = r.json().unwrap_or_default();
                if let Some(txs) = json["tx_responses"].as_array() {
                    for tx in txs {
                        if let Some(hash) = tx["txhash"].as_str() {
                            wasm_found += extract_and_store(&conn, tx, hash, height)?;
                        }
                    }
                }
            }
        }

        // 3️⃣ se não achou nada, tenta via RPC
               // 3️⃣ se não achou nada, tenta via RPC
               if wasm_found == 0 {
                let rpc_url = format!("{}/block_results?height={}", API_RPC, height);
                let rpc_resp = reqwest::blocking::get(&rpc_url);
    
                if let Ok(r) = rpc_resp {
                    if r.status().is_success() {
                        let rpc_json: Value = r.json().unwrap_or_default();
    
                        if let Some(txs) = rpc_json["result"]["txs_results"].as_array() {
                            for tx in txs {
                                if let Some(events) = tx["events"].as_array() {
                                    for ev in events {
                                        if ev["type"] == "wasm" {
                                            let binding = vec![];
                                            let attrs = ev["attributes"].as_array().unwrap_or(&binding);
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
                                                        format!("rpc_{}_{}", height, wasm_found), // gera hash fake só p/ salvar
                                                        contract_address,
                                                        action,
                                                        sender,
                                                        recipient,
                                                        amount,
                                                        "", // RPC não tem timestamp
                                                    ],
                                                )?;
                                                wasm_found += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
    

        if wasm_found > 0 {
            println!("✅ Bloco {} → {} evento(s) wasm encontrado(s)", height, wasm_found);
        } else {
            println!("⬜ Bloco {} → nenhum evento wasm", height);
        }

        thread::sleep(Duration::from_millis(700));
    }

    println!("\n🎯 Indexação concluída — resultados salvos em events_mainnet.db");
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
