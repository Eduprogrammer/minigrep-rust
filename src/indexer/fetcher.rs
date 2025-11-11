use anyhow::{Context, Result};
use serde_json::Value;

/// Busca /block_results em um endpoint RPC do Cosmos/Xion
/// Ex.: endpoint = "https://rpc.xion-testnet-2.burnt.com:443"
pub fn fetch_block_results(endpoint: &str, height: u64) -> Result<Value> {
    let url = format!("{}/block_results?height={}", trim_trailing_slash(endpoint), height);
    let resp = reqwest::blocking::get(&url)
        .with_context(|| format!("falha ao requisitar URL: {}", url))?;

    if !resp.status().is_success() {
        anyhow::bail!("status HTTP não-sucesso {} para {}", resp.status(), url);
    }

    let json: Value = resp.json().context("falha ao desserializar JSON")?;
    Ok(json)
}

/// remove a "/" final se o usuário passar com barra
fn trim_trailing_slash(s: &str) -> String {
    if s.ends_with('/') {
        s.trim_end_matches('/').to_string()
    } else {
        s.to_string()
    }
}
