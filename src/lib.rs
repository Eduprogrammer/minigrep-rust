use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("não há argumentos suficientes");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

     // 🔹 armazenamos as linhas encontradas em results
    let results = search_case_insensitive(&config.query, &contents);

    // 🔹 imprimindo cada linha encontrada
    for line in &results {
        println!("{}", line);
    }

    // 🔹 nova parte: contar o total de ocorrências da palavra no texto inteiro
    let total_count = contents
        .to_lowercase() // para buscar sem diferenciar maiúsculas/minúsculas
        .matches(&config.query.to_lowercase())
        .count();

    // 🔹 nova parte: mostrar resumo no final
    println!(
        "\nResumo: a palavra '{}' apareceu {} vez(es) no texto.",
        config.query, total_count
    );

    Ok(())
    
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
