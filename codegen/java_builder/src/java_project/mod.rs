//check what build systems are available
//implement a java project builder (for now only maven with spring boot codegen)
//select:
// - libraries
// - general options that have sensible defaults (model folder,docs folder, services folder etc. etc.)
// - extras: CRUDs + Search
pub mod maven_builder {
    use annotations::Annotation;
    use classes::JavaClass;
    use fields::Field;
    use imports::Import;
    use interfaces::Interface;
    use types::TypeName;

    use crate::java_structs::*;
    use std::fs::{self, create_dir, create_dir_all, remove_dir_all, write};

    use super::crud_builder::{jpa_repository_of, service_from_class, spring_boot_entity};

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
        services: Vec<JavaClass>,
        repos_folder: String,
        jpa_repos: Vec<Interface>,
        has_written_initial_files: bool,
    }

    impl MavenCodebase {
        pub fn write_initial_files(&mut self) {
            //todo write application.properties based on the pom.xml file
            if !self.has_written_initial_files {
                self.has_written_initial_files = true;
            } else {
                return;
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
        }
        pub fn new(pom_xml: PomXml, output_dir: &str) -> Self {
            let output_dir = output_dir.to_string();
            let main_default = output_dir.to_string() + "/src/main";
            let project_path = format!("{}.{}", &pom_xml.group_id, &pom_xml.artifact_id);
            let package_path = project_path.replace(".", "/");
            //the order those folders are being created matters,
            //create_dir_all fails if any of the parents exist

            let code_folder = main_default.to_owned() + &"/java/" + &package_path;
            if let Err(e) = create_dir_all(code_folder.clone()) {
                assert!(false, "Failed to create main/java folder, err {}", e);
            }

            match create_dir(code_folder.clone() + "/repositories") {
                Ok(r) => println!("Created repositories folder successfully"),
                Err(e) => println!("Failed to create /repositories folder {}", e),
            }
            match create_dir(code_folder.clone() + "/models") {
                Ok(r) => println!("Created models folder successfully"),
                Err(e) => println!("Failed to create /models folder {}", e),
            }
            match create_dir(code_folder.clone() + "/services") {
                Ok(r) => println!("Created services folder successfully"),
                Err(e) => println!("Failed to create /services folder {}", e),
            }
            let test_folder = output_dir.to_string() + &"/src/test/java/" + &package_path;
            if let Err(e) = create_dir_all(test_folder.clone()) {
                assert!(false, "Failed to create test folder, err {}", e);
            }
            let res_folder = main_default.to_owned() + &"/resources";
            if let Err(e) = create_dir(res_folder.clone()) {
                assert!(false, "Failed to create resources folder,err {}", e);
            }

            Self {
                pom_xml,
                root_folder: output_dir,
                code_folder,
                test_folder,
                res_folder,
                repos_folder: "repositories".into(),
                services_folder: "services".into(),
                models_folder: "models".into(),
                services: vec![],
                jpa_repos: vec![],
                entities: vec![],
                has_written_initial_files: false,
            }
        }

        //adds an entity model and the respective service and repo
        pub fn add_entity(mut self, jclass: JavaClass) -> Self {
            //i will be re-using this
            let model_import = format!("{}.models", self.pom_xml.get_root_package());
            let entity = spring_boot_entity(jclass.clone());
            let jpa_repo = jpa_repository_of(
                jclass.clone(),
                Import::new(model_import, jclass.clone().class_name),
            );
            let repo_import = format!("{}.repositories", self.pom_xml.get_root_package(),);

            let service = service_from_class(
                jclass.clone(),
                Import::new(repo_import, jpa_repo.name.clone()),
            );
            self.entities.push(entity);
            self.services.push(service);
            self.jpa_repos.push(jpa_repo);
            self
        }

        pub fn add_entities(mut self, jclasses: Vec<JavaClass>) -> Self {
            for jclass in jclasses {
                self = self.add_entity(jclass);
            }
            self
        }

        pub fn generate_code(&mut self) {
            self.write_initial_files();
            println!("Generating code");
            let entities_package = format!(
                "{}.{}",
                self.pom_xml.get_root_package(),
                &self.models_folder
            );
            for mut e in self.entities.clone().into_iter() {
                e = e.package(entities_package.clone());
                match fs::write(
                    self.code_folder.to_owned()
                        + "/"
                        + &self.models_folder
                        + "/"
                        + &e.class_name
                        + ".java",
                    e.generate_code(),
                ) {
                    Ok(r) => println!("Entities were successfully generated"),
                    Err(e) => println!("An error occurred when generating entities {}", e),
                }
            }
            let repos_package =
                format!("{}.{}", self.pom_xml.get_root_package(), &self.repos_folder);

            for mut repo in self.jpa_repos.clone().into_iter() {
                repo = repo.package(repos_package.clone());
                match fs::write(
                    self.code_folder.to_owned()
                        + "/"
                        + &self.repos_folder
                        + "/"
                        + &repo.name
                        + ".java",
                    repo.generate_code(),
                ) {
                    Ok(r) => println!("Successfully generated jpa repositories"),
                    Err(e) => println!("An error occurred when generating jpa repos {}", e),
                }
            }
            let services_package = format!(
                "{}.{}",
                self.pom_xml.get_root_package(),
                &self.services_folder
            );
            for mut s in self.services.clone().into_iter() {
                s = s.package(services_package.clone());
                match fs::write(
                    self.code_folder.to_owned()
                        + "/"
                        + &self.services_folder
                        + "/"
                        + &s.class_name
                        + ".java",
                    s.generate_code(),
                ) {
                    Ok(r) => println!("Services classes were successfully generated "),
                    Err(e) => println!("An error occurred when generating services {}", e),
                }
            }
        }
        pub fn init_mvn_project(&self) {
            //
            // if let Err(e) = create_dir_all(&self.code_folder) {
            //     assert!(false, "Failed to create main/java folder, err {}", e);
            // }
            //
            // if let Err(e) = create_dir_all(&self.test_folder) {
            //     assert!(false, "Failed to create test folder, err {}", e);
            // }
            // if let Err(e) = create_dir(&self.res_folder) {
            //     assert!(false, "Failed to create resources folder,err {}", e);
            // }
            // let entrypoint = create_spring_main_class(&self.pom_xml);
            // let mainfile = write(
            //     (&self.code_folder).to_owned() + "/" + &entrypoint.class_name + ".java",
            //     entrypoint.generate_code(),
            // );
            //
            // if let Err(e) = mainfile {
            //     assert!(false, "Main file could not be written err {}", e);
            // }
            //
            // if let Err(e) = write(
            //     (&self.root_folder).to_owned() + "/" + "pom.xml",
            //     &self.pom_xml.generate(),
            // ) {
            //     assert!(false, "pom.xml file could not be written err {}", e);
            // }
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
    pub fn create_spring_main_class(pom: &PomXml) -> JavaClass {
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

    pub struct PomXml {
        description: String,
        project_name: String,
        java: String,
        //add assertion that one dot is contained
        group_id: String,
        artifact_id: String,
        dependencies: Vec<Library>,
        //It has the same attributes thats why
        //+ a relative path ofc
        parent_pom: Library,
    }

    struct Library {
        group_id: String,
        artifact_id: String,
        version: Option<String>,
    }
    //TODO
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

        pub fn spring_boot_starter_actuator(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-actuator".into(),
            );
            self
        }

        pub fn spring_boot_starter_batch(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-batch".into(),
            );
            self
        }

        pub fn spring_boot_starter_data_jdbc(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-data-jdbc".into(),
            );
            self
        }

        pub fn spring_boot_starter_data_jpa(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-data-jpa".into(),
            );
            self
        }

        pub fn spring_boot_starter_data_ldap(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-data-ldap".into(),
            );
            self
        }

        pub fn spring_boot_starter_data_rest(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-data-rest".into(),
            );
            self
        }

        pub fn spring_boot_starter_mail(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-mail".into(),
            );
            self
        }

        pub fn spring_boot_starter_oauth2_authorization_server(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-oauth2-authorization-server".into(),
            );
            self
        }

        pub fn spring_boot_starter_oauth2_client(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-oauth2-client".into(),
            );
            self
        }

        pub fn spring_boot_starter_thymeleaf(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-thymeleaf".into(),
            );
            self
        }

        pub fn spring_boot_starter_web(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-web".into(),
            );
            self
        }

        pub fn spring_kafka(mut self) -> Self {
            self = self.add_library("org.springframework.kafka".into(), "spring-kafka".into());
            self
        }

        pub fn thymeleaf_extras_springsecurity6(mut self) -> Self {
            self = self.add_library(
                "org.thymeleaf.extras".into(),
                "thymeleaf-extras-springsecurity6".into(),
            );
            self
        }

        pub fn spring_boot_devtools(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-devtools".into(),
            );
            self
        }

        pub fn spring_boot_docker_compose(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-docker-compose".into(),
            );
            self
        }

        pub fn postgresql(mut self) -> Self {
            self = self.add_library("org.postgresql".into(), "postgresql".into());
            self
        }

        pub fn lombok(mut self) -> Self {
            self = self.add_library("org.projectlombok".into(), "lombok".into());
            self
        }

        pub fn spring_boot_starter_test(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-test".into(),
            );
            self
        }

        pub fn spring_boot_testcontainers(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-testcontainers".into(),
            );
            self
        }

        pub fn spring_batch_test(mut self) -> Self {
            self = self.add_library(
                "org.springframework.batch".into(),
                "spring-batch-test".into(),
            );
            self
        }

        pub fn spring_kafka_test(mut self) -> Self {
            self = self.add_library(
                "org.springframework.kafka".into(),
                "spring-kafka-test".into(),
            );
            self
        }

        pub fn spring_restdocs_mockmvc(mut self) -> Self {
            self = self.add_library(
                "org.springframework.restdocs".into(),
                "spring-restdocs-mockmvc".into(),
            );
            self
        }

        pub fn spring_security_test(mut self) -> Self {
            self = self.add_library(
                "org.springframework.security".into(),
                "spring-security-test".into(),
            );
            self
        }

        pub fn junit_jupiter(mut self) -> Self {
            self = self.add_library("org.testcontainers".into(), "junit-jupiter".into());
            self
        }

        pub fn kafka(mut self) -> Self {
            self = self.add_library("org.testcontainers".into(), "kafka".into());
            self
        }

        pub fn testcontainers_postgresql(mut self) -> Self {
            self = self.add_library("org.testcontainers".into(), "postgresql".into());
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
        java_structs::VariableParam,
        java_structs_tests::{
            Annotation, Field, GenericParams, Import, Interface, JavaClass, Method, TypeName,
        },
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

    pub fn dto_from_class(jclass: JavaClass) -> () {
        todo!();
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
}

// pub mod docs_builder {}
