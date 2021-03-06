
/// Builder
///
/// # Description
/// Module helping to generate a K8S struct with the following syntax
/// - Object (Controller)
/// - Service
use std::collections::HashMap;
use std::path::PathBuf;
use crate::docker::parser::{DockerService};
use crate::kubernetes::controllers::controller::{KubeController};
use crate::kubernetes::controllers::service::{KubeService};
use crate::kubernetes::controllers::ingress::{KubeIngress};
use crate::confiture::config::{ConfigConfiture, ConfigIngress};
use crate::core::configurator::config;
use crate::core::fs::toolbox;

/// Structure wrapping the Controller & the Service
#[derive(Debug)]
pub struct Kube {
    pub ctrl: KubeController,
    pub svc: Option<KubeService>,
    pub project_path: PathBuf
}

/// Get Basic Objects
///
/// # Description
/// Retrieve a List of Kube datastructure
///
/// # Arguments
/// * `dk_vec` &[DockerService]
/// * `options` Confiture configuration struct
///
/// # Return
/// * - `Kube vector` List of Kube
pub fn get_basic_objects(dk_vec: &[DockerService], options: HashMap<String, &ConfigConfiture>) -> Vec<Kube> {
    let kube_containers: Vec<Kube> = dk_vec
        .iter()
        .filter_map(|dk| {
            let base_path = get_object_path(&dk.name).unwrap_or_default();
            let option = options.get(&dk.name).unwrap();

            // Create the 2 basic element of a K8S cluster
            // services (networking)
            let svc = KubeService::new(dk.clone(), &option.service, &base_path);
            // controller (workload)
            let ctrl = KubeController::new(dk.clone(), &option.deployment, &base_path);

            ctrl.as_ref()?;
            Some(
                Kube {
                    ctrl: ctrl.unwrap(),
                    svc,
                    project_path: base_path
                }
            )
        })
        .collect();

    kube_containers
}

/// Get Ingress Object
///
/// # Description
/// Get Ingress Object
///
/// # Arguments
/// * `dk` &[DockerService]
/// * `conf` ConfigIngress
///
/// # Return
/// KubeIngress
pub fn get_ingress_object(dk: &[DockerService], conf: Option<ConfigIngress>) -> Option<KubeIngress> {
    if let Some(cnf) = conf {
        return Some(KubeIngress::new(dk, cnf));
    }

    None
}

/// Get Object Path
///
/// # Description
/// Create the destination path of the object based on the setted project path
///
/// # Arguments
/// * `name` &str
///
/// # Return
/// Optional PathBuf
fn get_object_path(name: &str) -> Option<PathBuf> {
    let project_path_opts = config::get_current_project_path();
    project_path_opts.as_ref()?;

    let object_path = project_path_opts.unwrap();
    Some(toolbox::concat_string_path(&object_path, &name))
}
