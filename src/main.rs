use color_eyre::Report;
use std::{
    collections::HashMap, path::PathBuf, process::Command, str::FromStr,
};
use structopt::StructOpt;

fn main() -> Result<(), Report> {
    log::debug!("Initializing logger and error handler ..");
    setup()?;

    log::debug!("Parsing command line args ..");
    let opt = bagex::args::Args::from_args();
    log::trace!("Command line args read: {:#?}", opt);

    log::debug!("Determining path of configuration file ..");
    let config_file: PathBuf = opt
        .config_file
        .unwrap_or(bagex::utils::default_config_file());
    assert!(config_file.exists(), "{:?} does not exist", config_file);
    log::info!("Using configuration file {:?}", config_file);

    log::debug!("Reading configuration file ..");
    let config = bagex::config::BagexConfig::from_pathbuf(config_file)?;
    if config.to_owned().validate() {
        log::trace!("Configuration read: {:#?}", config);
    } else {
        panic!("Error in configuration file");
    }

    let exe_abs_path: PathBuf = if opt.exe.contains("/") {
        log::warn!("A path ({}) instead of a name is requested", opt.exe);
        PathBuf::from_str(&opt.exe).unwrap_or_default()
    } else {
        log::debug!("Composing PATH ..");
        let path: Vec<PathBuf> = bagex::utils::compose_and_set_env_path(
            config.path.clone().unwrap_or_default(),
        );
        log::debug!("Finding executable '{}' from composed PATH ..", opt.exe);
        bagex::utils::get_exe_abs_path(opt.exe.clone(), path)
    };
    log::info!("Using executable from {:?}", exe_abs_path);

    log::debug!("Composing environments for the executable ..");
    let envs: HashMap<String, String> = bagex::utils::compose_environments(
        exe_abs_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        &config,
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
        let mut command = Command::new(exe_abs_path);
        if opt.clear_env || config.clear_env.unwrap_or(false) {
            log::info!("Command will not inherite environments from underlying shell");
            command.env_clear();
        }
        log::debug!("Spawning process ..");
        command
            .envs(envs)
            .args(opt.args)
            .status()
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
