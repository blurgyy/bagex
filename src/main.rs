use color_eyre::Report;
use std::{fs, path::PathBuf, str::FromStr};
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
    log::debug!("Initializing logger and error handler ..");
    setup()?;

    log::debug!("Reading command line args ..");
    let opt = Opt::from_args();
    log::trace!("Command line args read: {:#?}", opt);

    log::debug!("Determining path of configuration file ..");
    let config_file: PathBuf =
        opt.config_path.unwrap_or(utils::default_config_path());
    assert!(config_file.exists(), "{:?} does not exist", config_file);
    log::info!("Using configuration file {:?}", config_file);

    log::debug!("Reading configuration file ..");
    let confstr = fs::read_to_string(config_file).unwrap();
    let config: config::BagexConfig = toml::from_str(&confstr).unwrap();
    log::trace!("Configuration read: {:#?}", config);

    log::debug!("Composing PATH ..");
    let mut path: Vec<PathBuf> = config.path.unwrap_or_default();
    path.extend(
        std::env::var("PATH")
            .unwrap()
            .split(":")
            .map(|x| PathBuf::from_str(x).unwrap())
            .collect::<Vec<PathBuf>>(),
    );
    log::trace!("Paths in composed PATH: {:#?}", path);
    let env_path: String = path
        .iter()
        .map(|x| x.to_str().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(":");
    log::trace!("Composed PATH as environment variable: {:#?}", env_path);
    std::env::set_var("PATH", env_path);

    Ok(())
}

fn setup() -> Result<(), Report> {
    env_logger::init();
    color_eyre::install()?;
    Ok(())
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 26 2021, 11:04 [CST]
