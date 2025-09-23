# 🔎 Minigrep em Rust

Um mini clone simplificado do `grep`, escrito em **Rust**.  
O programa recebe como entrada uma palavra e um ou mais arquivos de texto, retornando todas as linhas que contêm a palavra, além de um resumo com as contagens por arquivo e no total.


---

## 📸 Demonstração

### Exemplo de uso no terminal:

```bash
cargo run "você" poema.txt livro.txt

## Saída esperada:

📂 Lendo arquivo: livro.txt

Linha 2 (livro.txt): Eu sabia que você iria voltar
Linha 5 (livro.txt): Você é a razão da melodia

📂 Lendo arquivo: poema.txt

Linha 16 (poema.txt): E é por isso que você é tão engraçado.

📊 Resumo Final:
A palavra 'você' apareceu no total de: 3 vez(es).
2 vez(es) em livro.txt
1 vez(es) em poema.txt



🚀 Como rodar o projeto
1. Clone o repositório
git clone https://github.com/seu-usuario/minigrep-rust.git
cd minigrep-rust


2. Compile e execute
cargo run -- "palavra" arquivo1.txt arquivo2.txt 


🛠️ Funcionalidades

✔️ Busca case-insensitive (não diferencia maiúsculas e minúsculas)
✔️ Aceita múltiplos arquivos na mesma execução
✔️ Exibe o nome do arquivo e número da linha para cada ocorrência encontrada
✔️ Mostra um resumo final com contagem por arquivo e no total
✔️ Implementação simples e organizada com Rust

## 📂 Estrutura do Projeto

minigrep/
└── src/
├── lib.rs # lógica principal (Config, run, testes)
└── main.rs # ponto de entrada da aplicação
├── Cargo.toml
├── README.md
├── poema.txt # arquivo de exemplo para testar a busca
├── livro.txt # outro arquivo de exemplo


⚙️ Tecnologias utilizadas

Rust
Cargo