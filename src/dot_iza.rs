use futures::prelude::*;
use std::fs;
use std::path;
use std::pin::Pin;

type RetFuture<T, E> = Pin<Box<dyn Future<Output = std::result::Result<T, E>> + Send>>;

pub fn init(working_directory: &'static str) -> RetFuture<(), failure::Error> {
    future::lazy(move |_| {
        let top_path_buf = top_path_buf_of_working_directory(working_directory);
        fs::create_dir(&top_path_buf)?;
        Ok(())
    })
    .boxed()
}

pub trait DotIza {
    type Module: Module;
    type YamlModule: YamlModule;
    type Error: Send + From<std::io::Error> + 'static;
    const MODULE_NAME: &'static str;
    const MODULE_PRURAL_NAME: &'static str;

    fn init_module_top(
        working_directory: &'static str,
    ) -> RetFuture<&'static path::Path, Self::Error> {
        future::lazy(move |_| {
            let mut module_top_path_buf = top_path_buf_of_working_directory(working_directory);
            module_top_path_buf.push(Self::MODULE_NAME);
            fs::create_dir(&module_top_path_buf)?;
            let module_top_path: &'static path::Path = {
                let t = module_top_path_buf.as_path().into();
                Box::leak::<'static>(t)
            };
            Ok(module_top_path)
        })
        .boxed()
    }

    fn init_module_files(top_path: &'static path::Path) -> RetFuture<(), Self::Error> {
        let top_path_buf = top_path.to_path_buf();
        let top_path_buf2 = top_path.to_path_buf();

        future::try_join(
            future::lazy(move |_| {
                let mut default_module_path_buf = top_path_buf.clone();
                default_module_path_buf.push("default");
                let _ = fs::File::create(&default_module_path_buf)?;

                Ok(())
            }),
            future::lazy(move |_| {
                let mut modules_path_buf = top_path_buf2.clone();
                modules_path_buf.push(Self::MODULE_PRURAL_NAME);
                let _ = fs::File::create(&modules_path_buf)?;

                Ok(())
            }),
        )
        .and_then(|_| future::ready(Ok(())))
        .boxed()
    }
}

pub trait Module {}
pub trait YamlModule {}

fn top_path_buf_of_working_directory(working_directory: &'static str) -> path::PathBuf {
    let mut p = path::Path::new(working_directory).to_path_buf();
    p.push(".iza");
    p
}
