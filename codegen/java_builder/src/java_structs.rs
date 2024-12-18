use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

//TODO implement builders
//and put the required things on the new call
//
//
//TODO retain only the logic in lib.rs and bring the struct stuff here
//(JavaClass Builder)
//write more macros so that the API is the same across structs.
//
pub trait Codegen {
    fn generate_code(&self) -> String;
}
#[derive(Hash, Eq, Clone)]
pub struct Field {
    //might be empty but we dont care
    pub annotation: Vec<Annotation>,
    //i want to make this a hashset to avoid duplicates but i dont think someone would
    //accidentally input duplicate stuff
    pub modifiers: Vec<AccessModifiers>,
    pub name: String,
    pub type_: TypeName,
    //this type can be stricter
    pub initializer: Option<String>,
}

pub type Implements = TypeName;

#[derive(Clone)]
pub struct VariableParam {
    pub name: String,
    pub type_: TypeName,
    pub annotation: Vec<Annotation>,
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
pub mod imports {
    #[derive(Clone)]
    pub struct Import {
        //import org.codegen.package.class_name
        pub class_name: String,
        pub package_name: String,
        //import static org.codegen.package.class_name
        pub static_import: bool,
    }

    impl Import {
        pub fn new(package_name: String, class_name: String) -> Self {
            assert!(package_name.contains("."),"Package name does not have dots, the params in the ::new method are the other way around");
            Self {
                class_name,
                package_name,
                static_import: false,
            }
        }

        pub fn static_(mut self) -> Self {
            self.static_import = true;
            self
        }
    }
    impl super::Codegen for Vec<Import> {
        fn generate_code(&self) -> String {
            let mut result = "".to_string();
            result.push('\n');
            for import in self.iter() {
                result.push_str(&*import.generate_code());
            }
            result
        }
    }

    impl super::Codegen for Import {
        fn generate_code(&self) -> String {
            match &self.static_import {
                false => format!("import {}.{};\n", self.package_name, self.class_name),
                true => format!("import static {}.{};\n", self.package_name, self.class_name),
            }
        }
    }
}
impl Codegen for Vec<Implements> {
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
impl Into<String> for AccessModifiers {
    fn into(self) -> String {
        match self {
            AccessModifiers::Public => "public".to_owned(),
            AccessModifiers::Private => "private".to_owned(),
            AccessModifiers::Protected => "protected".to_owned(),
            AccessModifiers::Static => "static".to_owned(),
            AccessModifiers::Abstract => "abstract".to_owned(),
            AccessModifiers::Final => "final".to_owned(),
        }
    }
}
impl Codegen for Vec<AccessModifiers> {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        let mut modifiers = self.clone();
        modifiers.sort_by(|a, b| b.cmp(a));
        //more rules for modifiers
        assert!(
            !(modifiers.contains(&AccessModifiers::Public)
                && modifiers.contains(&AccessModifiers::Protected)),
            "Modifiers {:?} and {:?} should not be used together",
            &AccessModifiers::Public,
            &AccessModifiers::Protected
        );
        assert!(
            !(modifiers.contains(&AccessModifiers::Protected)
                && modifiers.contains(&AccessModifiers::Private)),
            "Modifiers {:?} and {:?} should not be used together",
            &AccessModifiers::Protected,
            &AccessModifiers::Private
        );
        assert!(
            !(modifiers.contains(&AccessModifiers::Public)
                && modifiers.contains(&AccessModifiers::Private)),
            "Modifiers {:?} and {:?} should not be used together",
            &AccessModifiers::Public,
            &AccessModifiers::Private
        );

        modifiers.dedup();
        for m in modifiers.iter() {
            result.push_str(&format!("{} ", <AccessModifiers as Into<String>>::into(*m)));
        }
        result
    }
}
pub mod interfaces {

