use crate::{
    annotations::Annotation,
    classes::JavaClass,
    fields::Field,
    imports::Import,
    interfaces::Interface,
    java_structs::VariableParam,
    methods::Method,
    types::{GenericParams, TypeName},
};
pub struct CrudBuilder {
    for_class: JavaClass,
    // service_suffix: String,
    // repository_suffix: String,
    // controller_suffix: String,
    // service_package: String,
    // repository_package: String,
    // controller_package: String,
}

impl CrudBuilder {
    pub fn new(jclass: JavaClass) -> Self {
        Self { for_class: jclass }
    }
    //TODO, find a way to get rid of the imports

    fn jpa_class_name() -> String {
        todo!();
    }
    fn service_class_name() -> String {
        todo!();
    }

    pub fn jpa_repository_of(&self, cls_import: Import) -> Interface {
        let jclass = self.for_class.to_owned();
        let mut repo = Interface::new("".to_string(), jclass.class_name.clone() + "Repository");
        //theses calls could be completely written in the OutputDirs class
        let find_by_id_method = Method::new(
            TypeName::new_with_generics(
                "Optional".into(),
                GenericParams::new(vec![jclass.class_name.clone()]),
            ),
            "findById".into(),
        )
        .param(VariableParam::new("Long".into(), "id".into()));
        repo = repo
            .public()
            .import(cls_import)
            .import(Import::new("java.util".into(), "Optional".into()))
            .import(Import::new(
                "org.springframework.data.jpa.repository".into(),
                "JpaRepository".into(),
            ))
            .extends(TypeName::new_with_generics(
                "JpaRepository".to_owned(),
                GenericParams::new(vec![jclass.class_name.clone().into(), "Long".into()]),
            ))
            .method(find_by_id_method);
        repo
    }

    pub fn spring_boot_entity(&self) -> JavaClass {
        let jclass = self.for_class.to_owned();
        let id_field = id_field_for_entity();
        let lombok_annots: Vec<Annotation> = vec![
            "Data".into(),
            "AllArgsConstructor".into(),
            "NoArgsConstructor".into(),
        ];
        let entity_annotation = "Entity".into();
        let entity = jclass
            .import(Import::new("jakarta.persistence".into(), "Entity".into()))
            .import(Import::new(
                "jakarta.persistence".into(),
                "GeneratedValue".into(),
            ))
            .import(Import::new(
                "jakarta.persistence".into(),
                "GenerationType".into(),
            ))
            .import(Import::new("jakarta.persistence".into(), "Id".into()))
            .import(Import::new("lombok".into(), "AllArgsConstructor".into()))
            .import(Import::new("lombok".into(), "Data".into()))
            .import(Import::new("lombok".into(), "NoArgsConstructor".into()))
            .annotations(lombok_annots)
            .annotation(entity_annotation)
            .field(id_field);
        entity
    }

    pub fn dto_from_class(&self, class_import: Import) -> JavaClass {
        let jclass = self.for_class.to_owned();
        let name = jclass.class_name.clone() + "DTO";
        let initial_class_name = jclass.class_name.clone();
        let mut dto = jclass.class_name(name.clone());
        dto = dto.import(class_import);
        //DTO Constructor
        let dto_constructor =
            Method::new(TypeName::new("".into()), name)
                .public()
                .param(VariableParam::new(
                    TypeName::new(initial_class_name.clone()),
                    initial_class_name.clone().to_lowercase(),
                ));
        dto = dto.method(dto_constructor);

        dto
    }

    pub fn service_from_class(&self, jpa_import: Import) -> JavaClass {
        let jclass = self.for_class.to_owned();
        let mut service = jclass.clone();
        //need to find a way to not have to do the "magic" strings
        service.class_name = jclass.class_name.clone() + "Service";
        service = service.annotation("Service".into());
        let repo_name = (&jclass).class_name.to_owned() + "Repository";
        service = service.field(
            Field::n("repository".into(), TypeName::new(repo_name.clone()))
                .annotation(Annotation::autowired()),
        );
        let sclass_name = service.class_name.clone();
        service = service.method(
            Method::new(TypeName::new("".into()), sclass_name)
                .public()
                .annotation(Annotation::autowired())
                .param(VariableParam::new(repo_name.into(), "repository".into())),
        );

        service = service
            .public()
            .import(jpa_import)
            .import(Import::new(
                "org.springframework.beans.factory.annotation".into(),
                "Autowired".into(),
            ))
            .import(Import::new(
                "org.springframework.stereotype".into(),
                "Service".into(),
            ));
        service
    }

