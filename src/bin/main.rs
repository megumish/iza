#[macro_use]
extern crate clap;

use futures::{executor, prelude::*};
use iza::{credential::*, object::*, package::*};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

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
            (@subcommand rm =>
                (about: "remove new package")
                (@arg NAME: +required "removed package name")
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
        (@subcommand object =>
            (about: "object manager")
            (@arg PACKAGE_ID: +required "package name")
            (@subcommand new =>
                (about: "create new package")
                (setting: clap::AppSettings::ArgRequiredElseHelp)
                (@arg LOCAL_PATH: +required "new object local path")
                (@arg REMOTE_PATH: +required "new object remote path")
                (@arg CREDENTIAL_ID: +required "id of credential to deploy")
            )
        )
        (@subcommand deploy =>
            (about: "iza deploy")
            (@arg PACKAGE_ID: +required "package name")
        )
    )
    .get_matches();

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let current_dir: &'static str = {
        let d = Box::new(
            env::current_dir()?
                .to_str()
                .expect("Current Directory Name is Invalid")
                .to_owned(),
        );
        Box::leak::<'static>(d)
    };

    if matches.subcommand_matches("init").is_some() {
        let current_dir = current_dir.clone();
        let init_future = iza::dot_iza::init_dot_iza(current_dir)
            .map_err(Into::<failure::Error>::into)
            .and_then(|()| {
                future::try_join3(
                    iza::SUITE
                        .package_app()
                        .init(current_dir)
                        .map_err(Into::<failure::Error>::into),
                    iza::SUITE
                        .credential_app()
                        .init(current_dir)
                        .map_err(Into::<failure::Error>::into),
                    iza::SUITE
                        .object_app()
                        .init(current_dir)
                        .map_err(Into::<failure::Error>::into),
                )
            });
        let mut executor = executor::ThreadPool::new()?;
        let _ = executor.run(init_future)?;
    }

    if let Some(matches) = matches.subcommand_matches("package") {
        if let Some(matches) = matches.subcommand_matches("new") {
            let name = matches.value_of("NAME").unwrap();

            let new_package_future = iza::SUITE
                .package_app()
                .new_package(name.to_owned(), current_dir);

            let mut executor = executor::ThreadPool::new()?;
            let package = executor.run(new_package_future)?;
            println!("[+] New Package");
            println!("{}", package.name_of_package().to_string())
        } else if let Some(matches) = matches.subcommand_matches("rm") {
            let name = matches.value_of("NAME").unwrap();

            let rm_package_future = iza::SUITE
                .package_app()
                .delete_package(name.to_owned(), current_dir);

            let mut executor = executor::ThreadPool::new()?;
            let package = executor.run(rm_package_future)?;
            println!("[+] Removed Package");
            println!("{}", package.name_of_package().to_string())
        } else {
            let packages_future = iza::SUITE.package_app().packages(current_dir);

            let mut executor = executor::ThreadPool::new()?;

            let packages = executor.run(packages_future)?;
            println!("Package List");
            packages.into_iter().for_each(|p| {
                println!("{}", p.name_of_package().to_string());
            });
        }
    }

    if let Some(matches) = matches.subcommand_matches("credential") {
        if let Some(matches) = matches.subcommand_matches("ssh") {
            if let Some(matches) = matches.subcommand_matches("new") {
                let user = matches.value_of("USER").unwrap();
                let host = matches.value_of("HOST").unwrap();

                let mut info = HashMap::new();
                info.insert("user".to_owned(), user.to_owned());
                info.insert("host".to_owned(), host.to_owned());

                let new_ssh_future = iza::SUITE.credential_app().new_credential(
                    Arc::new(info),
                    "SSHConnection".to_owned(),
                    current_dir,
                );

                let mut executor = executor::ThreadPool::new()?;
                let (credential, detail) = executor.run(new_ssh_future)?;
                println!("[+] New Credential");
                println!("id: {}", credential.id_of_credential().to_string());
                println!("kind: {}", credential.kind_of_credential().to_string());
                println!("user: {}", detail["user"]);
                println!("host: {}", detail["host"]);
            }
        } else {
            let credentials_future = iza::SUITE.credential_app().credentials(current_dir);

            let mut executor = executor::ThreadPool::new()?;

            let (credentials, details) = executor.run(credentials_future)?;
            println!("Credential List");
            credentials
                .into_iter()
                .zip(details.iter())
                .for_each(|(c, d)| {
                    println!("id: {}", c.id_of_credential().to_string());
                    println!("kind: {}", c.kind_of_credential().to_string());
                    println!("user: {}", d["user"]);
                    println!("host: {}", d["host"]);
                    println!("");
                });
        }
    }

    if let Some(matches) = matches.subcommand_matches("object") {
        let package_id = matches.value_of("PACKAGE_ID").unwrap();
        if let Some(matches) = matches.subcommand_matches("new") {
            let local_path = matches.value_of("LOCAL_PATH").unwrap();
            let remote_path = matches.value_of("REMOTE_PATH").unwrap();
            let credential_id = matches.value_of("CREDENTIAL_ID").unwrap();

            let new_object_future = iza::SUITE.object_app().new_object(
                local_path.to_owned(),
                remote_path.to_owned(),
                credential_id.to_owned(),
                package_id.to_owned(),
                current_dir,
            );

            let mut executor = executor::ThreadPool::new()?;
            let (object, info) = executor.run(new_object_future)?;
            println!("[+] New Object");
            println!("id: {}", object.id_of_object().to_string());
            println!("package_id: {}", object.package_id_of_object().to_string());
            println!("object_info_id: {}", info.id_of_object_info().to_string());
            println!(
                "local_path: {}",
                info.local_path_of_object_info().to_string()
            );
            println!(
                "remote_path: {}",
                info.remote_path_of_object_info().to_string()
            );
            println!(
                "credential_id: {}",
                info.credential_id_of_object_info().to_string()
            );
        } else {
            let objects_future = iza::SUITE
                .object_app()
                .objects_of_package_id(package_id.to_owned(), current_dir);

            let mut executor = executor::ThreadPool::new()?;

            let objects = executor.run(objects_future)?;
            println!("[+] Package <{}> Object List", package_id);
            objects.iter().for_each(|(o, i)| {
                println!("id: {}", o.id_of_object().to_string());
                println!("object_info_id: {}", i.id_of_object_info().to_string());
                println!("local_path: {}", i.local_path_of_object_info().to_string());
                println!(
                    "remote_path: {}",
                    i.remote_path_of_object_info().to_string()
                );
                println!(
                    "credential_id: {}",
                    i.credential_id_of_object_info().to_string()
                );
                println!("");
            });
        }
    }

    if let Some(matches) = matches.subcommand_matches("deploy") {
        let package_id = matches.value_of("PACKAGE_ID").unwrap();
        let deploy_future = iza::SUITE
            .object_app()
            .objects_of_package_id(package_id.to_owned(), current_dir)
            .map_err(Into::<failure::Error>::into)
            .and_then(move |os| {
                future::try_join_all(os.iter().map(move |(_, i)| {
                    iza::SUITE
                        .credential_app()
                        .deploy_object(
                            i.credential_id_of_object_info().to_string(),
                            i.local_path_of_object_info().to_string(),
                            i.remote_path_of_object_info().to_string(),
                            current_dir,
                        )
                        .map_err(Into::<failure::Error>::into)
                }))
            })
            .and_then(|_| future::ready(Ok(())));
        let mut executor = executor::ThreadPool::new()?;
        executor.run(deploy_future)?;
    }
    Ok(())
}
