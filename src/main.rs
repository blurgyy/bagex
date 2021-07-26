use color_eyre::Report;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

use bagex::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "bagex",
    global_settings(&[structopt::clap::AppSettings::ColoredHelp])
)]
struct Opt {
    #[structopt(help = "Name of executable in PATH to run")]
    exe: String,
    #[structopt(help = "Additional arguments to pass to executable")]
    args: Vec<String>,
    #[structopt(help = "Path to config file", short, long)]
    config_path: Option<PathBuf>,
}

fn main() -> Result<(), Report> {
    // Initialize logger and error handler.
    setup()?;

    // Read command line args.
    let opt = Opt::from_args();
    log::debug!("Command line args read: {:#?}", opt);

    // Determine path of configuration file.
    let config_file: PathBuf =
        opt.config_path.unwrap_or(utils::default_config_path());
    assert!(config_file.exists(), "{:?} does not exist", config_file);
    log::debug!("Using configuration file {:?}", config_file);

    // Read configuration file.
    let confstr = fs::read_to_string(config_file).unwrap();
    let config: config::BagexConfig = toml::from_str(&confstr).unwrap();
    log::trace!("Configuration read: {:#?}", config);

    Ok(())
}

fn setup() -> Result<(), Report> {
    env_logger::init();
    color_eyre::install()?;
    Ok(())
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 26 2021, 11:04 [CST]
