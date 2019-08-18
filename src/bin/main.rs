#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use futures::{executor, prelude::*};
use iza::{credential::*, object::*, package::*, ssh_connection::*, system_directory::*};
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
        (@subcommand object =>
            (about: "object manager")
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
        let init_future = iza::dot_iza::init(current_dir).and_then(|()| {
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

    // if let Some(matches) = matches.subcommand_matches("package") {
    //     if let Some(matches) = matches.subcommand_matches("new") {
    //         let name = matches.value_of("NAME").unwrap();

    //         let new_package_future = iza::SUITE
    //             .package_app()
    //             .new_package(name.to_owned(), current_dir.to_owned());

    //         let mut executor = executor::ThreadPool::new()?;
    //         executor.run(new_package_future)?;
    //     } else {
    //         let packages_future = iza::SUITE.package_app().packages(current_dir.to_owned());

    //         let mut executor = executor::ThreadPool::new()?;

    //         let packages = executor.run(packages_future)?;
    //         println!("Package List");
    //         packages.into_iter().for_each(|p| {
    //             println!("{}", p.name_of_package());
    //         });
    //     }
    // }

    // if let Some(matches) = matches.subcommand_matches("credential") {
    //     if let Some(matches) = matches.subcommand_matches("ssh") {
    //         if let Some(matches) = matches.subcommand_matches("new") {
    //             let user = matches.value_of("USER").unwrap();
    //             let host = matches.value_of("HOST").unwrap();

    //             let mut info = HashMap::new();
    //             info.insert("user".to_owned(), user.to_owned());
    //             info.insert("host".to_owned(), host.to_owned());

    //             let new_ssh_future = iza::SUITE.credential_app().new_credential(
    //                 "SSHConnection".to_owned(),
    //                 info,
    //                 current_dir.to_owned(),
    //             );

    //             let mut executor = executor::ThreadPool::new()?;
    //             executor.run(new_ssh_future)?;
    //         } else {
    //             let ssh_connections_future = iza::SUITE
    //                 .ssh_connection_app()
    //                 .ssh_connections(current_dir.to_owned());

    //             let mut executor = executor::ThreadPool::new()?;

    //             let ssh_connections = executor.run(ssh_connections_future)?;
    //             println!("SSHConnection List");
    //             ssh_connections.into_iter().for_each(|c| {
    //                 println!("id: {}", c.id_of_ssh_connection().to_string());
    //                 println!("user: {}", c.user_name_of_ssh_connection().to_string());
    //                 println!("host: {}", c.host_name_of_ssh_connection().to_string());
    //                 println!("");
    //             });
    //         }
    //     }
    // }

    // if let Some(matches) = matches.subcommand_matches("object") {
    //     if let Some(matches) = matches.subcommand_matches("new") {
    //         let local_path = matches.value_of("LOCAL_PATH").unwrap();
    //         let remote_path = matches.value_of("REMOTE_PATH").unwrap();
    //         let credential_id = matches.value_of("CREDENTIAL_ID").unwrap();

    //         let new_object_future = iza::SUITE
    //             .package_app()
    //             .current_package(current_dir.to_owned())
    //             .map_err(Into::<failure::Error>::into)
    //             .and_then(move |p| {
    //                 iza::SUITE
    //                     .object_app()
    //                     .new_object(
    //                         local_path.to_owned(),
    //                         remote_path.to_owned(),
    //                         credential_id.to_owned(),
    //                         p.name_of_package().to_string(),
    //                         current_dir.to_owned(),
    //                     )
    //                     .map_err(Into::<failure::Error>::into)
    //             });

    //         let mut executor = executor::ThreadPool::new()?;
    //         executor.run(new_object_future)?;
    //     } else {
    //         let objects_future = iza::SUITE
    //             .package_app()
    //             .current_package(current_dir.to_owned())
    //             .map_err(Into::<failure::Error>::into)
    //             .and_then(move |p| {
    //                 future::try_join(
    //                     future::ready(Ok(p.clone())),
    //                     iza::SUITE
    //                         .object_app()
    //                         .objects_of_package_id(
    //                             p.name_of_package().to_string(),
    //                             current_dir.to_owned(),
    //                         )
    //                         .map_err(Into::<failure::Error>::into),
    //                 )
    //             })
    //             .and_then(move |(p, os)| {
    //                 future::try_join3(
    //                     future::ready(Ok(p.clone())),
    //                     future::ready(Ok(os.clone())),
    //                     future::try_join_all(os.iter().map(|o| {
    //                         iza::SUITE
    //                             .object_app()
    //                             .object_info_of_id(
    //                                 o.object_info_id_of_object().to_string(),
    //                                 current_dir.to_owned(),
    //                             )
    //                             .map_err(Into::<failure::Error>::into)
    //                     })),
    //                 )
    //             });

    //         let mut executor = executor::ThreadPool::new()?;

    //         let (package, objects, objects_info) = executor.run(objects_future)?;
    //         println!(
    //             "Package <{}> Object List",
    //             package.name_of_package().to_string()
    //         );
    //         objects_info.iter().zip(objects.iter()).for_each(|(i, o)| {
    //             println!("id: {}", o.id_of_object().to_string());
    //             println!("object_info_id: {}", i.id_of_object_info().to_string());
    //             println!("local_path: {}", i.local_path_of_object_info().to_string());
    //             println!(
    //                 "remote_path: {}",
    //                 i.remote_path_of_object_info().to_string()
    //             );
    //             println!(
    //                 "credential_id: {}",
    //                 i.credential_id_of_object_info().to_string()
    //             );
    //             println!("");
    //         });
    //     }
    // }

    // if matches.subcommand_matches("deploy").is_some() {
    //     let deploy_future = iza::SUITE
    //         .package_app()
    //         .current_package(current_dir.to_owned())
    //         .map_err(Into::<failure::Error>::into)
    //         .and_then(move |p| {
    //             iza::SUITE
    //                 .object_app()
    //                 .objects_of_package_id(p.name_of_package().to_string(), current_dir.to_owned())
    //                 .map_err(Into::<failure::Error>::into)
    //         })
    //         .and_then(move |os| {
    //             future::try_join_all(os.iter().map(|o| {
    //                 iza::SUITE
    //                     .object_app()
    //                     .object_info_of_id(
    //                         o.object_info_id_of_object().to_string(),
    //                         current_dir.to_owned(),
    //                     )
    //                     .map_err(Into::<failure::Error>::into)
    //                     .and_then(move |i| {
    //                         iza::SUITE
    //                             .credential_app()
    //                             .deploy_object(
    //                                 i.credential_id_of_object_info().to_string(),
    //                                 i.local_path_of_object_info().to_string(),
    //                                 i.remote_path_of_object_info().to_string(),
    //                                 current_dir.to_owned(),
    //                             )
    //                             .map_err(Into::<failure::Error>::into)
    //                     })
    //             }))
    //         })
    //         .and_then(|_| future::ready(Ok(())));
    //     let mut executor = executor::ThreadPool::new()?;
    //     executor.run(deploy_future)?;
    // }
    Ok(())
}
