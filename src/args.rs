use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "bagex",
    global_settings(&[structopt::clap::AppSettings::ColoredHelp])
)]
pub struct Args {
    #[structopt(help = "Name of executable in PATH to run")]
    pub exe: String,

    #[structopt(help = "Additional arguments to pass to executable")]
    pub args: Vec<String>,

    #[structopt(help = "Path to config file", short, long)]
    pub config_file: Option<PathBuf>,

    #[structopt(help = "Print the command to run and abort", short, long)]
    pub dry_run: bool,
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 31 2021, 18:32 [CST]
