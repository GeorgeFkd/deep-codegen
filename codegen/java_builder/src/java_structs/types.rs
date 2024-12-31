pub type Implements = TypeName;
#[derive(Debug, Clone)]
pub struct GenericParams {
    pub generics: Vec<String>,
}
impl Eq for TypeName {}
impl PartialEq<Self> for types::TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Hash for TypeName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl GenericParams {
    pub fn new(generics: Vec<String>) -> Self {
        Self { generics }
    }
}
impl Codegen for GenericParams {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        if self.generics.is_empty() {
            return "".to_string();
        } else {
            result.push('<');
        }
        for (pos, generic) in self.generics.iter().enumerate() {
            result.push_str(generic);
            if pos != &self.generics.len() - 1 {
                result.push(',');
            }
        }
        result.push('>');
        result.push(' ');
        result
    }
}

use classes::JavaClass;
use enums::JavaEnum;
use interfaces::Interface;

use super::*;
#[derive(Debug, Clone)]
pub struct TypeName {
    pub name: String,
    pub generic_params: Option<GenericParams>,
}

impl Into<VariableParam> for TypeName {
    fn into(self) -> VariableParam {
        let name = self.name.to_lowercase();
        VariableParam::new(self, name)
    }
}
impl Into<TypeName> for JavaClass {
    fn into(self) -> TypeName {
        if self.generic_params.generics.is_empty() {
            return TypeName::new(self.class_name);
        } else {
            todo!("If a class has generics how can i convert its generics into a typename?");
        }
    }
}

impl Into<TypeName> for Interface {
    fn into(self) -> TypeName {
        if self.generics.generics.is_empty() {
            return TypeName::new(self.name);
        } else {
            todo!("If a class has generics how can i convert its generics into a typename?");
        }
    }
}

impl Into<TypeName> for JavaEnum {
    fn into(self) -> TypeName {
        return TypeName::new(self.enum_name);
    }
}
impl Into<TypeName> for &str {
    fn into(self) -> TypeName {
        TypeName::new(self.to_owned())
    }
}

impl Into<TypeName> for String {
    fn into(self) -> TypeName {
        TypeName::new(self)
    }
}
impl TypeName {
    pub fn new(name: String) -> Self {
        Self {
            name,
            generic_params: None,
        }
    }

    pub fn new_with_generics(name: String, generics: GenericParams) -> Self {
        Self {
            name,
            generic_params: Some(generics),
        }
    }
}

impl Codegen for TypeName {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        result.push_str(&self.name);
        if let Some(generics) = &self.generic_params {
            result.push_str(&generics.generate_code());
        }
        result
    }
}
