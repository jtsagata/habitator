use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EnvChangeRequest {
    #[serde(default = "default_env")]
    pub environment: String,
    #[serde(default = "default_env_dirs")]
    pub before_paths: Vec<String>,
    #[serde(default = "default_env_dirs")]
    pub after_paths: Vec<String>,
    #[serde(default = "default_env_dirs")]
    pub delete_paths: Vec<String>,
}

impl EnvChangeRequest {
    pub fn set_var(&mut self, value: &str) -> &mut Self {
        self.environment = value.into();
        self
    }
    pub fn push_before(&mut self, value: &str) -> &mut Self {
        self.before_paths.push(value.into());
        self
    }
    pub fn push_after(&mut self, value: &str) -> &mut Self {
        self.after_paths.push(value.into());
        self
    }
    pub fn push_delete(&mut self, value: &str) -> &mut Self {
        self.delete_paths.push(value.into());
        self
    }
    pub fn process(&self) -> Vec<String> {
        let env = std::env::var(&self.environment).unwrap_or("".into());
        let os_paths: Vec<String> = env.split(":").map(|p| p.into()).collect();
        let mut paths: Vec<String> = Vec::new();
        // Join the paths
        paths.extend(self.before_paths.clone());
        paths.extend(os_paths.clone());
        paths.extend(self.after_paths.clone());
        // Remove deleted paths and empty paths. Trim the paths
        paths
            .into_iter()
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .filter(|p| !self.delete_paths.contains(p))
            .collect()
    }

    pub fn process_uniq(&self) -> Vec<String> {
        let paths = self.process();
        paths.into_iter().unique().collect()
    }
}

impl Default for EnvChangeRequest {
    fn default() -> Self {
        Self {
            environment: "PATH".into(),
            before_paths: Default::default(),
            after_paths: Default::default(),
            delete_paths: Default::default(),
        }
    }
}

fn default_env() -> String {
    "PATH".to_string()
}

fn default_env_dirs() -> Vec<String> {
    vec![]
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::EnvChangeRequest;
    #[test]
    fn test_default() {
        let empty = EnvChangeRequest::default();
        assert_eq!(empty.environment, "PATH");
        assert_eq!(empty.after_paths.len(), 0);
        assert_eq!(empty.before_paths.len(), 0);
        assert_eq!(empty.delete_paths.len(), 0);
    }

    #[test]
    fn test_ser() {
        env::set_var("PATH", "p1:p2");
        let mut changes = EnvChangeRequest::default();
        changes
            .push_after("l1")
            .push_after("l2")
            .push_before("f1")
            .push_delete("p2");
        let as_ron = ron::to_string(&changes);
        assert_eq!(
            as_ron,
            Ok("(environment:\"PATH\",before_paths:[\"f1\"],after_paths:[\"l1\",\"l2\"],delete_paths:[\"p2\"])".to_string()));
    }

    #[test]
    fn test_deserialize() {
        let ron = r#"(
        environment: "PATH",

        before_paths: [
            "a", 
            "b"
        ],
        
        after_paths: [
            "c", 
            "d"
        ],

        delete_paths: ["e", "f"]
        )"#;

        let changes: EnvChangeRequest = ron::from_str(ron).unwrap();
        println!("{:?}", changes);
        assert_eq!(
            changes,
            EnvChangeRequest {
                environment: "PATH".into(),
                before_paths: vec!["a".into(), "b".into()],
                after_paths: vec!["c".into(), "d".into()],
                delete_paths: vec!["e".into(), "f".into()],
            }
        );
    }

    #[test]
    fn test_deserialize_default() {
        let ron = r#"(
        before_paths: [
            "a", 
            "b"
        ]
        )"#;

        let changes: EnvChangeRequest = ron::from_str(ron).unwrap();
        println!("{:?}", changes);
        assert_eq!(
            changes,
            EnvChangeRequest {
                environment: "PATH".into(),
                before_paths: vec!["a".into(), "b".into()],
                after_paths: vec![],
                delete_paths: vec![],
            }
        );
    }

    #[test]
    fn test_populate() {
        env::set_var("PATH", "p1:p2");
        let mut changes = EnvChangeRequest::default();
        let changes = changes
            .push_after("l1")
            .push_after("l2")
            .push_before("f1")
            .push_delete("p2");
        let path_txt = changes.process();
        assert_eq!(path_txt.join(":"), "f1:p1:l1:l2".to_string());
    }

    #[test]
    fn test_populate_uniq() {
        env::set_var("PATH", "p1:p2:p1:p2");
        let changes = EnvChangeRequest::default();
        let path_txt = changes.process_uniq();
        assert_eq!(path_txt.join(":"), "p1:p2".to_string());
    }

    #[test]
    fn test_populate_empty_var() {
        env::set_var("PATH", "");
        let mut changes = EnvChangeRequest::default();
        let changes = changes
            .push_after("l1")
            .push_after("l2")
            .push_before("f1")
            .push_delete("p2")
            .set_var("__HABITATOR__");
        let path_txt = changes.process();
        assert_eq!(path_txt.join(":"), "f1:l1:l2".to_string());
        assert_eq!(changes.environment, "__HABITATOR__".to_string())
    }

    #[test]
    fn test_populate_skip_empty() {
        env::set_var("PATH", "p1::p2");
        let changes = EnvChangeRequest::default();
        let path_txt = changes.process();
        assert_eq!(path_txt.join(":"), "p1:p2".to_string());
    }

    #[test]
    fn test_populate_strip1() {
        env::set_var("PATH", "p1: p2 :p3");
        let changes = EnvChangeRequest::default();
        let path_txt = changes.process();
        assert_eq!(path_txt.join(":"), "p1:p2:p3".to_string());
    }

    #[test]
    fn test_populate_strip2() {
        env::set_var("PATH", "My Documents: p2 :p3");
        let changes = EnvChangeRequest::default();
        let path_txt = changes.process();
        assert_eq!(path_txt.join(":"), "My Documents:p2:p3".to_string());
    }
}
