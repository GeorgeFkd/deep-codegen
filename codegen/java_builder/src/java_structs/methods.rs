#[derive(Clone)]
pub struct Method {
    pub annotations: Vec<super::annotations::Annotation>,
    pub modifiers: Vec<super::modifiers::AccessModifiers>,
    pub generics: super::types::GenericParams,
    pub parameters: Vec<super::VariableParam>,
    pub return_type: super::types::TypeName,
    pub code: String,
    pub name: String,
    //add throws clause
}
impl super::Codegen for Method {
    fn generate_code(&self) -> String {
        if self
            .modifiers
            .contains(&super::modifiers::AccessModifiers::Abstract)
        {
            assert!(
                &self.code.is_empty(),
                "Abstract methods should not have a body"
            );
        }

        let mut result = "".to_string();

        //reminder: it is valid code to not have modifiers
        //might make it panic to discourage weird code
        result.push_str(&self.modifiers.generate_code());

        result.push_str(&format!("{} ", self.return_type.generate_code()));
        result.push_str(&format!("{}", self.name));
        result.push_str(&self.parameters.generate_code());

        if self
            .modifiers
            .contains(&super::modifiers::AccessModifiers::Abstract)
        {
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
    pub fn new(return_type: super::types::TypeName, name: String) -> Self {
        Self {
            return_type,
            name,
            annotations: vec![],
            generics: super::types::GenericParams::new(vec![]),
            parameters: vec![],
            modifiers: vec![],
            code: "".to_owned(),
        }
    }

    pub fn public(mut self) -> Self {
        self.modifiers
            .push(super::modifiers::AccessModifiers::Public);
        self
    }

    pub fn private(mut self) -> Self {
        self.modifiers
            .push(super::modifiers::AccessModifiers::Private);
        self
    }

    pub fn protected(mut self) -> Self {
        self.modifiers
            .push(super::modifiers::AccessModifiers::Protected);
        self
    }

    pub fn abstract_(mut self) -> Self {
        self.modifiers
            .push(super::modifiers::AccessModifiers::Abstract);
        self
    }

    pub fn static_(mut self) -> Self {
        self.modifiers
            .push(super::modifiers::AccessModifiers::Static);
        self
    }

    pub fn final_(mut self) -> Self {
        self.modifiers
            .push(super::modifiers::AccessModifiers::Final);
        self
    }

    pub fn modifier(mut self, m: super::modifiers::AccessModifiers) -> Self {
        self.modifiers.push(m);
        self
    }

    pub fn code(mut self, s: String) -> Self {
        self.code = s;
        self
    }

    pub fn annotation(mut self, a: super::annotations::Annotation) -> Self {
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