    pub fn controller_from_class(&self, service_import: Import, dto_import: Import) -> JavaClass {
        let jclass = self.for_class.to_owned();
        let id_path_variable =
            VariableParam::new("Long".into(), "id".into()).annotation("PathVariable".into());
        let initial_class_name = jclass.class_name.clone();
        let mut controller = JavaClass::new(initial_class_name.clone() + "Controller", "".into());
        let post_mapping = "PostMapping".into();
        let get_mapping = "GetMapping".into();
        let get_mapping_id = Annotation::new("GetMapping".into())
            .param("value".into(), "\"".to_owned() + "/{id}" + "\"");
        let delete_mapping_id = Annotation::new("DeleteMapping".into())
            .param("value".into(), "\"".to_owned() + "/{id}" + "\"");
        let update_mapping_id = Annotation::new("PutMapping".into())
            .param("value".into(), "\"".to_owned() + "/{id}" + "\"");
        let post = Method::new(
            TypeName::new_with_generics(
                "ResponseEntity".into(),
                GenericParams::new(vec![initial_class_name.clone() + "DTO"]),
            ),
            "create".to_owned() + &initial_class_name,
        )
        .annotation(post_mapping)
        .code(format!(
            r#"
            return null;
    "#,
        ));
        let get_by_id = Method::new(
            TypeName::new_with_generics(
                "ResponseEntity".into(),
                GenericParams::new(vec![initial_class_name.clone() + "DTO"]),
            ),
            "get".to_owned() + &initial_class_name + "ById",
        )
        .annotation(get_mapping_id)
        .param(id_path_variable.clone())
        .code(format!(
            r#"
            return null;
    "#,
        ));

        let get_all = Method::new(
            TypeName::new_with_generics(
                "ResponseEntity".into(),
                GenericParams::new(vec![format!("List<{}DTO>", initial_class_name)]),
            ),
            "getAll".to_owned() + &initial_class_name + "s",
        )
        .annotation(get_mapping)
        .code(format!(
            r#"
            return null;
    "#,
        ));

        let update = Method::new(
            TypeName::new_with_generics(
                "ResponseEntity".into(),
                GenericParams::new(vec![initial_class_name.clone() + "DTO"]),
            ),
            "update".to_owned() + &initial_class_name,
        )
        .annotation(update_mapping_id)
        .param(id_path_variable.clone())
        .code(format!(
            r#"
            return null;
    "#,
        ));

        let delete = Method::new(
            TypeName::new_with_generics(
                "ResponseEntity".into(),
                GenericParams::new(vec!["Void".to_string()]),
            ),
            "delete".to_owned() + &initial_class_name,
        )
        .annotation(delete_mapping_id)
        .param(id_path_variable)
        .code(format!(
            r#"
            return null;
    "#,
        ));

        controller = controller
            .method(post)
            .method(update)
            .method(delete)
            .method(get_all)
            .method(get_by_id);
        let spring_imports = vec![
            Import::new("org.springframework.http".into(), "HttpStatus".into()),
            Import::new("org.springframework.http".into(), "ResponseEntity".into()),
            Import::new("org.springframework.web.bind.annotation".into(), "*".into()),
        ];
        controller = controller.imports(spring_imports);

        controller = controller
            .import(service_import)
            .import(dto_import)
            .import(Import::new("java.util".into(), "List".into()));

        controller = controller.annotation("RestController".into()).annotation(
            Annotation::new("RequestMapping".into()).param(
                "value".into(),
                "\"".to_owned() + "/" + &initial_class_name.to_lowercase() + "\"",
            ),
        );

        let service_type: TypeName = (initial_class_name + "Service").into();
        controller = controller.field(service_type.clone().into());
        let constructor = Method::new("".into(), controller.class_name.clone())
            .param(service_type.into())
            .code("".into());
        controller = controller.method(constructor);

        controller
    }
}
fn id_field_for_entity() -> Field {
    let id_annotation = Annotation::new("Id".into());
    let id_annotation_strategy = Annotation::new("GeneratedValue".into())
        .param("strategy".into(), "GenerationType.IDENTITY".into());
    let id_field = Field::n("id".into(), TypeName::new("Long".into()))
        .annotation(id_annotation)
        .annotation(id_annotation_strategy);
    id_field
}
