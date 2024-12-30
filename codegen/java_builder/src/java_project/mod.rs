//check what build systems are available
//implement a java project builder (for now only maven with spring boot codegen)
//select:
// - libraries
// - general options that have sensible defaults (model folder,docs folder, services folder etc. etc.)
// - extras: CRUDs + Search
mod spring_packages;
pub mod maven_builder {
    use crate::java_project::spring_packages;
    use crate::java_structs::*;
    use annotations::Annotation;
    use classes::JavaClass;
    use fields::Field;
    use imports::Import;
    use interfaces::Interface;
    use std::{
        fs::{self, create_dir, create_dir_all, remove_dir_all, write},
        path::Path,
    };
    use types::TypeName;

    use super::{
        crud_builder::{
            controller_from_class, dto_from_class, jpa_repository_of, service_from_class,
            spring_boot_entity,
        },
        pom_xml::{Generate, PomXml},
    };

    struct Progress {
        pub has_written_initial_files: bool,
        pub has_created_initial_folders: bool,
    }

    pub struct MavenCodebase {
        test_folder: String,
        code_folder: String,
        res_folder: String,
        //group_id.artifact_id
        pom_xml: PomXml,
        root_folder: String,
        models_folder: String,
        entities: Vec<JavaClass>,
        services_folder: String,
        controllers_folder: String,
        controller_classes: Vec<JavaClass>,
        dtos_folder: String,
        dto_classes: Vec<JavaClass>,
        services: Vec<JavaClass>,
        repos_folder: String,
        jpa_repos: Vec<Interface>,
        progress: Progress,
    }

    impl MavenCodebase {
        fn create_initial_folders(&mut self) {
            if !self.progress.has_created_initial_folders {
                self.progress.has_created_initial_folders = true;
            } else {
                return;
            }
            let main_default = self.root_folder.clone() + "/src/main";
            let project_path = format!("{}.{}", &self.pom_xml.group_id, &self.pom_xml.artifact_id);
            let package_path = project_path.replace(".", "/");
            //the order those folders are being created matters,
            //create_dir_all fails if any of the parents exist

            let code_folder = main_default.to_owned() + &"/java/" + &package_path;
            if let Err(e) = create_dir_all(&code_folder) {
                assert!(false, "Failed to create main/java folder, err {}", e);
            }

            match create_dir(code_folder.clone() + "/" + &self.repos_folder) {
                Ok(r) => println!("Created repositories folder successfully"),
                Err(e) => println!("Failed to create /repositories folder {}", e),
            }
            match create_dir(code_folder.clone() + "/" + &self.models_folder) {
                Ok(r) => println!("Created models folder successfully"),
                Err(e) => println!("Failed to create /models folder {}", e),
            }
            match create_dir(code_folder.clone() + "/" + &self.services_folder) {
                Ok(r) => println!("Created services folder successfully"),
                Err(e) => println!("Failed to create /services folder {}", e),
            }

            if let Err(e) = create_dir(code_folder.clone() + "/" + &self.dtos_folder) {
                assert!(false, "Failed to create DTO folder, err {}", e);
            }

            if let Err(e) = create_dir(code_folder + "/" + &self.controllers_folder) {
                assert!(false, "Failed to create controllers folder, err {}", e);
            }

            if let Err(e) = create_dir_all(&self.test_folder) {
                assert!(false, "Failed to create test folder, err {}", e);
            }
            if let Err(e) = create_dir(&self.res_folder) {
                assert!(false, "Failed to create resources folder,err {}", e);
            }
        }
        pub fn write_initial_files(&mut self) {
            //todo write application.properties based on the pom.xml file
            if self.progress.has_written_initial_files {
                println!("Have already written initial_files, skipping");
                return;
            }

            if !self.progress.has_created_initial_folders {
                self.create_initial_folders();
            }

            let entrypoint = create_spring_main_class(&self.pom_xml);
            let mainfile = write(
                self.code_folder.clone().to_owned() + "/" + &entrypoint.class_name + ".java",
                entrypoint.generate_code(),
            );

            if let Err(e) = mainfile {
                assert!(false, "Main file could not be written err {}", e);
            }

            if let Err(e) = write(
                (&self.root_folder).to_owned() + "/" + "pom.xml",
                &self.pom_xml.generate(),
            ) {
                assert!(false, "pom.xml file could not be written err {}", e);
            }

            self.progress.has_written_initial_files = true;
        }
        pub fn new(pom_xml: PomXml, output_dir: &str) -> Self {
            let main_default = output_dir.to_string() + "/src/main";
            let project_path = format!("{}.{}", &pom_xml.group_id, &pom_xml.artifact_id);
            let package_path = project_path.replace(".", "/");
            //the order those folders are being created matters,
            //create_dir_all fails if any of the parents exist

            let code_folder = main_default.to_owned() + &"/java/" + &package_path;
            let test_folder = output_dir.to_string() + &"/src/test/java/" + &package_path;
            let res_folder = main_default.to_owned() + &"/resources";

            Self {
                pom_xml,
                root_folder: output_dir.to_string(),
                code_folder,
                test_folder,
                res_folder,
                controllers_folder: "controllers".into(),
                dtos_folder: "dto".into(),
                repos_folder: "repositories".into(),
                services_folder: "services".into(),
                models_folder: "models".into(),
                services: vec![],
                jpa_repos: vec![],
                entities: vec![],
                dto_classes: vec![],
                controller_classes: vec![],
                progress: Progress {
                    has_written_initial_files: false,
                    has_created_initial_folders: false,
                },
            }
        }

