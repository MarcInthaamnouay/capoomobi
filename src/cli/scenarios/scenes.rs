/// Picker
/// 
/// # Description
/// Module use to retrieve a Scenario from an action
pub mod picker {
    /// Trait to parse to get enum from string
    pub trait EnumHelper<T> {
        /// From String
        /// 
        /// # Description
        /// * `action` &str
        /// 
        /// # Return
        /// Option<T>
        fn from_string(action: &str) -> Option<T>;
    }

    /// Scenarios
    /// 
    /// # Description
    /// Supported main scenarios supported by the CLI (e.g: capoomobi init ...)
    #[derive(Debug)]
    pub enum Scenarios {
        Init,
        Help,
        Generate,
        Project
    }

    /// Helper Scenarios
    /// 
    /// # Description
    /// List of supported sub command after using the help command (e.g: capoomobi help init)
    #[derive(Debug)]
    pub enum HelpScenarios {
        Init,
        Generate,
        Project,
        Global
    }

    impl EnumHelper<Scenarios> for Scenarios {
        fn from_string(action: &str) -> Option<Scenarios> {      
            match action {
                "init" => Some(Scenarios::Init),
                "help" => Some(Scenarios::Help),
                "generate" => Some(Scenarios::Generate),
                "project" => Some(Scenarios::Project),
                _ => None
            }
        }
    }

    impl EnumHelper<HelpScenarios> for HelpScenarios {
        fn from_string(action: &str) -> Option<HelpScenarios> {
            match action {
                "init" => Some(HelpScenarios::Init),
                "generate" => Some(HelpScenarios::Generate),
                "project" => Some(HelpScenarios::Project),
                _ => Some(HelpScenarios::Global)
            }
        }
    }
}
