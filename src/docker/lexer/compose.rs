/**
 * Compose
 * 
 * List of struct defining the docker-compose file
 */
pub mod compose {
  use yaml_rust::{yaml};
  use std::collections::HashMap;
  use crate::errors::cli_error::{CliErr, ErrorHelper, ErrCode};

  // constant error
  const EMPTY_YAML_CONTENT_ERROR: &str = "Unable to parse empty content of docker-compose.yaml file";
  const SVC_NOT_ARRAY_TYPE_ERROR: &str = "Services attribute is not an array";

  // Service represent a service in the compose file
  // e.g services.portainer
  #[derive(Debug)]
  pub struct Service {
    pub name: String,
    pub image: String,
    pub commands: Vec<String>,
    pub ports: Vec<String>,
    pub labels: Vec<String>,
    pub environment: Vec<String>,
    pub volumes: Vec<String>
  }

  // Enumeration Field Kind
  // Use to choice which type of field to filter
  enum FieldKind {
    SingleField,
    ArrayField
  }

  /**
   * Get Supported attributes
   * 
   * Return a vector of the supported list of attributes
   */
  fn get_supported_attributes(field: FieldKind) -> Vec<&'static str> {
    match field {
      FieldKind::SingleField => {
        return vec![
          "image",
          "command",
          "labels",
        ];
      },
      FieldKind::ArrayField => {
        return vec![
          "command",
          "ports",
          "labels",
          "environment",
          "volumes",
        ];
      }
    };
  }

  /**
   * Get docker service structure
   * 
   * Generate a struct containing which represent the content
   * of a docker-compose file
   */
  pub fn get_docker_service_structure(content: Vec<yaml::Yaml>) -> Result<Vec<Service>, CliErr> {
    if content.is_empty() {
      return Err(
        CliErr::new(
          EMPTY_YAML_CONTENT_ERROR,
          "",
          ErrCode::NotFound
        )
      );
    }

    let compose_content  = &content[0];
    let services_content = compose_content["services"].to_owned();
    let services_hash = services_content.into_hash();

    if let Some(hashes) = services_hash {
      let iter = hashes.into_iter();

      let services: Vec<Service> = iter
        .map(|yaml| get_service(yaml.0, yaml.1))
        .collect();

      return Ok(services);
    }

    Err(
      CliErr::new(
        SVC_NOT_ARRAY_TYPE_ERROR,
        "",
        ErrCode::ParsingError
      )
    )
  }


  /**
   * Get Service
   * 
   * Get attribute value for each services
   */
  fn get_service(service_name: yaml::Yaml, yaml_service: yaml::Yaml) -> Service {
    let str_field_vec: Vec<&str> = get_supported_attributes(FieldKind::SingleField)
        .into_iter()
        .map(|key| yaml_service[key].as_str().unwrap_or(""))
        .collect();

    let mut array_attributes = HashMap::new();
    let attributes = get_supported_attributes(FieldKind::ArrayField);
    let empty_vec = vec![String::from("")];

    // @TODO refactor for using BTree instead
    for attr in attributes.into_iter() {
      let vec = yaml_service[attr].as_vec();
      if let Some(array) = vec {
        let str_vec_fields: Vec<String> = array
          .into_iter()
          .map(|value| value.as_str().unwrap_or(""))
          .map(|each| String::from(each))
          .collect();
        
        array_attributes.insert(attr, str_vec_fields);        
      } else {
        array_attributes.insert(attr, Vec::new());
      }
    }

    let fallback_cmd = vec![String::from(str_field_vec[1])];
    let fallback_label = vec![String::from(str_field_vec[2])];
    
    println!("value of fallback_label {:?}", fallback_label);

    Service {
      name: String::from(service_name.as_str().unwrap_or("unknown")),
      // Single line field
      // @TODO Put these fields as the unwrap_or values
      image: String::from(str_field_vec[0]),
      // Array fields
      commands: array_attributes.get("command").unwrap_or(&fallback_cmd).to_vec(),
      ports: array_attributes.get("ports").unwrap_or(&empty_vec).to_vec(),
      labels: array_attributes.get("labels").unwrap_or(&fallback_label).to_vec(),
      environment: array_attributes.get("environment").unwrap_or(&empty_vec).to_vec(),
      volumes: array_attributes.get("volumes").unwrap_or(&empty_vec).to_vec(),
    }
  }
}