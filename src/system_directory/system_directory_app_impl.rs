use crate::system_directory::*;

pub struct SystemDirectoryAppImpl;

impl HasSystemDirectoryMaker for SystemDirectoryAppImpl {
    type Component = SystemDirectoryMakerDefaultImpl;

    fn system_directory_maker(&self) -> &Self::Component {
        &SystemDirectoryMakerDefaultImpl
    }
}
