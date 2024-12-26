
#[derive(Clone)]
pub struct Interface {
    pub annotations: Vec<super::annotations::Annotation>,
    pub package: String,
    pub imports: Vec<super::imports::Import>,
    pub superclass: Option<super::types::TypeName>,
    pub name: String,
    //i need a way to
    pub methods: Vec<super::methods::Method>,
    //abstract should not be used
    //should static be used? it does not make that much sense
    pub modifier: super::modifiers::AccessModifiers,
    //i dont like the GenericParams thing
    //it might be better to just do a Vec<Generic>
    //so it is easy to reference the same generic
    //in different places
    pub generics: super::types::GenericParams,
}
//not using it but is the first macro i wrote with some help
macro_rules! generate_builder_methods_for_enum {
    ($struct_name:ident,$field_name:ident,$enum_name:ident,$($variant:ident)+) => {
    impl $struct_name {
        $(
            pub fn $variant(mut self) -> Self {
                self.$field_name = $enum_name::$variant;
                self
            }
        )*
    }
    }
}
impl super::Codegen for Interface {
    fn generate_code(&self) -> String {
        assert!(
            &self.methods.iter().all(|m| m.code.is_empty()),
            "Interface methods should have an empty body"
        );
        let mut result = "".to_owned();
        result.push_str(&format!("package {};\n", self.package));
        result.push_str(&self.imports.generate_code());
        //todo
        result.push_str(&(vec![self.modifier].generate_code()));
        result.push_str(&format!("interface {} ", self.name));

        if let Some(ref superclass) = self.superclass {
            result.push_str(&format!("extends {}", superclass.name));
            if let Some(ref generics) = superclass.generic_params {
                result.push_str(&generics.generate_code());
            }
            result.push(' ');
        }
        result.push('{');
        result.push('\n');

        for m in &self.methods {
            result.push('\t');
            result.push_str(&m.modifiers.generate_code());
            if !(&m.generics.generics.is_empty()) {
                result.push(' ');
                result.push_str(&m.generics.generate_code());
                result.push(' ');
            }
            result.push_str(&m.return_type.generate_code());
            result.push(' ');
            result.push_str(&m.name);
            result.push_str(&m.parameters.generate_code());
            result.push(';');
            result.push('\n');
        }
        result.push('}');
        println!("{}", result);
        return result;
    }
}

impl Interface {
    pub fn new(package_name: String, interface_name: String) -> Self {
        assert!(
            package_name.contains("."),
            "Interface::new(package,name) is the correct usage, reverse the order of the params"
        );
        Self {
            name: interface_name,
            package: package_name,
            generics: super::types::GenericParams::new(vec![]),
            modifier: super::modifiers::AccessModifiers::Public,
            methods: vec![],
            superclass: None,
            imports: vec![],
            annotations: vec![],
        }
    }
    pub fn modifier(mut self, m: super::modifiers::AccessModifiers) -> Self {
        self.modifier = m;
        self
    }

    pub fn public(mut self) -> Self {
        self.modifier = super::modifiers::AccessModifiers::Public;
        self
    }

    pub fn private(mut self) -> Self {
        self.modifier = super::modifiers::AccessModifiers::Private;
        self
    }

    pub fn protected(mut self) -> Self {
        self.modifier = super::modifiers::AccessModifiers::Protected;
        self
    }

    pub fn abstract_(mut self) -> Self {
        self.modifier = super::modifiers::AccessModifiers::Abstract;
        self
    }

    pub fn static_(mut self) -> Self {
        self.modifier = super::modifiers::AccessModifiers::Static;
        self
    }

    pub fn final_(mut self) -> Self {
        self.modifier = super::modifiers::AccessModifiers::Final;
        self
    }

    pub fn extends(mut self, sup: super::types::TypeName) -> Self {
        self.superclass = Some(sup);
        self
    }

    pub fn methods(mut self, methods: Vec<super::methods::Method>) -> Self {
        self.methods.extend(methods);
        self
    }

    pub fn import(mut self, i: super::imports::Import) -> Self {
        self.imports.push(i);
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
}
