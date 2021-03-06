/// Boostrap
///
/// # Description
/// The boostrap module is use for creating the project folder
use std::path::PathBuf;
use crate::core::fs::toolbox;
use crate::core::errors::cli_error::{CliErr, ErrHelper, ErrMessage};

/// Project Path struct
pub struct ConfigHelper {
    path: PathBuf
}

impl ConfigHelper {
    /// New
    ///
    /// # Description
    /// Return a new ProjectPath struct
    ///
    /// # Arguments
    /// * `input` the input string value input by the user capoomobi init <name> <project_path>
    /// * `project_name` &str project name
    ///
    /// # Return
    /// ProjectPath struct
    pub fn new(input: &str, project_name: &str) -> Self {
        let mut absolute_path = PathBuf::new();
        if input.is_empty() {
            absolute_path.push("./");
        } else {
            absolute_path.push(input);
        }

        absolute_path.push(project_name);

        ConfigHelper {
            path: absolute_path
        }
    }

    /// Build Project Folder
    ///
    /// # Description
    /// Create an empty project folder
    ///
    /// # Return
    /// Result<PathBuf, CliErr>
    pub fn build_project_folder(&self) -> Result<PathBuf, CliErr> {
        let res = toolbox::create_folder_from_pathbuf(&self.path);
        if let Err(err) = res {
            return Err(CliErr::new("", &err.to_string(), ErrMessage::IOError));
        }

        match toolbox::get_absolute_path(&self.path) {
            Ok(p) => Ok(p),
            Err(err) => Err(CliErr::new("", &err.to_string(), ErrMessage::IOError))
        }
    }

    /// Get Path As String
    ///
    /// # Description
    /// Get the path of the project as a String
    ///
    /// # Return
    /// String
    pub fn get_path_as_string(&self) -> String {
        match toolbox::get_absolute_path(&self.path) {
            Ok(p) => String::from(p.to_str().unwrap_or("")),
            Err(_) => String::new()
        }
    }
}
