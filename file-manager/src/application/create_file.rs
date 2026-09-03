use crate::{infrastructure::file_system_adapter};
use crate::infrastructure::path_resolver::PathResolver;

use super::errors::ApplicationError;


pub fn create_file(file_path: &str, file_name: &str) -> Result<String, ApplicationError> {
    let resolved_path = file_path.resolve_tilde()?;

    
    let file = file_system_adapter::join_path(&resolved_path, &file_name);
    
    if !file_system_adapter::path_exists(&resolved_path) {
        return Err(ApplicationError::DirNotExists);
    }
    if file_system_adapter::path_exists(&file) {
        return Err(ApplicationError::FileExists);
    }

    file_system_adapter::new_file(&file)?;
    
    Ok(file)
}