        //adds an entity model and the respective service and repo
        pub fn add_entity(mut self, jclass: JavaClass) -> Self {
            //i will be re-using this
            let model_import = format!("{}.models", self.pom_xml.get_root_package());
            let model_import = Import::new(model_import, jclass.clone().class_name);
            let entity = spring_boot_entity(jclass.clone());
            let jpa_repo = jpa_repository_of(jclass.clone(), model_import.clone());
            let repo_import = format!("{}.repositories", self.pom_xml.get_root_package());

            let service = service_from_class(
                jclass.clone(),
                Import::new(repo_import, jpa_repo.name.clone()),
            );

            let service_import = format!("{}.repositories", self.pom_xml.get_root_package());
            let dto = dto_from_class(jclass.clone(), model_import.clone());
            let dto_import = format!("{}.dto", self.pom_xml.get_root_package());
            let controller = controller_from_class(
                jclass.clone(),
                Import::new(service_import, service.class_name.clone()),
                Import::new(dto_import, dto.class_name.clone()),
            );
            self.entities.push(entity);
            self.services.push(service);
            self.jpa_repos.push(jpa_repo);
            self.dto_classes.push(dto);
            self.controller_classes.push(controller);
            self
        }

        pub fn add_entities(mut self, jclasses: Vec<JavaClass>) -> Self {
            for jclass in jclasses {
                self = self.add_entity(jclass);
            }
            self
        }

        pub fn generate_classes_in(&self, classes: Vec<JavaClass>, sub_folder: &str) {
            let in_package = format!("{}.{}", self.pom_xml.get_root_package(), sub_folder);
            classes
                .into_iter()
                .map(|cls| cls.package(in_package.clone()))
                .for_each(|cls| {
                    let path = self.code_folder.to_owned()
                        + "/"
                        + sub_folder
                        + "/"
                        + &cls.class_name
                        + ".java";
                    match fs::write(path, cls.generate_code()) {
                        Ok(r) => println!("Entities were successfully generated"),
                        Err(e) => println!("An error occurred when generating entities {}", e),
                    }
                });
        }

