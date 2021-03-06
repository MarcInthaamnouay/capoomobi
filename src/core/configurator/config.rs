use super::configure;
use crate::core::errors::cli_error::ErrHelper;

/// Get Current Project Path
/// 
/// # Description
/// Retrieve the current project set within the .capoomobi.json
/// 
/// # Return
/// Option<String>
pub fn get_current_project_path() -> Option<String> {
    let config_opt = configure::exist();
    // Check if None
    config_opt.as_ref()?;

    let config = config_opt.unwrap();
    let capoos = config.get_content();
    if let Err(err) = capoos {
        err.log_pretty();
        return None;
    }

    let unwrapped_capoos = capoos.unwrap();
    let current_name = unwrapped_capoos.current;
    let project_path = unwrapped_capoos
        .projects
        .into_iter()
        .filter(|p| p.name == current_name)
        .map(|p| p.path)
        .fold(String::new(), |_, value| value);

    if project_path.is_empty() {
        return None;
    }
    
    Some(project_path)
}
