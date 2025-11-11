use anyhow::{Context, Result};
use serde_json::Value;

/// Busca uma transação da API pública e imprime apenas eventos wasm
fn main() -> Result<()> {
    // Você pode mudar o hash aqui para testar outras transações
    let hash = "98882FCA8E104245F1D00FFC6A4BDB657A3F286C69B67F21404B45C8417785C7";
    let url = format!(
        "https://api.xion-testnet-2.burnt.com/cosmos/tx/v1beta1/txs/{}",
        hash
    );

    println!("🔎 Buscando e analisando eventos wasm da transação {}", hash);
    let resp = reqwest::blocking::get(&url)
        .with_context(|| format!("Falha ao requisitar {}", url))?;

    if !resp.status().is_success() {
        anyhow::bail!("Status HTTP não-sucesso: {}", resp.status());
    }

    let json: Value = resp.json().context("Erro ao decodificar JSON")?;

    // Caminho até os eventos
    let events = json["tx_response"]["events"]
        .as_array()
        .unwrap_or(&vec![])
        .to_owned();

    if events.is_empty() {
        println!("⚠️ Nenhum evento encontrado nesta transação.");
        return Ok(());
    }

    let wasm_events: Vec<&Value> = events
        .iter()
        .filter(|ev| ev["type"] == "wasm")
        .collect();

    if wasm_events.is_empty() {
        println!("⚠️ Nenhum evento wasm nesta transação.");
        return Ok(());
    }

    println!("🧩 Eventos wasm encontrados:");
    for ev in wasm_events {
        println!("→ Tipo: {}", ev["type"]);
        if let Some(attrs) = ev["attributes"].as_array() {
            for attr in attrs {
                let key = attr["key"].as_str().unwrap_or("");
                let val = attr["value"].as_str().unwrap_or("");
                println!("   {} = {}", key, val);
            }
        }
        println!();
    }

    Ok(())
}