    #[derive(Clone)]
    pub struct Interface {
        pub annotations: Vec<super::Annotation>,
        pub package: String,
        pub imports: Vec<super::imports::Import>,
        pub superclass: Option<super::TypeName>,
        pub name: String,
        //i need a way to
        pub methods: Vec<super::methods::Method>,
        //abstract should not be used
        //should static be used? it does not make that much sense
        pub modifier: super::AccessModifiers,
        //i dont like the GenericParams thing
        //it might be better to just do a Vec<Generic>
        //so it is easy to reference the same generic
        //in different places
        pub generics: super::GenericParams,
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
                generics: super::GenericParams::new(vec![]),
                modifier: super::AccessModifiers::Public,
                methods: vec![],
                superclass: None,
                imports: vec![],
                annotations: vec![],
            }
        }
        pub fn modifier(mut self, m: super::AccessModifiers) -> Self {
            self.modifier = m;
            self
        }

        pub fn public(mut self) -> Self {
            self.modifier = super::AccessModifiers::Public;
            self
        }

        pub fn private(mut self) -> Self {
            self.modifier = super::AccessModifiers::Private;
            self
        }

        pub fn protected(mut self) -> Self {
            self.modifier = super::AccessModifiers::Protected;
            self
        }

        pub fn abstract_(mut self) -> Self {
            self.modifier = super::AccessModifiers::Abstract;
            self
        }

        pub fn static_(mut self) -> Self {
            self.modifier = super::AccessModifiers::Static;
            self
        }

        pub fn final_(mut self) -> Self {
            self.modifier = super::AccessModifiers::Final;
            self
        }

        pub fn extends(mut self, sup: super::TypeName) -> Self {
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

        pub fn annotation(mut self, a: super::Annotation) -> Self {
            self.annotations.push(a);
            self
        }

        pub fn generic_param(mut self, g: String) -> Self {
            self.generics.generics.push(g);
            self
        }
    }
}

