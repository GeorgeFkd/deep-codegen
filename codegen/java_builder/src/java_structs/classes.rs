use std::collections::HashSet;

use crate::java_structs::Codegen;

#[derive(Clone)]
pub struct JavaClass {
    // modifiers could just be separate methods
    //TODO add constructors
    pub imports: Vec<super::imports::Import>,
    pub implements: Vec<super::types::Implements>,
    pub class_annotations: Vec<super::annotations::Annotation>,
    pub fields: HashSet<super::fields::Field>,
    pub methods: Vec<super::methods::Method>,
    pub class_name: String,
    pub generic_params: super::types::GenericParams,
    pub class_modifiers: Vec<super::modifiers::AccessModifiers>,
    pub superclass: Option<super::types::TypeName>,
    pub package: String,
}
impl super::Codegen for JavaClass {
    fn generate_code(&self) -> String {
        //i could refactor to more immutability in this method
        let mut result: String = "".to_string();
        result.push_str(&format!("package {};\n", self.package));
        result.push_str("\n");

        if self.imports.is_empty() {
            println!("No imports found you might have forgotten them");
        }
        result.push_str(self.imports.generate_code().as_str());

        result.push_str("\n");

        if self.class_modifiers.is_empty() {
            println!("No class modifiers you might want to make your class public");
        }
        result.push_str(&self.class_annotations.generate_code());
        result.push_str(&self.class_modifiers.generate_code());

        result.push_str(&format!("class {}", self.class_name));
        result.push_str(&self.generic_params.generate_code());

        if let Some(ref superclass) = self.superclass {
            result.push_str(&format!("extends {}", superclass.name));
            if let Some(ref generics) = superclass.generic_params {
                result.push_str(&generics.generate_code());
            }
            result.push(' ');
        }

        result.push_str(self.implements.generate_code().as_str());

        result.push('{');
        result.push('\n');
        for field in self.fields.iter() {
            result.push_str(field.generate_code().as_str());
        }

        for method in self.methods.iter() {
            result.push_str(&method.generate_code());
        }
        result.push_str("\n}\n");
        println!("Result is: {}", result);
        result
    }
}
impl JavaClass {
    pub fn method(mut self, m: super::methods::Method) -> Self {
        self.methods.push(m);
        self
    }

    pub fn generic_param(mut self, generic: String) -> Self {
        assert!(!generic.is_empty(), "Empty  Params are not allowed");
        self.generic_params.generics.push(generic);
        self
    }

    pub fn public(mut self) -> Self {
        self.class_modifiers
            .push(super::modifiers::AccessModifiers::Public);
        self
    }

    pub fn private(mut self) -> Self {
        self.class_modifiers
            .push(super::modifiers::AccessModifiers::Private);
        self
    }

    pub fn static_(mut self) -> Self {
        self.class_modifiers
            .push(super::modifiers::AccessModifiers::Static);
        self
    }

    pub fn abstract_(mut self) -> Self {
        self.class_modifiers
            .push(super::modifiers::AccessModifiers::Abstract);
        self
    }

    pub fn final_(mut self) -> Self {
        self.class_modifiers
            .push(super::modifiers::AccessModifiers::Final);
        self
    }
    pub fn protected(mut self) -> Self {
        self.class_modifiers
            .push(super::modifiers::AccessModifiers::Protected);
        self
    }
    pub fn new(class_name: String, package: String) -> JavaClass {
        //package can be empty as it might change for the codegen process
        assert!(
            !class_name.is_empty(),
            "You forgot to include the class name"
        );
        JavaClass {
            imports: vec![],
            class_name,
            superclass: None,
            class_annotations: vec![],
            class_modifiers: vec![],
            implements: vec![],
            fields: HashSet::new(),
            package,
            methods: vec![],
            generic_params: super::types::GenericParams::new(vec![]),
        }
    }

    pub fn package(mut self, pkg: String) -> Self {
        self.package = pkg;
        self
    }

    pub fn class_modifiers(mut self, modifiers: Vec<super::modifiers::AccessModifiers>) -> Self {
        self.class_modifiers.append(&mut modifiers.to_owned());
        self
    }

    pub fn class_name(mut self, name: String) -> Self {
        self.class_name = name;
        self
    }

    pub fn extends(mut self, extends: super::types::TypeName) -> Self {
        self.superclass = Some(extends);
        self
    }

    pub fn import(mut self, imp: super::imports::Import) -> Self {
        self.imports.push(imp);
        self
    }

    pub fn imports(mut self, imps: Vec<super::imports::Import>) -> Self {
        self.imports.extend(imps);
        self
    }

    pub fn field(mut self, f: super::fields::Field) -> Self {
        self.fields.insert(f);
        self
    }

    pub fn annotation(mut self, a: super::annotations::Annotation) -> Self {
        self.class_annotations.push(a);
        self
    }

    pub fn annotations(mut self, a: Vec<super::annotations::Annotation>) -> Self {
        self.class_annotations.extend(a);
        self
    }
    pub fn implements(mut self, interface: super::types::Implements) -> Self {
        self.implements.push(interface);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
