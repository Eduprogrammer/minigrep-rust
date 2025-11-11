pub mod indexer;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filenames: Vec<String>, // alteração: agora aceita vários arquivos
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("não há argumentos suficientes");
        }

        let query = args[1].clone();
        let filenames = args[2..].to_vec(); // pega todos os arquivos a partir do índice 2

        Ok(Config { query, filenames })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut total_global = 0;
    let mut resultados: Vec<(String, usize)> = Vec::new();

    // 🔹 clona e ordena a lista de arquivos antes de processar
    let mut filenames = config.filenames.clone();
    filenames.sort();

    for filename in filenames {
        println!("Lendo arquivo: {}\n", filename);

        let f = File::open(&filename)?;
        let reader = BufReader::new(f);

        let mut total_count = 0;

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if line.to_lowercase().contains(&config.query.to_lowercase()) {
                println!("Linha {} ({}): {}", i + 1, filename, line);
                total_count += line
                    .to_lowercase()
                    .matches(&config.query.to_lowercase())
                    .count();
            }
        }

        total_global += total_count;
        resultados.push((filename.clone(), total_count));
    }

    println!("\n📊 Resumo Final:");
    println!(
        "A palavra '{}' apareceu no total de: {} vez(es).",
        config.query, total_global
    );

    for (filename, count) in resultados {
        println!("{} vez(es) em {}", count, filename);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

// 🔹 Teste 1: Config::new deve falhar se houver poucos argumentos
#[test]
fn config_new_args_insuficientes() {
    let args = vec![String::from("programa")];
    let result = Config::new(&args);
    assert!(result.is_err());
}

// 🔹 Teste 2: Pesquisa simples em um arquivo com uma ocorrência
#[test]
fn busca_uma_ocorrencia() -> Result<(), Box<dyn Error>> {
    let mut temp = NamedTempFile::new()?;
    writeln!(temp, "Rust é incrível!")?;

    let filename = temp.path().to_str().unwrap().to_string();
    let config = Config {
        query: "incrível".to_string(),
        filenames: vec![filename],
    };

    let result = run(config);
    assert!(result.is_ok());
    Ok(())
}

// 🔹 Teste 3: Pesquisa em múltiplos arquivos com várias ocorrências
#[test]
fn busca_multiplos_arquivos() -> Result<(), Box<dyn Error>> {
    let mut temp1 = NamedTempFile::new()?;
    let mut temp2 = NamedTempFile::new()?;
    writeln!(temp1, "Rust é rápido. Rust é seguro.")?;
    writeln!(temp2, "Rust domina sistemas.")?;

    let filenames = vec![
        temp1.path().to_str().unwrap().to_string(),
        temp2.path().to_str().unwrap().to_string(),
    ];

    let config = Config {
        query: "Rust".to_string(),
        filenames,
    };

    let result = run(config);
        assert!(result.is_ok());
        Ok(())
    }
}
