use std::collections::HashMap;
use crate::docker::lexer;
use crate::docker::lexer::compose;
use crate::cli::core::logger::logging;
use crate::cli::core::input::input;
use crate::kubernetes::{tree, io};
use crate::errors::cli_error::ErrHelper;

/// Constant referring to the compose file which need to be parse
const COMPOSE_FILE_NAME: &str = "docker-compose.yaml";

/// Launch
/// 
/// # Description
/// Launch the generate scenario with the command below
/// capoomobi generate <path_to_docker-compose.yaml>
/// e.g: capoomobi generate ./example
/// 
/// # Arguments
/// * `sub_action`: slice of string representing the path
pub fn launch(sub_action: &str) {
  logging::write(
    logging::LogType::Info,
    "Preparing to parse the docker-compose.yml located on the path: ",
    Some(String::from(sub_action))
  );

  let yaml_content = match lexer::yaml_parser::parse(sub_action, COMPOSE_FILE_NAME) {
    Ok(content) => content,
    Err(e) => {
      return logging::write(logging::LogType::Error, e, None);
    }
  };

  let services = match compose::compose::get_docker_service_structure(yaml_content) {
    Ok(vector) => vector,
    Err(e) => {
      return e.log_pretty()
    }
  };

  let prefs = ask_services_details(&services);
  let kubes = tree::tree::get_kube_abstract_tree(services, prefs);
  match io::bootstrap::bootstrap::prepare_kube(&kubes) {
    Ok(()) => io::writer::writer::write_kubernetes_yaml(kubes),
    Err(e) => panic!("error {:?}", e)
  };
}

/// Ask Services Details
/// 
/// # Description
/// Set of questions ask to the user in order to get more
/// information on each docker services
/// 
/// # Arguments
/// * `services` Reference to a Vector of a docker service
/// 
/// # Return
/// HashMap of a hashmap containing answer. The hashmap is mapped like
/// so: [service_foo => [{...}], service_bar => [{...}]]
fn ask_services_details(services: &Vec<compose::compose::Service>) -> HashMap<String, HashMap<&'static str, String>> {
  let mut preferences: HashMap<String, HashMap<&str, String>> = HashMap::new();
  for service in services.into_iter() {
    logging::write(
      logging::LogType::Info,
      format!("{}{}", "Preparing services for: ", service.name).as_str(),
      None
    );

    let replicas = input::get_user_input("Enter number of wishes replicas (e.g: 5)");
    let service_type = input::get_user_input("Enter service type (NodePort, ClusterIP)");
    let nodeport = input::get_user_input("Enter NodePort number if needed (e.g: 30120) or (e.g: N) for no NodePort");
    let controller = input::get_user_input("Enter controller type");

    let mut prefs: HashMap<&str, String> = HashMap::new();
    prefs.insert("replicas", replicas);
    prefs.insert("service", service_type);
    prefs.insert("nodeport", nodeport);
    prefs.insert("controller", controller);

    let name = service.name.to_string();
    preferences.insert(name, prefs);
  }

  return preferences;
}