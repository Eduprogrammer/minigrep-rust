use anyhow::{Context, Result};
use serde_json::Value;

fn main() -> Result<()> {
    let hash = "98882FCA8E104245F1D00FFC6A4BDB657A3F286C69B67F21404B45C8417785C7";
    let url = format!(
        "https://api.xion-testnet-2.burnt.com/cosmos/tx/v1beta1/txs/{}",
        hash
    );

    println!("🔎 Buscando transação {}", hash);
    let resp = reqwest::blocking::get(&url)
        .with_context(|| format!("Falha ao requisitar {}", url))?;

    if !resp.status().is_success() {
        anyhow::bail!("Status HTTP não-sucesso: {}", resp.status());
    }

    let json: Value = resp.json().context("Erro ao decodificar JSON")?;
    println!("✅ JSON recebido:");
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
