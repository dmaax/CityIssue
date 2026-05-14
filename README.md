# 🏙️ CityIssue – Sistema de Gestão de Chamados Urbanos

O **CityIssue** é uma aplicação web desenvolvida em **Rust** que permite que cidadãos registrem problemas urbanos (buracos, iluminação, lixo, etc.) e que administradores gerenciem esses chamados através de um painel administrativo.

O objetivo do projeto é centralizar a comunicação entre população e gestão pública de forma simples, rápida e segura.

---

# Visão Geral

O sistema possui dois perfis principais:

## Cidadão
- Criar conta e autenticar
- Abrir chamados urbanos
- Acompanhar status dos chamados

##  Administrador
- Visualizar todos os chamados
- Gerenciar usuários
- Atualizar status dos chamados

---

# Stack Tecnológica

## Backend
- Rust
- Framework HTTP (ex: Actix / Axum – conforme dependências)
- JWT para autenticação
- Hash de senha seguro
- Arquitetura em camadas (Controller → Repository → Model)

## Frontend
- HTML, CSS e JavaScript puro
- Interface estática servida pelo backend

## Banco de Dados
- Banco relacional (configurado via `db.rs`)
- Camada de repositório para abstração de acesso

---
# Segurança Implementada

- Autenticação via **JWT**
- Hash de senha seguro (não armazena senha em texto plano)
- Validação de dados de entrada
- Separação de responsabilidades (arquitetura limpa)
- Padronização de respostas HTTP

---

# Como Executar o Projeto

## Pré-requisitos

Instalar Rust:

https://www.rust-lang.org/tools/install

Verificar instalação:

```bash
rustc --version
cargo --version
