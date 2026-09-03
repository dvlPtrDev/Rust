use crate::application::errors::ApplicationError;

use super::os_adapter;

pub trait PathResolver {
    fn resolve_tilde(&self) -> Result<String, ApplicationError>;
}

impl PathResolver for str {
    fn resolve_tilde(&self) -> Result<String, ApplicationError> {
        if self.starts_with('~') {
            let Some(user_home) = os_adapter::get_user_home() else {
                return Err(ApplicationError::UserHomeNotFound);
            };
    
            return Ok(self.replace(
                '~',
                user_home.to_string_lossy().as_ref(),
            ));
        }
    
        Ok(self.to_string())
    }
}