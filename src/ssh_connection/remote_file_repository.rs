use crate::ssh_connection::*;
use std::path;
use std::process::Command;

pub trait RemoteFileRepository {
    fn push(&self, remote_file: &RemoteFile, working_directory: &str) -> Result<()>;
}

pub struct RemoteFileRepositoryDefaultImpl;

impl RemoteFileRepository for RemoteFileRepositoryDefaultImpl {
    fn push(&self, remote_file: &RemoteFile, working_directory: &str) -> Result<()> {
        let user = remote_file.user_name_of_remote_file();
        let host = remote_file.host_name_of_remote_file();
        let local_path = remote_file.local_path_of_remote_file();
        let remote_path = remote_file.remote_path_of_remote_file();

        let real_local_path;
        {
            let mut p = path::Path::new(&working_directory).to_path_buf();
            p.push(local_path.to_string());
            match p.to_str() {
                None => return Err(Error::InvalidLocalPath),
                Some(s) => real_local_path = s.to_owned(),
            }
        }
        info!(
            "scp -C -v -q {} {}@{}:{}",
            &real_local_path,
            &user.to_string(),
            &host.to_string(),
            &remote_path.to_string()
        );

        let output = Command::new("scp")
            .arg("-C")
            .arg("-v")
            .arg("-q")
            .arg(real_local_path)
            .arg(format!(
                "{}@{}:{}",
                user.to_string(),
                host.to_string(),
                remote_path.to_string()
            ))
            .output()?;

        info!(
            "[*] result output: {}",
            std::str::from_utf8(&output.stdout)?
        );

        info!(
            "[*] result error output: {}",
            std::str::from_utf8(&output.stderr)?
        );
        Ok(())
    }
}

pub trait HasRemoteFileRepository {
    type Repository: RemoteFileRepository;

    fn remote_file_repository(&self) -> &Self::Repository;
}
