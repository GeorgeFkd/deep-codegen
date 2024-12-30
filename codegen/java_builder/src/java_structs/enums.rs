use super::{imports::Import, modifiers::AccessModifiers, Codegen};

pub struct JavaEnum {
    enum_types: Vec<(String, String)>,
    enum_name: String,
    modifiers: Vec<AccessModifiers>,
    package: String,
    imports: Vec<Import>,
}
impl Codegen for JavaEnum {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        result.push_str(&format!("package {};\n", &self.package));
        result.push_str(&self.imports.generate_code());
        result.push('\n');

        result.push_str(&self.modifiers.generate_code());

        result.push_str(&format!("enum {} {{ \n", self.enum_name));
        for (position, (enum_type_name, enum_type_value)) in self.enum_types.iter().enumerate() {
            result.push_str(&format!("\t{}({})", enum_type_name, enum_type_value));
            if position != &self.enum_types.len() - 1 {
                result.push(',');
            } else {
                result.push(';');
            }
            result.push('\n');
        }
        result.push('\n');
        result.push('}');
        result
    }
}

impl JavaEnum {
    pub fn new(enum_name: String, package_name: String) -> Self {
        JavaEnum {
            enum_types: vec![],
            modifiers: vec![],
            imports: vec![],
            package: package_name,
            enum_name,
        }
    }
    pub fn types(mut self, enum_types: Vec<(String, String)>) -> Self {
        self.enum_types.extend(enum_types);
        self
    }

    pub fn public(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Public);
        self
    }
    pub fn protected(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Protected);
        self
    }

    pub fn private(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Private);
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

    pub fn abstract_(mut self) -> Self {
        self.modifiers.push(AccessModifiers::Abstract);
        self
    }

    pub fn modifiers(mut self, modifiers: Vec<AccessModifiers>) -> Self {
        self.modifiers.extend(modifiers);
        self
    }

    pub fn imports(mut self, imports: Vec<Import>) -> Self {
        self.imports.extend(imports);
        self
    }
}
