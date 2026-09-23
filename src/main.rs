mod cli;
mod log;
mod tui;

use log::LogStore;

fn main() -> anyhow::Result<()> {
    let args = cli::parse_args();

    if !args.file.exists() {
        eprintln!("Erro: O arquivo '{:?}' não existe.", args.file);
        std::process::exit(1);
    }

    let logs = LogStore::load_from_file(&args.file)?;
    tui::run_app(args, logs)?;

    Ok(())
}