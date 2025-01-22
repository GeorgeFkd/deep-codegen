use std::{collections::HashSet, rc::Rc};

use super::{
    annotations::Annotation,
    fields::Field,
    imports::Import,
    methods::Method,
    modifiers::AccessModifiers,
    types::{GenericParams, Implements, TypeName},
    Codegen,
};

#[derive(Clone)]
pub struct JavaClass {
    // modifiers could just be separate methods
    //TODO add constructors
    pub imports: Vec<Import>,
    pub implements: Vec<Implements>,
    pub class_annotations: Vec<Annotation>,
    pub fields: HashSet<Field>,
    pub methods: Vec<Method>,
    pub class_name: String,
    pub generic_params: GenericParams,
    pub class_modifiers: Vec<AccessModifiers>,
    pub superclass: Option<TypeName>,
    pub package: String,
}
impl Codegen for JavaClass {
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
        //println!("Result is: {}", result);
        result
    }
}
impl JavaClass {
    pub fn method(mut self, m: Method) -> Self {
        self.methods.push(m);
        self
    }

    pub fn generic_param(mut self, generic: String) -> Self {
        assert!(!generic.is_empty(), "Empty  Params are not allowed");
        self.generic_params.generics.push(generic);
        self
    }

    pub fn public(mut self) -> Self {
        self.class_modifiers.push(AccessModifiers::Public);
        self
    }

    pub fn private(mut self) -> Self {
        self.class_modifiers.push(AccessModifiers::Private);
        self
    }

    pub fn static_(mut self) -> Self {
        self.class_modifiers.push(AccessModifiers::Static);
        self
    }

    pub fn abstract_(mut self) -> Self {
        self.class_modifiers.push(AccessModifiers::Abstract);
        self
    }

    pub fn final_(mut self) -> Self {
        self.class_modifiers.push(AccessModifiers::Final);
        self
    }
    pub fn protected(mut self) -> Self {
        self.class_modifiers.push(AccessModifiers::Protected);
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
            generic_params: GenericParams::new(vec![]),
        }
    }

    pub fn package_in_place(&mut self, pkg: String) {
        self.package = pkg;
    }

    pub fn package(mut self, pkg: String) -> Self {
        self.package = pkg;
        self
    }

    pub fn class_modifiers(mut self, modifiers: Vec<AccessModifiers>) -> Self {
        self.class_modifiers.append(&mut modifiers.to_owned());
        self
    }

    pub fn class_name(mut self, name: String) -> Self {
        self.class_name = name;
        self
    }

    pub fn extends(mut self, extends: TypeName) -> Self {
        self.superclass = Some(extends);
        self
    }

    pub fn import(mut self, imp: Import) -> Self {
        self.imports.push(imp);
        self
    }

    pub fn imports(mut self, imps: Vec<Import>) -> Self {
        self.imports.extend(imps);
        self
    }

    pub fn field(mut self, f: Field) -> Self {
        self.fields.insert(f);
        self
    }

    pub fn annotation(mut self, a: Annotation) -> Self {
        self.class_annotations.push(a);
        self
    }

    pub fn annotations(mut self, a: Vec<Annotation>) -> Self {
        self.class_annotations.extend(a);
        self
    }
    pub fn implements(mut self, interface: Implements) -> Self {
        self.implements.push(interface);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
