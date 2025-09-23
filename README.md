# 🔎 Minigrep em Rust

Um mini clone simplificado do `grep`, escrito em Rust.  
O programa recebe como entrada uma **palavra** e um **arquivo de texto**, retornando todas as linhas que contêm a palavra.

---

## 📸 Demonstração

### Exemplo de uso no terminal:

```bash
cargo run -- "você" poema.txt


Procurando por: você
No arquivo: poema.txt

Eu sou ninguém! Quem é você?
você é muito engraçado!
Eu não sabia que você era uma pessoa tão engraçada assim.


🚀 Como rodar o projeto
1. Clone o repositório
git clone https://github.com/seu-usuario/minigrep-rust.git
cd minigrep-rust


2. Compile e execute
cargo run -- "palavra" input.txt


🛠️ Funcionalidades

Busca case-sensitive
Busca case-insensitive (com testes prontos)
Retorna todas as linhas contendo a palavra procurada
Implementação simples e organizada com Rust


📂 Estrutura do Projeto
src/
├── lib.rs   # lógica principal (Config, run, search, testes)
└── main.rs  # entrada da aplicação


⚙️ Tecnologias utilizadas

Rust
Cargo