/// Controller
/// 
/// # Path
/// kubernetes/template/controller.rs
/// 
/// # Description
/// Module use to template a K8S controller
use crate::kubernetes::template::helper::common::{TemplateBuilder};

/// Controller Tmpl Builder
/// 
/// # Description
/// Struct use to build the controller template
pub struct ControllerTmplBuilder {}

impl TemplateBuilder for ControllerTmplBuilder {}
