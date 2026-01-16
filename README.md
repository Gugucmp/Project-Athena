# 🦉 Athena CLI Assistant

> Uma assistente virtual de linha de comando (CLI) desenvolvida em **Rust**, focada em performance, segurança de memória e automação de tarefas.

![Rust](https://img.shields.io/badge/made%20with-Rust-orange?style=for-the-badge&logo=rust)
![Status](https://img.shields.io/badge/status-Em%20Desenvolvimento-yellow?style=for-the-badge)

## 📋 Sobre o Projeto
A **Athena** nasceu da necessidade de automatizar rotinas e consumir APIs diretamente pelo terminal, sem a necessidade de interfaces gráficas pesadas. 

Este projeto é o marco da minha transição de carreira para o desenvolvimento de software, onde aplico conceitos de **Ownership**, **Borrowing** e **Assincronicidade** do Rust na prática.

## 🚀 Funcionalidades
* **Comunicação Web:** Realiza requisições HTTP robustas (GET/POST) para integrar com APIs externas.
* **Processamento de Dados:** Serialização e desserialização de JSON em tempo real.
* **Interface Amigável:** Saída de terminal colorida e formatada para melhor experiência do usuário (UX).
* **Gestão Temporal:** Manipulação precisa de datas e horários (Timezone aware).

## 🛠️ Tecnologias e Crates
O projeto foi construído utilizando o ecossistema moderno do Rust:

| Crate | Função |
|-------|--------|
| **`reqwest`** | Cliente HTTP assíncrono para consumo de APIs. |
| **`serde` / `serde_json`** | Framework para serializar/desserializar dados complexos. |
| **`tokio`** | Runtime para execução de código assíncrono (Async/Await). |
| **`colored`** | Estilização de texto no terminal. |
| **`chrono`** | Manipulação e formatação de datas e horas. |

## 📦 Como rodar localmente

### Pré-requisitos
* Rust e Cargo instalados ([Guia de instalação](https://www.rust-lang.org/tools/install))

### Instalação
```bash
# Clone este repositório
$ git clone [https://github.com/SEU-USUARIO/athena-cli-assistant.git](https://github.com/SEU-USUARIO/athena-cli-assistant.git)

# Entre na pasta
$ cd athena-cli-assistant

# Compile e rode o projeto
$ cargo run
