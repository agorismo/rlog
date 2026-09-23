use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Parser, Debug)]
#[command(
    name = "rlog",
    author = "agorismo",
    version = "0.1.0",
    about = "Analisador e visualizador de logs interativo em Rust"
)]
pub struct Cli {
    /// caminho para o arquivo de log a ser analisado
    #[arg(short, long, value_name = "FILE")]
    pub file: PathBuf,

    /// filtro de nivel mínimo de log (debug, info, warn, error)
    #[arg(short, long, value_enum, default_value_t = LogLevel::Debug)]
    pub min_level: LogLevel,

    /// termo ou expressão para filtrar as linhas do log
    #[arg(short, long)]
    pub query: Option<String>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}