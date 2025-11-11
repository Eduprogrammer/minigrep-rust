use serde_json::Value;

/// Extrai todos os eventos `wasm` de dentro de um JSON retornado pelo /block_results
///
/// Retorna um vetor de tuplas (tipo_evento, atributos)
pub fn extract_wasm_events(json: &Value) -> Vec<(String, Vec<(String, String)>)> {
    let mut eventos = Vec::new();

    // Navega até result.txs_results[*].events
    if let Some(txs_results) = json.get("result").and_then(|r| r.get("txs_results")).and_then(|t| t.as_array()) {
        for tx in txs_results {
            if let Some(events) = tx.get("events").and_then(|e| e.as_array()) {
                for ev in events {
                    if let Some(event_type) = ev.get("type").and_then(|t| t.as_str()) {
                        if event_type == "wasm" {
                            // extrai atributos
                            let mut atributos = Vec::new();
                            if let Some(attrs) = ev.get("attributes").and_then(|a| a.as_array()) {
                                for attr in attrs {
                                    let key = attr.get("key").and_then(|k| k.as_str()).unwrap_or("").to_string();
                                    let value = attr.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                    atributos.push((key, value));
                                }
                            }
                            eventos.push((event_type.to_string(), atributos));
                        }
                    }
                }
            }
        }
    }

    eventos
}
