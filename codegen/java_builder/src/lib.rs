//inspiration from: https://github.com/palantir/javapoet/

pub mod java_structs;

pub mod java_builder {

    pub use crate::java_structs::{
        AccessModifiers, Annotation, Field, GenericParams, Implements, Import, Interface,
        JavaClass, Method, TypeName, VariableParam,
    };
    use std::collections::HashSet;
    use std::fs;
    use std::path::PathBuf;

    use tree_sitter::Parser;

    fn java_parser() -> Parser {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_java::LANGUAGE.into())
            .expect("Error loading java grammar");
        parser
    }

    fn assert_program_is_syntactically_correct(java_str: &str) {
        let mut parser = java_parser();
        let tree = parser.parse(java_str, None).unwrap();
        assert!(!tree.root_node().has_error());
    }

    #[test]
    #[should_panic]
    fn panics_when_modifiers_used_incorrectly() {
        let modifiers: Vec<AccessModifiers> =
            vec![AccessModifiers::Protected, AccessModifiers::Public];
        let result = modifiers.generate_code();
        println!("Result: {}", result);
    }

    //testing could be per element to decrease the surface
    //covered by a single test
    #[test]
    pub fn can_generate_class() {
        //this is like an integration test
        //todo write smaller finer grained unit tests
        //similar to the ./FieldSpec.java file
        //with some extras to cover extra stuff
        let class_name = "FieldSpec";
        let package_name = "com.palantir.javapoet";
        //we care about correct syntax in this library and offering a proper api
        //as usages grow things will be added
        let xml_root_elem_annotation = Annotation {
            qualified_name: "XmlRootElement".to_string(),
            params_list: Some(vec![("name".to_string(), "phone-number".to_string())]),
        };
        let m1 = Method {
            annotations: vec![],
            code: "ArrayList<String> names = new ArrayList<>();".to_owned(),
            return_type: TypeName {
                name: "ArrayList".to_string(),
                generic_params: Some(GenericParams {
                    generics: vec!["String".to_string()],
                }),
            },
            parameters: vec![
                VariableParam {
                    type_: TypeName {
                        name: "String".to_owned(),
                        generic_params: None,
                    },
                    name: "name".to_owned(),
                    annotation: vec![],
                },
                VariableParam {
                    annotation: vec![],
                    name: "combiner".to_string(),
                    type_: TypeName {
                        name: "Function".to_string(),
                        generic_params: Some(GenericParams {
                            generics: vec!["Integer".to_string(), "Integer".to_string()],
                        }),
                    },
                },
            ],
            name: "addName".to_owned(),
            modifiers: vec![AccessModifiers::Public],
            generic_params: None,
        };
        let m2 = Method {
            annotations: vec![],
            code: "System.out.println(\"Hello World\");".to_string(),
            return_type: TypeName {
                name: "void".to_string(),
                generic_params: None,
            },
            parameters: vec![VariableParam {
                type_: TypeName {
                    name: "String".to_string(),
                    generic_params: None,
                },
                name: "Greeting".to_string(),
                annotation: vec![],
            }],
            modifiers: vec![AccessModifiers::Public, AccessModifiers::Static],
            name: "main".to_owned(),
            generic_params: None,
        };
        let m3 = Method {
            annotations: vec![],
            code: "".to_owned(),
            return_type: TypeName {
                name: "void".to_owned(),
                generic_params: None,
            },
            parameters: vec![],
            modifiers: vec![AccessModifiers::Public, AccessModifiers::Static],
            generic_params: None,
            name: "EmptyMethod".to_owned(),
        };
        let methods = vec![m1.clone(), m2.clone(), m3.clone()];
        let f1 = Field {
            annotation: vec![Annotation {
                qualified_name: "Autowired".to_string(),
                params_list: None,
            }],
            modifiers: vec![AccessModifiers::Final, AccessModifiers::Private],
            name: "type".to_string(),
            type_: TypeName {
                name: "TypeName".to_string(),
                generic_params: None,
            },
            initializer: None,
        };
        let f2 = Field {
            name: "name".to_string(),
            type_: TypeName {
                name: "String".to_string(),
                generic_params: None,
            },
            modifiers: vec![AccessModifiers::Private, AccessModifiers::Final],
            initializer: None,
            annotation: vec![xml_root_elem_annotation.clone()],
        };
        let fields = vec![f1.clone(), f2.clone()];
        let superclass = TypeName {
            name: "Object".to_string(),
            generic_params: None,
        };
        let generic_interface = TypeName {
            name: "Comparable".to_string(),
            generic_params: Some(GenericParams {
                generics: vec!["ChronoLocalDate".to_string()],
            }),
        };

        let result = JavaClass::new(class_name.to_owned(), package_name.to_owned())
            .public()
            .generic_param("T".to_string())
            .generic_param("L".to_string())
            .extends(superclass.clone())
            .implements(generic_interface.clone())
            .import(Import {
                class_name: "IOException".to_string(),
                package_name: "java.io".to_string(),
                static_import: false,
            })
            .import(Import {
                class_name: "UncheckedIOException".to_string(),
                package_name: "java.io".to_string(),
                static_import: false,
            })
            .import(Import {
                class_name: "List".to_string(),
                package_name: "java.util".to_string(),
                static_import: false,
            })
            .import(Import {
                class_name: "SourceVersion".to_string(),
                package_name: "javax.lang.model".to_string(),
                static_import: false,
            })
            .import(Import {
                class_name: "TemplateEngine".to_string(),
                package_name: "org.openapi.tools".to_string(),
                static_import: false,
            })
            .method(m3)
            .method(m2)
            .method(m1)
            .field(f1)
            .field(f2)
            .generate_code();

        assert!(result.len() > 0, "Codegen gave empty output");
        assert_program_is_syntactically_correct(&result);
        println!("{}", result);
        assert!(
            result.contains(package_name),
            "The package name was not properly included"
        );

        println!("Result is: \n{result}");
        assert!(
            result.contains(class_name),
            "The classname was not properly included"
        );
        //private,public,protected > abstract > final > static
        //i could add an assert to ensure that private,public,protected are not in the same declaration
        //i could do the asserts in a more property-based testing manner
        //but right now i wont
        assert!(!result.contains("final private"));
        assert!(!result.contains("static public"));
        assert!(result.contains(&format!("extends {}", superclass.name.as_str())));
        assert!(result.contains(&xml_root_elem_annotation.qualified_name));
        assert_methods_are_generated(
            &result,
            methods,
            "In Class, Methods are not properly generated",
        );
        assert_fields_are_generated(
            result.as_str(),
            fields,
            "In Class, Fields are not properly generated",
        );
        // assert_imports_are_generated(&result,imports)

        assert!(result.contains(&format!("implements {}", generic_interface.name)));
    }

    #[test]
    pub fn can_generate_interface() {
        let m1 = Method {
            annotations: vec![],
            generic_params: None,
            name: "findByLastName".to_owned(),
            modifiers: vec![],
            parameters: vec![VariableParam {
                name: "lastName".to_owned(),
                type_: TypeName {
                    name: "String".to_owned(),
                    generic_params: None,
                },
                annotation: vec![],
            }],
            code: "".to_owned(),
            return_type: TypeName {
                name: "List".to_owned(),
                generic_params: Some(GenericParams {
                    generics: vec!["Customer".to_owned()],
                }),
            },
        };

        let m2 = Method {
            generic_params: None,
            name: "findById".to_owned(),
            annotations: vec![],
            code: "".to_owned(),
            modifiers: vec![],
            parameters: vec![VariableParam {
                annotation: vec![],
                name: "id".to_owned(),
                type_: TypeName {
                    name: "long".to_owned(),
                    generic_params: None,
                },
            }],
            return_type: TypeName {
                name: "Customer".to_owned(),
                generic_params: None,
            },
        };
        let interface = Interface {
            name: "CustomerRepository".to_owned(),
            modifier: AccessModifiers::Public,
            package: "com.example.accessingdatajpa".to_owned(),
            imports: vec![
                Import {
                    package_name: "java.util".to_owned(),
                    class_name: "List".to_owned(),
                    static_import: false,
                },
                Import {
                    package_name: "org.springframework.data.repository".to_owned(),
                    class_name: "CrudRepository".to_owned(),
                    static_import: false,
                },
            ],
            generics: None,
            methods: vec![
                m1.clone(),
                m2.clone(), //play with defaults to avoid writing too much code
            ],
            annotations: vec![],
            superclass: Some(TypeName {
                name: "CrudRepository".to_owned(),
                generic_params: Some(GenericParams {
                    generics: vec!["Customer".to_owned(), "Long".to_owned()],
                }),
            }),
        };

        let result = interface.generate_code();
        assert_program_is_syntactically_correct(&result);
        assert_methods_are_generated(
            &result,
            vec![m1, m2],
            "Methods for interface are not properly generated",
        );
        assert!(
            result.contains("interface"),
            "Interface Keyword was not included in interface codegen"
        );
    }

    #[test]
    pub fn can_generate_enum() {
        // similar to ./TemplateFileType.java
        let enum_name = "TemplateFileType".to_string();
        let package_name = "org.openapitools.codegen.api".to_string();
        let enum_types = vec![
            ("API".to_string(), "Constants.APIS".to_string()),
            ("Model".to_string(), "Constants.MODELS".to_string()),
            ("APIDocs".to_string(), "Constants.API_DOCS".to_string()),
            ("ModelDocs".to_string(), "MODEL_DOCS".to_string()),
            ("APITests".to_string(), "Constants.API_TESTS".to_string()),
            (
                "SupportingFiles".to_string(),
                "Constants.SUPPORTING_FILES".to_string(),
            ),
        ];
        let enum_modifiers = vec![AccessModifiers::Public];
        let mut builder = JavaEnum::new(enum_name.clone(), package_name.clone());
        // builder = builder.
        builder = builder.types(enum_types.clone());
        builder = builder.modifiers(enum_modifiers.clone());
        let imports = vec![
            Import {
                class_name: "StringJoiner".to_string(),
                package_name: "java.util".to_string(),
                static_import: true,
            },
            Import {
                class_name: "ArrayList".to_string(),
                package_name: "java.util".to_string(),
                static_import: false,
            },
        ];
        builder = builder.imports(imports.clone());
        let result = builder.generate_code();
        assert_program_is_syntactically_correct(&result);

        assert!(
            result.contains(&format!("package {};", package_name)),
            "Package declaration is not included"
        );
        assert!(result.contains(&enum_name), "Enum name is not included");

        assert_imports_are_generated(&result, imports, "Imports are not generated properly");

        for types in enum_types {
            assert!(result.contains(&types.0), "Enum Type is Not included");
            assert!(result.contains(&types.1), "Enum Value is Not included");
        }
    }

    #[test]
    #[should_panic]
    // #[should_panic(expected = "Private and Public modifiers should not be used in the same declaration")]
    fn panics_when_modifiers_used_incorrectly_2() {
        let modifiers: Vec<AccessModifiers> =
            vec![AccessModifiers::Private, AccessModifiers::Public];
        let result = modifiers.generate_code();
        println!("Result: {}", result);
    }

    #[test]
    #[should_panic]
    fn panics_when_abstract_method_has_body() {
        let m = Method {
            modifiers: vec![AccessModifiers::Abstract],
            code: "System.out.println('Hello World');".to_string(),
            name: "Greeting".to_string(),
            parameters: vec![],
            annotations: vec![],
            generic_params: None,
            return_type: TypeName {
                name: "void".to_string(),
                generic_params: None,
            },
        };
        m.generate_code();
    }

    #[test]
    #[should_panic]
    fn panics_when_modifiers_used_incorrectly_3() {
        let modifiers: Vec<AccessModifiers> =
            vec![AccessModifiers::Protected, AccessModifiers::Private];
        let result = modifiers.generate_code();
        println!("Result: {}", result);
    }

    fn assert_modifiers_are_generated(java_str: &str, modifiers: Vec<AccessModifiers>) {
        for modifier in modifiers {
            assert!(java_str.contains(<AccessModifiers as Into<String>>::into(modifier).as_str()));
        }
    }

    fn assert_imports_are_generated(java_str: &str, imports: Vec<Import>, msg: &str) {
        for imp in imports {
            assert!(
                java_str.contains(&format!("import {}.{};", imp.package_name, imp.class_name))
                    || java_str.contains(&format!(
                        "import static {}.{}",
                        imp.package_name, imp.class_name
                    )),
                "{}",
                msg
            )
        }
    }

    fn assert_fields_are_generated(java_str: &str, fields: Vec<Field>, msg: &str) {
        fields.iter().for_each(|f| {
            assert!(java_str.contains(&f.name), "{}", msg);
        });
    }

    fn assert_methods_are_generated(java_str: &str, methods: Vec<Method>, msg: &str) {
        for m in methods {
            assert!(java_str.contains(&m.name), "{}", msg);
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
                if let Some(generics) = &m.generic_params {
                    result.push(' ');
                    result.push_str(&generics.generate_code());
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

    impl Codegen for Vec<Import> {
        fn generate_code(&self) -> String {
            let mut result = "".to_string();
            result.push('\n');
            for import in self.iter() {
                result.push_str(&*import.generate_code());
            }
            result
        }
    }

    impl Codegen for Import {
        fn generate_code(&self) -> String {
            match &self.static_import {
                false => format!("import {}.{};\n", self.package_name, self.class_name),
                true => format!("import static {}.{};\n", self.package_name, self.class_name),
            }
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

    pub trait Codegen {
        fn generate_code(&self) -> String;
    }

    impl Codegen for Vec<Implements> {
        fn generate_code(&self) -> String {
            let mut result = "".to_string();
            result.push_str("implements ");

            for (pos, elem) in self.iter().enumerate() {
                result.push_str(&elem.generate_code());
                // result.push_str(&elem.name);
                // if let Some(generics) = &elem.generic_params {
                //     result.push_str(&generics.generate_code());
                // }

                if pos != self.len() - 1 {
                    result.push_str(", ");
                }
            }
            result.push(' ');
            result
        }
    }
    //these could also be impl Into<> for <>
    fn extract_interface_from_class(class_obj: &JavaClass) -> Interface {
        todo!("Class to interface conversion is under construction");
    }

    fn implement_interface(interface: &Interface) -> JavaClass {
        todo!("Automatic Interface implementation is under construction");
    }

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
            for (position, (enum_type_name, enum_type_value)) in self.enum_types.iter().enumerate()
            {
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
                generic_params: GenericParams { generics: vec![] },
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

        pub fn import(mut self, imp: Import) -> Self {
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

        pub fn generate_class_to_file(&self, path_buf: PathBuf) -> Result<String, String> {
            let result = self.generate_code();
            fs::write(path_buf, self.generate_code()).expect("TODO: panic message");
            Ok(result)
        }

        pub fn build(self) -> Self {
            self
        }
    }

    mod record_builder {}
    // not needed rn will be implemented later
    mod annotation_builder {}
}
