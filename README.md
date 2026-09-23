# 🚀 rlog

**rlog** é um analisador e visualizador de logs interativo (*TUI*) desenvolvido em Rust, feito para ser extremamente leve, rápido e simples de usar diretamente no terminal Linux.

![License](https://img.shields.io/github/license/agorismo/rlog?color=blue)
![Rust](https://img.shields.io/badge/rust-2021-orange.svg)
![Release](https://img.shields.io/github/v/release/agorismo/rlog?color=green)

---

## ✨ Funcionalidades

- **Performance Extrema:** Leitura e indexação instantânea de arquivos de log com dezenas de milhares de linhas.
- **Destaque por Cores:** Identificação visual automática para níveis de log (`ERROR`, `WARN`, `INFO`, `DEBUG`).
- **Filtro por Nível Mínimo:** Exiba apenas falhas ou avisos críticos usando a opção `--min-level`.
- **Busca por Texto:** Filtre rapidamente mensagens contendo palavras-chave específicas com `--query`.
- **Navegação Estilo Vim:** Suporte total a rolagem por setas do teclado e teclas `j` / `k`.

---

## 🛠️ Requisitos e Instalação

### Compilando via Cargo (Rust)

Com o ecossistema Rust instalado no seu Arch Linux, execute:

```bash
git clone [https://github.com/agorismo/rlog.git](https://github.com/agorismo/rlog.git)
cd rlog
cargo install --path .

```

> **Dica no Arch Linux:** Certifique-se de que o diretório `~/.cargo/bin` está presente no seu `$PATH` (no arquivo `~/.zshrc` ou `~/.bashrc`).

---

## 🚀 Como Usar

### Exemplos do Dia a Dia

```bash
# Abrir e visualizar qualquer arquivo de log
rlog --file /var/log/pacman.log

# Filtrar para exibir apenas avisos (WARN) e erros (ERROR)
rlog --file /var/log/pacman.log --min-level warn

# Buscar termos específicos no log
rlog --file /var/log/pacman.log --query "upgraded"

```

---

## ⚙️ Controles no Terminal (TUI)

| Tecla | Ação |
| --- | --- |
| `↓` ou `j` | Rolar para a próxima linha |
| `↑` ou `k` | Rolar para a linha anterior |
| `q` ou `Esc` | Sair do aplicativo |

---


