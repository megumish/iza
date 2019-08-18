use crate::system_directory::*;
use futures::prelude::*;
use std::fs;
use std::path;
use std::pin::Pin;

pub trait SystemDirectoryMaker {
    fn make_top_directory(
        &'static self,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send>>;

    fn make_package_system(
        &self,
        top_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

    fn make_object_system(
        &self,
        top_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

    fn make_credential_system(
        &self,
        top_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
}

pub struct SystemDirectoryMakerDefaultImpl;

impl SystemDirectoryMaker for SystemDirectoryMakerDefaultImpl {
    fn make_top_directory(
        &'static self,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send>> {
        let working_directory = working_directory.to_owned();
        future::lazy(move |_| {
            let mut top_path_buf = path::Path::new(&working_directory).to_path_buf();
            top_path_buf.push(".iza");
            fs::create_dir(&top_path_buf)?;
            let top_path_string;
            match top_path_buf.to_str() {
                None => return Err(Error::InvalidTopPath),
                Some(s) => top_path_string = s,
            }
            Ok(top_path_string.to_owned())
        })
        .boxed()
    }

    fn make_package_system(
        &self,
        top_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let top_directory = top_directory.to_owned();
        future::lazy(move |_| {
            let mut package_top_path_buf = path::Path::new(&top_directory).to_path_buf();
            package_top_path_buf.push("package");
            fs::create_dir(&package_top_path_buf)?;

            Ok(package_top_path_buf)
        })
        .and_then(|t| {
            let t2 = t.clone();
            future::try_join(
                future::lazy(move |_| {
                    let mut packages_path_buf = t.clone();
                    packages_path_buf.push("packages");
                    let _ = fs::File::create(&packages_path_buf)?;

                    Ok(())
                }),
                future::lazy(move |_| {
                    let mut current_package_path_buf = t2.clone();
                    current_package_path_buf.push("current");
                    let _ = fs::File::create(&current_package_path_buf)?;

                    Ok(())
                }),
            )
        })
        .and_then(|_| future::ready(Ok(())))
        .boxed()
    }

    fn make_object_system(
        &self,
        top_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let top_directory = top_directory.to_owned();
        future::lazy(move |_| {
            let mut object_top_path_buf = path::Path::new(&top_directory).to_path_buf();
            object_top_path_buf.push("object");
            fs::create_dir(&object_top_path_buf)?;

            Ok(object_top_path_buf)
        })
        .and_then(|t| {
            let t2 = t.clone();
            future::try_join(
                future::lazy(move |_| {
                    let mut objects_path_buf = t.clone();
                    objects_path_buf.push("objects");
                    let _ = fs::File::create(&objects_path_buf)?;

                    Ok(())
                }),
                future::lazy(move |_| {
                    let mut object_info_path_buf = t2.clone();
                    object_info_path_buf.push("object_info");
                    let _ = fs::File::create(&object_info_path_buf)?;

                    Ok(())
                }),
            )
        })
        .and_then(|_| future::ready(Ok(())))
        .boxed()
    }

    fn make_credential_system(
        &self,
        top_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let top_directory = top_directory.to_owned();
        future::lazy(move |_| {
            let mut credential_top_path_buf = path::Path::new(&top_directory).to_path_buf();
            credential_top_path_buf.push("credential");
            fs::create_dir(&credential_top_path_buf)?;

            Ok(credential_top_path_buf)
        })
        .and_then(|t| {
            let t2 = t.clone();
            future::try_join(
                future::lazy(move |_| {
                    let mut credentials_path_buf = t.clone();
                    credentials_path_buf.push("credentials");
                    let _ = fs::File::create(&credentials_path_buf)?;

                    Ok(())
                }),
                future::lazy(move |_| {
                    let mut ssh_connection_path_buf = t2.clone();
                    ssh_connection_path_buf.push("ssh_connection");
                    let _ = fs::File::create(&ssh_connection_path_buf)?;

                    Ok(())
                }),
            )
        })
        .and_then(|_| future::ready(Ok(())))
        .boxed()
    }
}

pub trait HasSystemDirectoryMaker {
    type Component: SystemDirectoryMaker;

    fn system_directory_maker(&self) -> &Self::Component;
}
