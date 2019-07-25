/**
 * Container
 * 
 * Module use to create a container of a T kubernetes controller
 */
pub mod container {
  use std::collections::HashMap;
  use std::collections::BTreeMap;
  use std::path::PathBuf;
  use serde::Serialize;
  use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper};
  use crate::docker::lexer::compose::compose::{Service};
  use crate::kubernetes::controllers::common::{KubeHelper};
  use crate::cli::core::fs::operations::toolbox;

  // Constant
  const CONTROLLER_FILENAME: &str = "controller.yaml";
  const SERVICE_FILENAME: &str = "service.yaml";


  /**
   * List of supported K8S controllers
   * by the generator
   */
  #[derive(Debug)]
  #[derive(Serialize)]
  pub enum ControllerKind {
    Deployment,
    ReplicaSet,
    StatefulSet,
    DaemonSet
  }

  /**
   * Parse string to enum ControllerKind deployment
   */
  impl EnumHelper<ControllerKind> for ControllerKind {
    fn from_str(controller: &str) -> Option<ControllerKind> {
      match controller {
        "deployment" => Some(ControllerKind::Deployment),
        "replicaset" => Some(ControllerKind::ReplicaSet),
        "statefulset" => Some(ControllerKind::StatefulSet),
        "daemonset" => Some(ControllerKind::DaemonSet),
        _ => Some(ControllerKind::Deployment)
      }
    }
  }
  
  /**
   * Kube Container
   * 
   * Structure representing a kubernetes container
   */
  #[derive(Debug)]
  #[derive(Serialize)]
  pub struct KubeContainer {
    pub controller_type: ControllerKind,
    pub name: String,
    pub image: String,
    pub replicas: u8,
    pub path: PathBuf,
    pub controller_path: PathBuf,
    pub service_path: PathBuf,
    pub commands: Vec<String>,
    pub labels: Vec<String>,
    pub environement: Vec<String>,
  }

  impl KubeHelper<&'static str, String> for KubeContainer {
    fn get_tree_map(&self) -> BTreeMap<&'static str, String> {
      let mut tree = BTreeMap::new();
      tree.insert("name", String::from(&self.name));
      tree.insert("image", String::from(&self.image));

      return tree;
    }
  }

  /**
   * Create Kube Struct
   * 
   * Create a KubeContainer structure
   */
  pub fn create_kube_struct(docker_service: Service, option: &HashMap<&str, String>) -> KubeContainer {
    let mut controller_kind: ControllerKind = ControllerKind::Deployment;
    if let Some(controller) = option.get("controller") {
      controller_kind = ControllerKind::from_str(controller.to_lowercase().as_str()).unwrap();
    }

    let mut replica_count: u8 = 3;
    if let Some(replicas) = option.get("replicas") {
      replica_count = replicas.parse::<u8>().unwrap_or(3);
    }

    let base_path = toolbox::get_kube_path_for_service(docker_service.name).unwrap_or(PathBuf::new());
    let mut controller_path = PathBuf::from(&base_path);
    controller_path.push(CONTROLLER_FILENAME);

    let mut service_path = PathBuf::from(&base_path);
    service_path.push(SERVICE_FILENAME);

    let kube_container = KubeContainer {
      controller_type: controller_kind,
      name: docker_service.name,
      image: docker_service.image,
      replicas: replica_count,
      commands: docker_service.commands,
      labels: docker_service.labels,
      environement: docker_service.environment,
      // Paths
      path: base_path,
      controller_path: controller_path,
      service_path: service_path,
    };

    return kube_container;
  }
}