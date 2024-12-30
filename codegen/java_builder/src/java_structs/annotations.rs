use std::hash::{Hash, Hasher};

use super::Codegen;
#[derive(Clone)]
pub struct Annotation {
    pub qualified_name: String,
    //name = value
    pub params_list: Option<Vec<(String, String)>>,
}

impl Annotation {
    pub fn autowired() -> Self {
        Self {
            qualified_name: "Autowired".into(),
            params_list: None,
        }
    }

    pub fn new(qualified_name: String) -> Self {
        Self {
            params_list: None,
            qualified_name,
        }
    }

    pub fn param(mut self, name: String, value: String) -> Self {
        //this is probably not correctly written there is a better way likely
        match self.params_list {
            None => self.params_list = Some(vec![(name, value)]),
            Some(mut params) => {
                params.push((name, value));
                self.params_list = Some(params)
            }
        }
        self
    }

    pub fn params(mut self, name_val_pairs: Vec<(String, String)>) -> Self {
        match self.params_list {
            None => self.params_list = Some(name_val_pairs),
            Some(mut params) => {
                params.extend(name_val_pairs);
                self.params_list = Some(params)
            }
        }
        self
    }
}
impl super::Codegen for Annotation {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();

        result.push('\n');
        result.push_str(&format!("@{} ", self.qualified_name));
        if let Some(ref params_list) = self.params_list {
            result.push('(');
            result.push('\n');
            for param in params_list {
                result.push_str(&format!("{} = {}\n", param.0, param.1))
            }
            result.push(')');
        }
        result
    }
}

impl super::Codegen for Vec<Annotation> {
    fn generate_code(&self) -> String {
        let mut result = "".to_owned();
        for ann in self {
            result.push('\n');
            result.push_str(&ann.generate_code());
        }
        result
    }
}
impl PartialEq<Self> for Annotation {
    fn eq(&self, other: &Self) -> bool {
        self.qualified_name.eq(&other.qualified_name)
    }
}

impl Eq for Annotation {}

impl Hash for Annotation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.qualified_name.hash(state)
    }
}
