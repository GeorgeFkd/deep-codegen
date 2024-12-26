use std::hash::{Hash, Hasher};
#[derive(Clone)]
pub struct Annotation {
    pub qualified_name: String,
    //name = value
    pub params_list: Option<Vec<(String, String)>>,
}
impl super::Codegen for Annotation {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        result.push_str(&format!("@{} ", self.qualified_name));
        if let Some(ref params_list) = self.params_list {
            result.push('(');
            result.push('\n');
            for param in params_list {
                result.push_str(&format!("{} = {}\n", param.0, param.1))
            }
            result.push(')');
        }
        result.push('\n');
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
