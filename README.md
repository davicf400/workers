# 🦀 Workers Management System

Este é o meu primeiro projeto **CRUD Full-Stack** utilizando **Rust**. O objetivo principal foi explorar o ecossistema de alto desempenho do Rust no backend, integrando-o com um banco de dados relacional e uma interface moderna.

## 🚀 Tecnologias Utilizadas

### Backend
* **Language:** [Rust](https://www.rust-lang.org/) (Foco em segurança de memória e performance).
* **Framework Web:** [Axum](https://github.com/tokio-rs/axum) (Baseado em Tokio).
* **Database Driver:** [SQLx](https://github.com/launchbadge/sqlx) (Consultas SQL assíncronas e tipadas em tempo de compilação).
* **Banco de Dados:** PostgreSQL.

### Frontend
* **Interface:** HTML5, CSS3 (Flexbox/Grid) e Vanilla JavaScript.
* **Estética:** Inspirada no design moderno do **ClickUp**, com foco em usabilidade e clean UI.
* **Desenvolvimento:** A estrutura visual e a lógica de integração do front foram geradas e refinadas com o auxílio de **IA (Gemini/DeepSeek)**, permitindo focar a energia no aprendizado da arquitetura do Rust.

## 🛠️ Funcionalidades (CRUD Completo)
O sistema permite a gestão de 4 entidades principais: **Funcionários, Departamentos, Produtos e Fornecedores**.
* **Listagem (Read):** Busca dinâmica de dados via API.
* **Criação (Create):** Modal inteligente para inserção de novos registros.
* **Edição (Update):** Alteração de dados existentes com preenchimento automático.
* **Exclusão (Delete):** Remoção de registros com confirmação de segurança.

## 🧠 Aprendizados e Desafios
Sendo meu primeiro contato prático com desenvolvimento Web em Rust, os principais desafios foram:
1.  **Gerenciamento de Erros:** Entender o sistema de `Result` e `Option` do Rust para lidar com falhas de conexão.
2.  **Ambiente Linux:** Configuração do ambiente de desenvolvimento no **Fedora**, incluindo o gerenciamento do serviço PostgreSQL via terminal.
3.  **Variáveis de Ambiente:** Troubleshooting de conexões com banco de dados e autenticação (`28P01`), resolvido através de manipulação de variáveis no `.env` e strings de conexão no `main.rs`.
4.  **CORS:** Configuração de middlewares para permitir que o navegador se comunique com o servidor local.

## ⚙️ Como Rodar o Projeto

1.  Certifique-se de ter o **Rust** e o **PostgreSQL** instalados.
2.  Clone o repositório:
    ```bash
    git clone [https://github.com/SEU_USUARIO/workers.git](https://github.com/SEU_USUARIO/workers.git)
    ```
3.  Configure seu arquivo `.env` na raiz:
    ```env
    DATABASE_URL=postgres://usuario:senha@localhost:5432/nome_do_banco
    ```
4.  Rode as migrações ou crie as tabelas conforme as structs em `models.rs`.
5.  Execute o servidor:
    ```bash
    cargo run
    ```
6.  Acesse `http://localhost:3000` no seu navegador.

---
Desenvolvido por **Davicf** como projeto de estudo na **UFMT-blockchain**.
