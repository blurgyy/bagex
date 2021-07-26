use color_eyre::Report;
use std::{
    collections::HashMap, fs, path::PathBuf, process::Command, str::FromStr,
};
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
    config_file: Option<PathBuf>,
}

fn main() -> Result<(), Report> {
    log::debug!("Initializing logger and error handler ..");
    setup()?;

    log::debug!("Parsing command line args ..");
    let opt = Opt::from_args();
    log::trace!("Command line args read: {:#?}", opt);

    log::debug!("Determining path of configuration file ..");
    let config_file: PathBuf =
        opt.config_file.unwrap_or(utils::default_config_file());
    assert!(config_file.exists(), "{:?} does not exist", config_file);
    log::info!("Using configuration file {:?}", config_file);

    log::debug!("Reading configuration file ..");
    let confstr = fs::read_to_string(config_file).unwrap();
    let config: config::BagexConfig = toml::from_str(&confstr).unwrap();
    log::trace!("Configuration read: {:#?}", config);

    let exe_abs_path: PathBuf = if opt.exe.starts_with("/") {
        log::debug!("An absolute path {} is requested", opt.exe);
        PathBuf::from_str(&opt.exe).unwrap_or_default()
    } else {
        log::debug!("Composing PATH ..");
        let path: Vec<PathBuf> = utils::compose_and_set_path(
            config.path.clone().unwrap_or_default(),
        );
        log::debug!("Finding executable '{}' from composed PATH ..", opt.exe);
        utils::get_executable_path(opt.exe.clone(), path)
    };
    log::info!("Using executable from {:?}", exe_abs_path);

    log::debug!("Composing environments for the executable ..");
    let envs: HashMap<String, String> = utils::compose_environments(
        exe_abs_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        config,
    );
    log::trace!("Composed additional environments: {:#?}", envs);

    log::debug!("Spawning process ..");
    Command::new(exe_abs_path)
        .args(opt.args)
        .envs(envs)
        .spawn()
        .expect("Failed to run executable");

    Ok(())
}

fn setup() -> Result<(), Report> {
    env_logger::init();
    color_eyre::install()?;
    Ok(())
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 26 2021, 11:04 [CST]
