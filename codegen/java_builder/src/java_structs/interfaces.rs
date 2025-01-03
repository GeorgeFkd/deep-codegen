use super::{
    annotations::Annotation,
    classes::JavaClass,
    imports::Import,
    methods::Method,
    modifiers::AccessModifiers,
    types::{GenericParams, TypeName},
    Codegen,
};

#[derive(Clone)]
pub struct Interface {
    pub annotations: Vec<Annotation>,
    pub package: String,
    pub imports: Vec<Import>,
    pub superclass: Option<TypeName>,
    pub name: String,
    pub methods: Vec<Method>,
    pub modifier: AccessModifiers,
    pub generics: GenericParams,
}
//not using it but is the first macro i wrote with some help
macro_rules! _generate_builder_methods_for_enum {
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

impl Into<JavaClass> for Interface {
    fn into(self) -> JavaClass {
        let mut c = JavaClass::new(self.name.clone() + "Impl", self.package.clone());
        c = c.implements(self.clone().into());
        for mut m in self.methods.into_iter() {
            m.modifiers = vec![];
            m = m.public();
            c = c.method(m);
        }
        c
    }
}
impl Codegen for Interface {
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
    pub fn package(mut self, pkg: String) -> Self {
        self.package = pkg;
        self
    }

    pub fn package_in_place(&mut self, pkg: String) {
        self.package = pkg;
    }

    pub fn new(package_name: String, interface_name: String) -> Self {
        // assert!(
        //     package_name.contains("."),
        //     "Interface::new(package,name) is the correct usage, reverse the order of the params"
        // );
        Self {
            name: interface_name,
            package: package_name,
            generics: GenericParams::new(vec![]),
            modifier: AccessModifiers::Public,
            methods: vec![],
            superclass: None,
            imports: vec![],
            annotations: vec![],
        }
    }
    pub fn modifier(mut self, m: AccessModifiers) -> Self {
        self.modifier = m;
        self
    }

    pub fn public(mut self) -> Self {
        self.modifier = AccessModifiers::Public;
        self
    }

    pub fn private(mut self) -> Self {
        self.modifier = AccessModifiers::Private;
        self
    }

    pub fn protected(mut self) -> Self {
        self.modifier = AccessModifiers::Protected;
        self
    }

    pub fn abstract_(mut self) -> Self {
        self.modifier = AccessModifiers::Abstract;
        self
    }

    pub fn static_(mut self) -> Self {
        self.modifier = AccessModifiers::Static;
        self
    }

    pub fn final_(mut self) -> Self {
        self.modifier = AccessModifiers::Final;
        self
    }

    pub fn extends(mut self, sup: TypeName) -> Self {
        self.superclass = Some(sup);
        self
    }

    pub fn methods(mut self, methods: Vec<Method>) -> Self {
        self.methods.extend(methods);
        self
    }

    pub fn method(mut self, m: Method) -> Self {
        self.methods.push(m);
        self
    }

    pub fn import(mut self, i: Import) -> Self {
        self.imports.push(i);
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
}
