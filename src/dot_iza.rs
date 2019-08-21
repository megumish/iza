use futures::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml as yaml;
use std::fs;
use std::io::{Read, Write};
use std::path;
use std::sync::Arc;

pub fn init_dot_iza(working_directory: &'static str) -> ResultFuture<()> {
    future::lazy(move |_| {
        let top_path_buf = top_path_buf_of_working_directory(working_directory);
        fs::create_dir(&top_path_buf)?;
        Ok(())
    })
    .boxed()
}

pub fn init_module_file(
    working_directory: &'static str,
    module_prural_name: &'static str,
) -> ResultFuture<()> {
    future::lazy(move |_| {
        let modules_file_path_buf =
            modules_file_path_buf_of_working_directory(working_directory, module_prural_name);
        let _ = fs::File::create(&modules_file_path_buf)?;
        Ok(())
    })
    .boxed()
}

pub fn push_module<M, YM>(
    module: Arc<M>,
    working_directory: &'static str,
    module_prural_name: &'static str,
) -> ResultFuture<Arc<M>>
where
    M: Module,
    YM: YamlModule<M>,
{
    future::lazy(move |_| {
        let mut new_yaml_modules: Vec<YM> =
            yaml_modules_of_working_directory::<M, YM>(working_directory, module_prural_name)?;

        let new_yaml_module = YM::new_yaml_module(module.clone());
        match new_yaml_modules.iter().find(|m| *m == &new_yaml_module) {
            Some(_) => Err(ErrorKind::AlreadyExistModule)?,
            None => new_yaml_modules.push(new_yaml_module),
        }

        save_yaml_moduels_of_working_directory::<M, YM>(
            new_yaml_modules,
            working_directory,
            module_prural_name,
        )?;

        Ok(module.clone())
    })
    .boxed()
}

pub fn delete_module<M, YM>(
    module: Arc<M>,
    working_directory: &'static str,
    module_prural_name: &'static str,
) -> ResultFuture<Arc<M>>
where
    M: Module,
    YM: YamlModule<M>,
{
    future::lazy(move |_| {
        let new_yaml_modules: Vec<YM> =
            yaml_modules_of_working_directory::<M, YM>(working_directory, module_prural_name)?;

        let removed_yaml_module = YM::new_yaml_module(module.clone());
        let new_yaml_modules: Vec<YM> = new_yaml_modules
            .iter()
            .filter(|m| *m != &removed_yaml_module)
            .map(Clone::clone)
            .collect();

        save_yaml_moduels_of_working_directory::<M, YM>(
            new_yaml_modules,
            working_directory,
            module_prural_name,
        )?;

        Ok(module.clone())
    })
    .boxed()
}

fn yaml_modules_of_working_directory<M, YM>(
    working_directory: &str,
    module_prural_name: &str,
) -> Result<Vec<YM>, Error>
where
    M: Module,
    YM: YamlModule<M>,
{
    let modules_file_path_buf =
        modules_file_path_buf_of_working_directory(working_directory, module_prural_name);

    let mut input_data = Vec::new();
    let mut modules_file = fs::File::open(&modules_file_path_buf)?;
    modules_file.read_to_end(&mut input_data)?;
    Ok(if input_data.is_empty() {
        Vec::new()
    } else {
        yaml::from_slice(&input_data)?
    })
}

fn save_yaml_moduels_of_working_directory<M, YM>(
    yaml_modules: Vec<YM>,
    working_directory: &str,
    module_prural_name: &str,
) -> Result<(), Error>
where
    M: Module,
    YM: YamlModule<M>,
{
    let modules_file_path_buf =
        modules_file_path_buf_of_working_directory(working_directory, module_prural_name);

    let mut yaml_modules = yaml_modules;
    yaml_modules.sort();

    let output_data = yaml::to_vec(&yaml_modules)?;
    let mut modules_file = fs::File::create(&modules_file_path_buf)?;
    modules_file.write(&output_data)?;
    Ok(())
}

fn modules_file_path_buf_of_working_directory(
    working_directory: &str,
    module_prural_name: &str,
) -> path::PathBuf {
    let mut p = top_path_buf_of_working_directory(working_directory);
    p.push(module_prural_name);
    p
}

fn top_path_buf_of_working_directory(working_directory: &str) -> path::PathBuf {
    let mut p = path::Path::new(working_directory).to_path_buf();
    p.push(".iza");
    p
}

pub trait Module: Clone + Sync + Send + 'static {}
pub trait YamlModule<M>: Eq + std::hash::Hash + Serialize + DeserializeOwned + Ord + Clone {
    fn new_yaml_module(module: Arc<M>) -> Self
    where
        M: Module;
}

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "IO Error")]
    IO,
    #[fail(display = "Already Exist Module")]
    AlreadyExistModule,
    #[fail(display = "yaml serialize or deserialize error")]
    YamlSerializeOrDeserialize,
}

pub type ResultFuture<T> =
    std::pin::Pin<Box<dyn futures::Future<Output = std::result::Result<T, Error>> + Send>>;

/* ----------- failure boilerplate ----------- */

use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::IO),
        }
    }
}

impl From<yaml::Error> for Error {
    fn from(error: yaml::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::YamlSerializeOrDeserialize),
        }
    }
}
