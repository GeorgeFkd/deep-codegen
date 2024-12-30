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

use super::*;
#[derive(Debug, Clone)]
pub struct TypeName {
    pub name: String,
    pub generic_params: Option<GenericParams>,
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
