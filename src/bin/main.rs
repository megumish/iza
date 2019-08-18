#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use futures::{executor, prelude::*};
use iza::{package::*, system_directory::*};
use std::env;

use std::result::Result;

fn main() -> Result<(), failure::Error> {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (setting: clap::AppSettings::SubcommandRequiredElseHelp)
        (author: "Keishi Kawada <megumish@exploitouri.st>")
        (about: "iikanizi ni deploy command")
        (@subcommand init =>
            (about: "initalize Iza environment")
            (version: "0.1.0")
            (author: "Keishi Kawada <megumish@exploitouri.st>")
        )
    )
    .get_matches();

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let current_dir = &env::current_dir()?
        .to_str()
        .expect("Current Directory Name is Invalid")
        .to_owned();
    if matches.subcommand_matches("init").is_some() {
        let init_future = iza::SUITE
            .system_directory_app()
            .new_system_directory(current_dir.to_owned())
            .map_err(Into::<failure::Error>::into)
            .and_then(move |_| {
                iza::SUITE
                    .package_app()
                    .new_package("master".to_owned(), current_dir.to_owned())
                    .map_err(Into::<failure::Error>::into)
            })
            .and_then(move |_| {
                iza::SUITE
                    .package_app()
                    .switch_current_package("master".to_owned(), current_dir.to_owned())
                    .map_err(Into::<failure::Error>::into)
            });
        let mut executor = executor::ThreadPool::new()?;
        match executor.run(init_future) {
            Err(err) => warn!("[-] Error: {:#?}", err),
            Ok(()) => info!("[+] Success Initialization"),
        }
    }
    Ok(())
}
