use super::annotations::Annotation;
use super::modifiers::AccessModifiers;
use super::types::{GenericParams, TypeName};
use super::Codegen;

#[derive(Clone)]
pub struct Method {
    pub annotations: Vec<Annotation>,
    pub modifiers: Vec<AccessModifiers>,
    pub generics: GenericParams,
    pub parameters: Vec<super::VariableParam>,
    pub return_type: TypeName,
    pub code: String,
    pub name: String,
    //add throws clause
}

#[derive(Clone)]
pub struct VariableParam {
    pub name: String,
    pub type_: TypeName,
    pub annotation: Vec<Annotation>,
}

impl VariableParam {
    pub fn new(type_: TypeName, name: String) -> Self {
        Self {
            name,
            type_,
            annotation: vec![],
        }
    }
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
impl Codegen for Method {
    fn generate_code(&self) -> String {
        if self.modifiers.contains(&AccessModifiers::Abstract) {
            assert!(
                &self.code.is_empty(),
                "Abstract methods should not have a body"
            );
        }

        let mut result = "".to_string();

        //reminder: it is valid code to not have modifiers
        //might make it panic to discourage weird code
        result.push_str(&self.annotations.generate_code());
        result.push_str(&self.modifiers.generate_code());

        result.push_str(&format!("{} ", self.return_type.generate_code()));
        result.push_str(&format!("{}", self.name));
        result.push_str(&self.parameters.generate_code());

        if self.modifiers.contains(&AccessModifiers::Abstract) {
            result.push(';');
            result.push('\n');
            return result;
        }
        result.push('{');
        if self.code.is_empty() {
            result.push('}');
        } else {
            result.push('\n');
            for line in self.code.lines() {
                result.push_str(&format!("\t{}\n", line));
            }
            result.push('}');
        }
        result.push('\n');
        result
    }
}
impl Method {
    pub fn new(return_type: TypeName, name: String) -> Self {
        Self {
            return_type,
            name,
            annotations: vec![],
            generics: GenericParams::new(vec![]),
            parameters: vec![],
            modifiers: vec![],
            code: "".to_owned(),
        }
    }

    pub fn public(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Public);
        self
    }

    pub fn private(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Private);
        self
    }

    pub fn protected(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Protected);
        self
    }

    pub fn abstract_(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Abstract);
        self
    }

    pub fn static_(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Static);
        self
    }

    pub fn final_(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Final);
        self
    }

    pub fn modifier(mut self, m: AccessModifiers) -> Self {
        self.modifiers.push(m);
        self
    }

    pub fn code(mut self, s: String) -> Self {
        self.code = s;
        self
    }

    pub fn annotation(mut self, a: Annotation) -> Self {
        self.annotations.push(a);
        self
    }

    pub fn generic_param(mut self, g: String) -> Self {
        self.generics.generics.push(g);
        self
    }

    pub fn param(mut self, v: super::VariableParam) -> Self {
        self.parameters.push(v);
        self
    }
}
