#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use futures::{executor, prelude::*};
use iza::{package::*, system_directory::*};
use std::collections::HashMap;
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
        (@subcommand package =>
            (about: "package manager")
            (@subcommand new =>
                (about: "create new package")
                (@arg NAME: +required "new package name")
            )
        )
        (@subcommand credential =>
            (about: "credential manager")
            (@subcommand ssh =>
                (@subcommand new =>
                    (about: "ssh connection")
                    (@arg USER: +required "ssh user name")
                    (@arg HOST: +required "ssh host name")
                )
            )
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
        executor.run(init_future)?;
    }

    if let Some(matches) = matches.subcommand_matches("package") {
        if let Some(matches) = matches.subcommand_matches("new") {
            let name = matches.value_of("NAME").unwrap();

            let new_package_future = iza::SUITE
                .package_app()
                .new_package(name.to_owned(), current_dir.to_owned());

            let mut executor = executor::ThreadPool::new()?;
            executor.run(new_package_future)?;
        } else {
            let packages_future = iza::SUITE.package_app().packages(current_dir.to_owned());

            let mut executor = executor::ThreadPool::new()?;

            let packages = executor.run(packages_future)?;
            println!("Package List");
            packages.into_iter().for_each(|p| {
                println!("{}", p.name_of_package());
            });
        }
    }

    if let Some(matches) = matches.subcommand_matches("credential") {
        if let Some(matches) = matches.subcommand_matches("ssh") {
            if let Some(matches) = matches.subcommand_matches("new") {
                let user = matches.value_of("USER").unwrap();
                let host = matches.value_of("HOST").unwrap();

                let mut info = HashMap::new();
                info.insert("user", user);
                info.insert("host", host);

                // let new_ssh_future = iza::SUITE.credential_app().new_credential(
                //     "SSHConnection".to_owned(),
                //     info,
                //     current_dir.to_owned(),
                // );

                // let mut executor = executor::ThreadPool::new()?;
                // executor.run(new_ssh_future)?;
            }
        }
    }

    Ok(())
}
