use clap::{Parser, Subcommand};
#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 新規口座作成
    New,
    /// 口座に入金
    Deposit,
    /// 口座から出金
    Withdraw,
    /// CSVからインポート
    Import,
    /// レポート出力
    Report,
}

fn main() {
    let _args = App::parse();
}
