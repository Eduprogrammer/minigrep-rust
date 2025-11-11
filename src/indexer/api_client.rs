use anyhow::{Context, Result};
use serde::Deserialize;

/// Estrutura simplificada de uma transação CosmWasm
#[derive(Debug, Deserialize)]
pub struct TxResponse {
    pub height: String,
    pub txhash: String,
    pub tx: Option<TxBody>,
}

#[derive(Debug, Deserialize)]
pub struct TxBody {
    pub body: TxBodyInner,
}

#[derive(Debug, Deserialize)]
pub struct TxBodyInner {
    pub messages: Vec<Msg>,
}

#[derive(Debug, Deserialize)]
pub struct Msg {
    #[serde(rename = "@type")]
    pub msg_type: String,
}

/// Estrutura de resultado da API
#[derive(Debug, Deserialize)]
struct ApiResult {
    pub tx_responses: Option<Vec<TxResponse>>,
}

/// Busca as últimas transações da testnet e filtra apenas as CosmWasm
pub fn fetch_wasm_txs(limit: u32) -> Result<Vec<TxResponse>> {
    let url = format!(
        "https://api.xion-testnet-2.burnt.com/cosmos/tx/v1beta1/txs?limit={}",
        limit
    );

    let resp = reqwest::blocking::get(&url)
        .with_context(|| format!("Falha ao requisitar URL: {}", url))?;

    if !resp.status().is_success() {
        anyhow::bail!("Status HTTP não-sucesso: {}", resp.status());
    }

    let api_result: ApiResult = resp.json().context("Erro ao interpretar JSON da API")?;

    // Filtra mensagens do tipo CosmWasm
    let filtered: Vec<TxResponse> = api_result
        .tx_responses
        .unwrap_or_default()
        .into_iter()
        .filter(|tx| {
            if let Some(tx_data) = &tx.tx {
                tx_data
                    .body
                    .messages
                    .iter()
                    .any(|m| m.msg_type.contains("cosmwasm.wasm"))
            } else {
                false
            }
        })
        .collect();

    Ok(filtered)
}
