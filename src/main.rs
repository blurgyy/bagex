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
    #[structopt(help = "Print the command to run and abort", short, long)]
    dry_run: bool,
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
    let config: config::BagexConfig =
        config::BagexConfig::from_pathbuf(config_file)?;
    log::trace!("Configuration read: {:#?}", config);

    let exe_abs_path: PathBuf = if opt.exe.starts_with("/")
        || opt.exe.starts_with("./")
    {
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

    if opt.dry_run {
        let envs_string: String = envs
            .keys()
            .into_iter()
            .map(|k| format!("{}=\"{}\"", k, envs.get(k).unwrap()))
            .collect::<Vec<String>>()
            .join(" ");
        let args_string: String = opt
            .args
            .iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<String>>()
            .join(" ");
        let command_string = vec![
            envs_string,
            exe_abs_path.to_str().unwrap().to_string(),
            args_string,
        ]
        .join(" ")
        .trim()
        .to_string();
        println!("{}", command_string);
    } else {
        log::debug!("Spawning process ..");
        Command::new(exe_abs_path)
            .envs(envs)
            .args(opt.args)
            .spawn()
            .expect("Failed to run executable");
    }

    Ok(())
}

fn setup() -> Result<(), Report> {
    env_logger::init();
    color_eyre::install()?;
    Ok(())
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 26 2021, 11:04 [CST]
