mod common;

#[cfg(test)]
pub mod java_structs_tests {

    use java_builder::{
        annotations::Annotation,
        classes::JavaClass,
        enums::JavaEnum,
        fields::Field,
        imports::Import,
        interfaces::Interface,
        methods::Method,
        modifiers::AccessModifiers,
        types::{GenericParams, TypeName},
        Codegen, VariableParam,
    };

    use crate::common::assert_program_is_syntactically_correct;

    #[test]
    #[should_panic]
    fn panics_when_interface_method_has_body() {
        let i = Interface::new(
            "org.openapi.tools".to_owned(),
            "WithNonEmptyMethods".to_owned(),
        );
        let non_empty_body_method =
            Method::new(TypeName::new("void".to_owned()), "addToList".to_owned())
                .code(r#"System.out.println(\"Default Impl\")"#.to_owned());
        let i = i.methods(vec![non_empty_body_method]);
        i.generate_code();
    }

    #[test]
    #[should_panic]
    fn panics_when_modifiers_protected_and_public() {
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
        let m1 = Method::new(
            TypeName::new_with_generics(
                "ArrayList".to_string(),
                GenericParams::new(vec!["String".to_string()]),
            ),
            "addName".to_owned(),
        )
        .code("ArrayList<String> names = new ArrayList<>();".to_owned())
        .param(VariableParam {
            type_: TypeName::new("String".to_owned()),
            name: "name".to_owned(),
            annotation: vec![],
        })
        .param(VariableParam {
            annotation: vec![],
            name: "combiner".to_string(),
            type_: TypeName::new_with_generics(
                "Function".to_string(),
                GenericParams::new(vec!["Integer".to_string(), "Integer".to_string()]),
            ),
        })
        .public();
        let m2 = Method::new(TypeName::new("void".to_string()), "main".to_owned())
            .public()
            .static_()
            .code("System.out.println(\"Hello World\");".to_string())
            .param(VariableParam {
                type_: TypeName::new("String".to_string()),
                name: "Greeting".to_string(),
                annotation: vec![],
            });
        let m3 = Method::new(TypeName::new("void".to_owned()), "EmptyMethod".to_owned())
            .code("".to_owned())
            .modifier(AccessModifiers::Static);
        let methods = vec![m1.clone(), m2.clone(), m3.clone()];
        let f1 = Field {
            annotation: vec![Annotation {
                qualified_name: "Autowired".to_string(),
                params_list: None,
            }],
            modifiers: vec![AccessModifiers::Final, AccessModifiers::Private],
            name: "type".to_string(),
            type_: TypeName::new("TypeName".to_string()),
            initializer: None,
        };
        let f2 = Field {
            name: "name".to_string(),
            type_: TypeName::new("String".to_string()),
            modifiers: vec![AccessModifiers::Private, AccessModifiers::Final],
            initializer: None,
            annotation: vec![xml_root_elem_annotation.clone()],
        };
        let fields = vec![f1.clone(), f2.clone()];
        let superclass = TypeName::new("Object".to_string());
        let generic_interface = TypeName::new_with_generics(
            "Comparable".to_string(),
            GenericParams::new(vec!["ChronoLocalDate".to_string()]),
        );

        let result = JavaClass::new(class_name.to_owned(), package_name.to_owned())
            .public()
            .generic_param("T".to_string())
            .generic_param("L".to_string())
            .extends(superclass.clone())
            .implements(generic_interface.clone())
            .import(Import::new(
                "java.io".to_string(),
                "IOException".to_string(),
            ))
            .import(Import::new(
                "java.io".to_string(),
                "UncheckedIOException".to_string(),
            ))
            .import(Import::new("java.util".to_string(), "List".to_string()))
            .import(Import::new(
                "javax.lang.model".to_string(),
                "SourceVersion".to_string(),
            ))
            .import(Import::new(
                "org.openapi.tools".to_string(),
                "TemplateEngine".to_string(),
            ))
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
        let m1 = Method::new(
            TypeName::new_with_generics(
                "List".to_owned(),
                GenericParams::new(vec!["Customer".to_owned()]),
            ),
            "findByLastName".to_owned(),
        )
        .param(VariableParam {
            name: "lastName".to_owned(),
            annotation: vec![],
            type_: TypeName::new("String".to_owned()),
        });

        let m2 = Method::new(TypeName::new("Customer".to_owned()), "findById".to_owned()).param(
            VariableParam {
                annotation: vec![],
                name: "id".to_owned(),
                type_: TypeName::new("long".to_owned()),
            },
        );
        let interface = Interface::new(
            "com.example.accessingdatajpa".to_owned(),
            "CustomerRepository".to_owned(),
        )
        .public()
        .import(Import::new("java.util".to_owned(), "List".to_owned()))
        .import(Import::new(
            "org.springframework.data.repository".to_owned(),
            "CrudRepository".to_owned(),
        ))
        .methods(vec![
            m1.clone(),
            m2.clone(), //play with defaults to avoid writing too much code
        ])
        .extends(TypeName {
            name: "CrudRepository".to_owned(),
            generic_params: Some(GenericParams::new(vec![
                "Customer".to_owned(),
                "Long".to_owned(),
            ])),
        });

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
            Import::new("java.util".to_string(), "StringJoiner".to_string()).static_(),
            Import::new("java.util".to_string(), "ArrayList".to_string()),
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
    fn panics_when_modifiers_private_and_public() {
        let modifiers: Vec<AccessModifiers> =
            vec![AccessModifiers::Private, AccessModifiers::Public];
        let result = modifiers.generate_code();
        println!("Result: {}", result);
    }

    #[test]
    #[should_panic]
    fn panics_when_abstract_method_has_body() {
        let m = Method::new(TypeName::new("void".to_string()), "Greeting".to_string())
            .abstract_()
            .code("System.out.println('Hello World');".to_string());

        m.generate_code();
    }

    #[test]
    #[should_panic]
    fn panics_when_modifiers_protected_and_private() {
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
}
