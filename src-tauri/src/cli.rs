use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "qpeek", author, version, about = "Quick Peek Markdown Cheat-Sheet Viewer", long_about = None)]
pub struct AppCli {
    #[arg(short, long, default_value_t = false)]
    pub daemon: bool,

    #[arg(short, long)]
    pub config: Option<String>,

    #[arg(short, long)]
    pub vault: Option<String>,
}

impl AppCli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
