pub mod imports;

pub mod annotations;
pub mod enums;
pub mod fields;
pub mod interfaces;
pub mod types;

pub mod classes;
pub mod methods;
pub mod modifiers;
use std::hash::{Hash, Hasher};
//TODO implement builders
//and put the required things on the new call
pub trait Codegen {
    fn generate_code(&self) -> String;
}

#[derive(Clone)]
pub struct VariableParam {
    pub name: String,
    pub type_: types::TypeName,
    pub annotation: Vec<annotations::Annotation>,
}
impl Codegen for Vec<VariableParam> {
    fn generate_code(&self) -> String {
        let mut result = "".to_owned();
        result.push('(');
        for (pos, param) in self.iter().enumerate() {
            for ann in param.annotation.iter() {
                result.push_str(ann.generate_code().as_str());
            }
            result.push_str(&format!("{} {}", param.type_.generate_code(), param.name));
            if pos != self.len() - 1 {
                result.push(',');
            }
        }
        result.push(')');
        result
    }
}

impl Codegen for Vec<types::Implements> {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        result.push_str("implements ");

        for (pos, elem) in self.iter().enumerate() {
            result.push_str(&elem.generate_code());
            // }

            if pos != self.len() - 1 {
                result.push_str(", ");
            }
        }
        result.push(' ');
        result
    }
}