        pub fn generate_interfaces_in(&self, interfaces: Vec<Interface>, sub_folder: &str) {
            let in_package = format!("{}.{}", self.pom_xml.get_root_package(), sub_folder);
            interfaces
                .into_iter()
                .map(|cls| cls.package(in_package.clone()))
                .for_each(|cls| {
                    let path =
                        self.code_folder.to_owned() + "/" + sub_folder + "/" + &cls.name + ".java";
                    match fs::write(path, cls.generate_code()) {
                        Ok(r) => println!("Entities were successfully generated"),
                        Err(e) => println!("An error occurred when generating entities {}", e),
                    }
                });
        }

        pub fn generate_code(mut self) -> Self {
            self.create_initial_folders();
            self.write_initial_files();
            println!("Generating code");
            //TODO gotta fix all those .clone() calls
            let entities = self.entities.clone();
            let models_folder = self.models_folder.clone();
            self.generate_classes_in(entities, &models_folder);

            let repos = self.jpa_repos.clone();
            let repos_folder = self.repos_folder.clone();
            self.generate_interfaces_in(repos, &repos_folder);

            let services = self.services.clone();
            let services_folder = self.services_folder.clone();
            self.generate_classes_in(services, &services_folder);

            let dtos = self.dto_classes.clone();
            let dtos_folder = self.dtos_folder.clone();
            self.generate_classes_in(dtos, &dtos_folder);
            self
        }
    }

    //https://nick.groenen.me/notes/capitalize-a-string-in-rust/
    pub fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
    //todo i have some trouble getting it to compile with maven
    pub fn create_spring_main_class(pom: &super::pom_xml::PomXml) -> JavaClass {
        let class_name = capitalize(&pom.project_name);
        let package = pom.group_id.to_owned() + "." + &pom.artifact_id;
        let jclass = JavaClass::new(class_name.clone(), package)
            .import(imports::Import::new(
                "org.springframework.boot".to_owned(),
                "SpringApplication".to_owned(),
            ))
            .import(imports::Import::new(
                "org.springframework.boot.autoconfigure".into(),
                "SpringBootApplication".into(),
            ))
            .annotation(annotations::Annotation::new("SpringBootApplication".into()))
            .public()
            .method(
                methods::Method::new(types::TypeName::new("void".into()), "main".to_owned())
                    .static_()
                    .public()
                    .param(VariableParam::new(
                        types::TypeName::new("String[]".into()),
                        "args".into(),
                    ))
                    .code(format!("SpringApplication.run({}.class,args);", class_name)),
            );
        jclass
    }

    fn cleanup_folder(dir: &str) {
        if let Err(e) = remove_dir_all(dir) {
            assert!(false, "Removing all files from folder failed");
        }
    }
}
pub mod pom_xml {
    use super::spring_packages::*;
    pub struct PomXml {
        pub description: String,
        pub project_name: String,
        pub java: String,
        //add assertion that one dot is contained
        pub group_id: String,
        pub artifact_id: String,
        pub dependencies: Vec<Library>,
        //It has the same attributes thats why
        //+ a relative path ofc
        pub parent_pom: Library,
    }

    struct Library {
        group_id: String,
        artifact_id: String,
        version: Option<String>,
    }
    struct ProjectInfo {
        group_id: String,
        artifact_id: String,
        name: String,
        description: String,
        version: String,
    }

    impl Default for Library {
        fn default() -> Self {
            Self {
                group_id: "".to_owned(),
                artifact_id: "".to_owned(),
                version: None,
            }
        }
    }

    impl Library {
        pub fn new(group_id: String, artifact_id: String) -> Self {
            Self {
                group_id,
                artifact_id,
                version: None,
            }
        }

        pub fn new_with_version(group_id: String, artifact_id: String, version: String) -> Self {
            Self {
                group_id,
                artifact_id,
                version: Some(version),
            }
        }
    }

    impl Generate for Library {
        fn generate(&self) -> String {
            let mut result = "".to_owned();
            result += "<dependency>\n";
            result += &("<groupId>".to_owned() + &self.group_id + "</groupId>");
            result += &("<artifactId>".to_owned() + &self.artifact_id + "</artifactId>");
            if let Some(ref v) = &self.version {
                result += &("<version>".to_owned() + v + "</version>");
            }
            result += "\n</dependency>";
            result
        }
    }