pub mod methods {
    #[derive(Clone)]
    pub struct Method {
        pub annotations: Vec<super::Annotation>,
        pub modifiers: Vec<super::AccessModifiers>,
        pub generics: super::GenericParams,
        pub parameters: Vec<super::VariableParam>,
        pub return_type: super::TypeName,
        pub code: String,
        pub name: String,
        //add throws clause
    }
    impl super::Codegen for Method {
        fn generate_code(&self) -> String {
            if self.modifiers.contains(&super::AccessModifiers::Abstract) {
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

            if self.modifiers.contains(&super::AccessModifiers::Abstract) {
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
        pub fn new(return_type: super::TypeName, name: String) -> Self {
            Self {
                return_type,
                name,
                annotations: vec![],
                generics: super::GenericParams::new(vec![]),
                parameters: vec![],
                modifiers: vec![],
                code: "".to_owned(),
            }
        }

        pub fn public(mut self) -> Self {
            self.modifiers.push(super::AccessModifiers::Public);
            self
        }

        pub fn private(mut self) -> Self {
            self.modifiers.push(super::AccessModifiers::Private);
            self
        }

        pub fn protected(mut self) -> Self {
            self.modifiers.push(super::AccessModifiers::Protected);
            self
        }

        pub fn abstract_(mut self) -> Self {
            self.modifiers.push(super::AccessModifiers::Abstract);
            self
        }

        pub fn static_(mut self) -> Self {
            self.modifiers.push(super::AccessModifiers::Static);
            self
        }

        pub fn final_(mut self) -> Self {
            self.modifiers.push(super::AccessModifiers::Final);
            self
        }

        pub fn modifier(mut self, m: super::AccessModifiers) -> Self {
            self.modifiers.push(m);
            self
        }

        pub fn code(mut self, s: String) -> Self {
            self.code = s;
            self
        }

        pub fn annotation(mut self, a: super::Annotation) -> Self {
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
}

pub struct JavaClass {
    // modifiers could just be separate methods
    pub imports: Option<Vec<self::imports::Import>>,
    pub implements: Option<Vec<Implements>>,
    pub class_annotations: Option<Vec<Annotation>>,
    pub fields: HashSet<Field>,
    pub methods: Vec<self::methods::Method>,
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

        if let Some(ref imports) = self.imports {
            result.push_str(imports.generate_code().as_str());
        } else {
            println!("No imports found you might have forgotten them");
        }

        result.push_str("\n");

        if self.class_modifiers.is_empty() {
            println!("No class modifiers you might want to make your class public");
        }

        result.push_str(self.class_modifiers.generate_code().as_str());

        result.push_str(&format!("class {}", self.class_name));
        result.push_str(&self.generic_params.generate_code());

        if let Some(ref superclass) = self.superclass {
            result.push_str(&format!("extends {}", superclass.name));
            if let Some(ref generics) = superclass.generic_params {
                result.push_str(&generics.generate_code());
            }
            result.push(' ');
        }

        if let Some(ref implements) = self.implements {
            result.push_str(implements.generate_code().as_str());
        }

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
impl Codegen for Field {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        //todo run a java formatter after generation
        //i do some basic formatting so it is not unreadable
        result.push_str("    ");
        for annotation in self.annotation.iter() {
            result.push_str(annotation.generate_code().as_str());
        }
        result.push_str("    ");
        let mut sorted_modifiers = self.modifiers.to_owned();
        sorted_modifiers.sort_by(|a, b| b.cmp(a));
        for m in sorted_modifiers {
            result.push_str(&format!("{} ", <AccessModifiers as Into<String>>::into(m)));
        }
        result.push_str(&format!("{} ", self.type_.generate_code()));
        result.push_str(&format!("{};\n", self.name));

        if let Some(ref init) = self.initializer {
            result.push_str(&format!("= {}", init));
        }
        result
    }
}
impl JavaClass {
    pub fn method(mut self, m: self::methods::Method) -> Self {
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
        assert!(!package.is_empty(),"You forgot to include the package declaration, on the Builder object you can use the .package() method.");
        assert!(
            !class_name.is_empty(),
            "You forgot to include the class name"
        );
        JavaClass {
            imports: None,
            class_name,
            superclass: None,
            class_annotations: None,
            class_modifiers: vec![],
            implements: None,
            fields: HashSet::new(),
            package,
            methods: vec![],
            generic_params: GenericParams::new(vec![]),
        }
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

    pub fn import(mut self, imp: self::imports::Import) -> Self {
        match self.imports {
            Some(ref mut imports) => {
                imports.push(imp);
                self
            }
            None => {
                self.imports = Some(vec![imp]);
                self
            }
        }
    }
    pub fn field(mut self, f: Field) -> Self {
        self.fields.insert(f);
        self
    }

    pub fn annotation(mut self, a: Annotation) -> Self {
        match self.class_annotations {
            Some(ref mut annotations) => {
                annotations.push(a);
                self
            }
            None => {
                self.class_annotations = Some(vec![a]);
                self
            }
        }
    }
    pub fn implements(mut self, interface: Implements) -> Self {
        match self.implements {
            Some(ref mut implements) => {
                implements.push(interface);
                self
            }
            None => {
                self.implements = Some(vec![interface]);
                self
            }
        }
    }

    pub fn build(self) -> Self {
        self
    }
}
//will probably split it up in methodDecl and methodBody
pub struct JavaEnum {
    enum_types: Vec<(String, String)>,
    enum_name: String,
    modifiers: Vec<AccessModifiers>,
    package: String,
    imports: Vec<self::imports::Import>,
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

    pub fn imports(mut self, imports: Vec<self::imports::Import>) -> Self {
        self.imports.extend(imports);
        self
    }
}
#[derive(Debug, Clone)]
pub struct GenericParams {
    pub generics: Vec<String>,
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

#[derive(Clone)]
pub struct Annotation {
    pub qualified_name: String,
    //name = value
    pub params_list: Option<Vec<(String, String)>>,
}
impl Codegen for Annotation {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        result.push_str(&format!("@{} ", self.qualified_name));
        if let Some(ref params_list) = self.params_list {
            result.push('(');
            result.push('\n');
            for param in params_list {
                result.push_str(&format!("{} = {}\n", param.0, param.1))
            }
            result.push(')');
        }
        result.push('\n');
        result
    }
}

#[derive(Debug, Clone)]
pub struct TypeName {
    pub name: String,
    pub generic_params: Option<GenericParams>,
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

impl PartialEq<Self> for Annotation {
    fn eq(&self, other: &Self) -> bool {
        self.qualified_name.eq(&other.qualified_name)
    }
}

impl Eq for Annotation {}

impl Hash for Annotation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.qualified_name.hash(state)
    }
}

impl PartialEq<Self> for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for TypeName {}

impl Hash for TypeName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl Hash for AccessModifiers {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        <AccessModifiers as Into<String>>::into(*self).hash(state)
    }
}

impl PartialEq<Self> for AccessModifiers {
    fn eq(&self, other: &Self) -> bool {
        <AccessModifiers as Into<String>>::into(*self)
            == <AccessModifiers as Into<String>>::into(*other)
    }
}

impl PartialOrd<Self> for AccessModifiers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if <AccessModifiers as Into<String>>::into(*self)
            == <AccessModifiers as Into<String>>::into(*other)
        {
            return Some(Ordering::Equal);
        }
        //private,public,protected > abstract > final > static
        if self.eq(&AccessModifiers::Private)
            || self.eq(&AccessModifiers::Public)
            || self.eq(&AccessModifiers::Protected)
        {
            return Some(Ordering::Greater);
        }
        if other.eq(&AccessModifiers::Protected)
            || other.eq(&AccessModifiers::Public)
            || other.eq(&AccessModifiers::Private)
        {
            return Some(Ordering::Less);
        }

        if self.eq(&AccessModifiers::Abstract) {
            return Some(Ordering::Greater);
        }
        if other.eq(&AccessModifiers::Abstract) {
            return Some(Ordering::Less);
        }

        if self.eq(&AccessModifiers::Final) {
            return Some(Ordering::Greater);
        }
        if other.eq(&AccessModifiers::Final) {
            return Some(Ordering::Less);
        }

        Some(Ordering::Equal)
    }
}

impl Eq for AccessModifiers {}

impl Ord for AccessModifiers {
    fn cmp(&self, other: &Self) -> Ordering {
        if <AccessModifiers as Into<String>>::into(*self)
            == <AccessModifiers as Into<String>>::into(*other)
        {
            return Ordering::Equal;
        }
        //private,public,protected > abstract > fina
        if self.eq(&AccessModifiers::Private)
            || self.eq(&AccessModifiers::Public)
            || self.eq(&AccessModifiers::Protected)
        {
            return Ordering::Greater;
        }
        if other.eq(&AccessModifiers::Protected)
            || other.eq(&AccessModifiers::Public)
            || other.eq(&AccessModifiers::Private)
        {
            return Ordering::Less;
        }

        if self.eq(&AccessModifiers::Abstract) {
            return Ordering::Greater;
        }
        if other.eq(&AccessModifiers::Abstract) {
            return Ordering::Less;
        }

        if self.eq(&AccessModifiers::Final) {
            return Ordering::Greater;
        }
        if other.eq(&AccessModifiers::Final) {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

impl Into<&str> for AccessModifiers {
    fn into(self) -> &'static str {
        match self {
            AccessModifiers::Public => "public",
            AccessModifiers::Private => "private",
            AccessModifiers::Protected => "protected",
            AccessModifiers::Static => "static",
            AccessModifiers::Abstract => "abstract",
            AccessModifiers::Final => "final",
        }
    }
}
impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub enum AccessModifiers {
    #[default]
    Public,
    Private,
    Protected,
    Static,
    Abstract,
    Final,
    //Will not use those
    //Native
    //Synchronised
    //Transient
    //Volatile
    //strictfp
}
