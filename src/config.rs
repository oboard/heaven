use clap::Parser;


#[derive(Parser)]
pub struct Build {
    #[arg(short, long)]
    pub target: Option<String>,
    #[arg(long, default_value = "src")]
    pub source_dir: String,
    #[arg(long, default_value = "target/app/{target}")]
    pub target_dir: String,
}