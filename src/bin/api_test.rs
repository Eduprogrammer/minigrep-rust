use anyhow::Result;
use minigrep::indexer::api_client::fetch_wasm_txs;

fn main() -> Result<()> {
    println!("🔎 Buscando últimas transações CosmWasm na testnet Xion...");

    let txs = fetch_wasm_txs(5)?;

    if txs.is_empty() {
        println!("⚠️ Nenhuma transação wasm encontrada.");
    } else {
        for tx in txs {
            println!("✅ Altura: {}, Hash: {}", tx.height, tx.txhash);
        }
    }

    Ok(())
}
