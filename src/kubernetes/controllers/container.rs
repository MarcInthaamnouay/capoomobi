pub mod container {
  use std::collections::HashMap;
  use crate::cli::scenarios::scenes::scenes_helper::{EnumHelper};
  use crate::docker::lexer::compose::compose::{Service};

  /**
   * List of supported K8S controllers
   * by the generator
   */
  #[derive(Debug)]
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
    fn from_str(controller: &str) -> Result<ControllerKind, &'static str> {
      match controller {
        "deployment" => Ok(ControllerKind::Deployment),
        "replicaset" => Ok(ControllerKind::ReplicaSet),
        "statefulset" => Ok(ControllerKind::StatefulSet),
        "daemonset" => Ok(ControllerKind::DaemonSet),
        _ => Ok(ControllerKind::Deployment)
      }
    }
  }
  
  /**
   * Kube Container
   * 
   * Structure representing a kubernetes container
   */
  #[derive(Debug)]
  pub struct KubeContainer {
    controller_type: ControllerKind,
    name: String,
    image: String,
    replicas: u8,
    commands: Vec<String>,
    labels: Vec<String>,
    environement: Vec<String>,
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

    let kube_container = KubeContainer {
      controller_type: controller_kind,
      name: docker_service.name,
      image: docker_service.image,
      replicas: replica_count,
      commands: docker_service.commands,
      labels: docker_service.labels,
      environement: docker_service.environment
    };

    return kube_container;
  }
}