    impl Generate for Vec<Library> {
        fn generate(&self) -> String {
            let mut result = "".to_owned();
            result += "<dependencies>\n";
            for lib in self.iter() {
                result += &lib.generate();
            }
            result += "\n</dependencies>";
            result
        }
    }
    impl PomXml {
        pub fn get_root_package(&self) -> String {
            self.group_id.to_owned() + "." + &self.artifact_id
        }

        pub fn new() -> Self {
            Self {
                description: "".to_owned(),
                project_name: "".to_owned(),
                java: "".to_owned(),
                group_id: "".to_owned(),
                artifact_id: "".to_owned(),
                dependencies: vec![],
                parent_pom: Library::default(),
            }
        }
        pub fn java_version(mut self, version: String) -> Self {
            self.java = version;
            self
        }

        pub fn artifact(mut self, id: String) -> Self {
            self.artifact_id = id;
            self
        }

        pub fn spring_boot(mut self) -> Self {
            let spring_parent = Library::new_with_version(
                "org.springframework.boot".to_owned(),
                "spring-boot-starter-parent".to_owned(),
                "3.4.1".to_owned(),
            );
            self.parent_pom = spring_parent;

            self = self.spring_boot_starter_web();
            self = self.lombok();
            self = self.spring_boot_starter_data_jpa();
            self
        }

        pub fn description(mut self, descr: String) -> Self {
            self.description = descr;
            self
        }

        pub fn project_name(mut self, name: String) -> Self {
            self.project_name = name;
            self
        }

        pub fn group_id(mut self, id: String) -> Self {
            self.group_id = id;
            self
        }

        pub fn add_library_with_version(
            mut self,
            artifact_id: String,
            group_id: String,
            version: String,
        ) -> Self {
            self.dependencies
                .push(Library::new_with_version(group_id, artifact_id, version));
            self
        }

        pub fn add_library(mut self, group_id: String, artifact_id: String) -> Self {
            self.dependencies.push(Library::new(group_id, artifact_id));
            self
        }

        fn check_dependencies_exist() {
            todo!("implement a way to check if dependencies exist with their related versions");
        }
    }

    impl Generate for PomXml {
        fn generate(&self) -> String {
            let mut result = "".to_owned();
            result += r#"<?xml version="1.0" encoding="UTF-8"?>"#;
            result += r#"<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">"#;
            result += &("<modelVersion>4.0.0</modelVersion>");
            result += "<parent>\n";
            result += &("<groupId>".to_owned() + &self.parent_pom.group_id + "</groupId>");
            result += &("<artifactId>".to_owned() + &self.parent_pom.artifact_id + "</artifactId>");
            if let Some(ref v) = &self.parent_pom.version {
                result += &("<version>".to_owned() + v + "</version>");
            }
            result += "\n</parent>";

            result += &(r#"<description>"#.to_owned() + &self.description + &"</description>\n");
            result += &(r#"<name>"#.to_owned() + &self.project_name + &"</name>\n");
            result += &("<groupId>".to_owned() + &self.group_id + &"</groupId>\n");

            result += &("<artifactId>".to_owned() + &self.artifact_id + &"</artifactId>");
            result += &("<version>".to_owned() + &"0.0.1-SNAPSHOT" + &"</version>");
            result += "<properties>\n";
            result += &("<java.version>".to_owned() + &self.java + &"</java.version>");
            result += "\n</properties>";

            result += &self.dependencies.generate();
            result += r#"</project>"#;
            println!("Xml result {}", result);
            result
        }
    }

    pub trait Generate {
        fn generate(&self) -> String {
            "".to_owned()
        }
    }
}
pub mod crud_builder {
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

