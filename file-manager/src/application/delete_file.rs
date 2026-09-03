use crate::{
    application::errors::ApplicationError, 
    infrastructure::{
        file_system_adapter, 
        path_resolver::PathResolver
    }
};


pub fn delete_file(file_path: &str) -> Result<(), ApplicationError>{
    let resolved_path = file_path.resolve_tilde()?;

    if !file_system_adapter::path_exists(&resolved_path) {
        return Err(ApplicationError::FileNotExists);
    }
    file_system_adapter::delete_file(file_path);

    Ok(())
}