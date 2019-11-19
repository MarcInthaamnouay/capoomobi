use crate::cli::scenarios::scenes::picker::EnumHelper;
use crate::cli::scenarios::sketch::helper;

/// Generate Options
/// 
/// # Description
/// Supported command options
pub enum GenerateOptions {
  Print,
  Ingress
}

impl EnumHelper<GenerateOptions> for GenerateOptions {
  fn from_string(action: &String) -> Option<GenerateOptions> {
      match action.to_lowercase().as_str() {
          "--print" => Some(GenerateOptions::Print),
          "--ingress" => Some(GenerateOptions::Ingress),
          _ => None
      }
  }
}

/// Retrieve Cmd Options
///
/// # Description
/// Retrive the options passed to a command
/// 
/// # Arguments
/// * `options` &Vec<String>
/// 
/// # Return
/// Option<GenerateOptions>
pub fn retrieve_cmd_options(options: &Vec<String>) -> Option<GenerateOptions> {
  let opt = match helper::retrieve_options_by_idx(options, 0) {
      Some(p) => p,
      None => String::new()
  };

  GenerateOptions::from_string(&opt)
}