    //TDD as usual easy af
    //takes classes as inputs
    //CrudOptions {controller:bool,service:bool,entity:bool,repository:bool,dto:bool}
    //bulk_create(Vec<JavaClass>,CrudOptions)
    //in the beginning the default will be everything true.
    //creates controllers + services + entities + repositories + repos
    //create_controller(JavaClass) -> JavaClass
    //create_service(JavaClass) -> JavaClass
    //create_entity(JavaClass) -> JavaClass
    //create_repository(JavaClass) -> JavaClass
    //create_dto_entity(JavaClass) -> JavaClass
    fn id_field_for_entity() -> Field {
        let id_annotation = Annotation::new("Id".into());
        let id_annotation_strategy = Annotation::new("GeneratedValue".into())
            .param("strategy".into(), "GenerationType.IDENTITY".into());
        let entity_annotation = Annotation::new("Entity".into());
        let id_field = Field::n("id".into(), TypeName::new("Long".into()))
            .annotation(id_annotation)
            .annotation(id_annotation_strategy);
        id_field
    }

    pub fn jpa_repository_of(jclass: JavaClass, cls_import: Import) -> Interface {
        let mut repo = Interface::new("".to_string(), jclass.class_name.clone() + "Repository");
        let find_by_id_method = Method::new(
            TypeName::new_with_generics(
                "Optional".into(),
                GenericParams::new(vec![jclass.class_name.clone()]),
            ),
            "findById".into(),
        )
        .param(VariableParam::new(
            TypeName::new("Long".into()),
            "id".into(),
        ));
        //todo add the import of the class above
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

    pub fn spring_boot_entity(jclass: JavaClass) -> JavaClass {
        let id_field = id_field_for_entity();
        let lombok_annots = vec![
            Annotation::new("Data".into()),
            Annotation::new("AllArgsConstructor".into()),
            Annotation::new("NoArgsConstructor".into()),
        ];
        let entity_annotation = Annotation::new("Entity".into());
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

    pub fn dto_from_class(jclass: JavaClass, class_import: Import) -> JavaClass {
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

    pub fn service_from_class(jclass: JavaClass, jpa_import: Import) -> JavaClass {
        let mut service = jclass.clone();
        service.class_name = jclass.class_name.clone() + "Service";
        service = service.annotation(Annotation::new("Service".into()));
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
                .param(VariableParam::new(
                    TypeName::new(repo_name),
                    "repository".into(),
                )),
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

    pub fn controller_from_class(
        jclass: JavaClass,
        service_import: Import,
        dto_import: Import,
    ) -> JavaClass {
        let id_path_variable = VariableParam::new("Long".into(), "id".into());
        let initial_class_name = jclass.class_name.clone();
        let mut controller = JavaClass::new(initial_class_name.clone() + "Controller", "".into());
        let post_mapping = Annotation::new("PostMapping".into());
        let get_mapping = Annotation::new("GetMapping".into());
        let get_mapping_id =
            Annotation::new("GetMapping".into()).param("value".into(), "/{id}".into());
        let delete_mapping_id =
            Annotation::new("DeleteMapping".into()).param("value".into(), "/{id}".into());
        let update_mapping_id =
            Annotation::new("PutMapping".into()).param("value".into(), "/{id}".into());
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

        controller = controller.import(service_import).import(dto_import);
        controller = controller
            .annotation(Annotation::new("RestController".into()))
            .annotation(Annotation::new("RequestMapping".into()).param(
                "value".into(),
                "/".to_owned() + &initial_class_name.to_lowercase(),
            ));

        let service_property = initial_class_name.to_lowercase() + "Service";
        let service_type = TypeName::new(initial_class_name);
        controller = controller.field(Field::n(service_property.clone(), service_type.clone()));
        let constructor = Method::new("".into(), controller.class_name.clone())
            .param(VariableParam::new(service_type, service_property))
            .code("return null;".into());
        controller = controller.method(constructor);

        //constructor

        controller
    }
}

// pub mod docs_builder {}
