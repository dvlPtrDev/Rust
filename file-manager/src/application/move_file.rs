use crate::{
    application::errors::ApplicationError, infrastructure::{file_system_adapter, path_resolver::{PathResolver}}
};

pub fn move_file(src: &str, dest: &str) -> Result<(), ApplicationError> {
    let (src, dest) = (
        src.resolve_tilde()?, 
        dest.resolve_tilde()?
    );
    if !file_system_adapter::path_exists(&src) {
        return Err(ApplicationError::FileNotExists);
    }
    if file_system_adapter::path_exists(&dest) {
        return Err(ApplicationError::FileExists);
    } 
    file_system_adapter::move_file(&src, &dest);
    Ok(())
}