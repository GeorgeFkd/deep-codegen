//here we will:
//
//check what build systems are available
//implement a java project builder (for now only maven with spring boot codegen)
//select:
// - libraries
// - general options that have sensible defaults (model folder,docs folder, services folder etc. etc.)
// - extras: CRUDs + Search
//
//
//
pub mod maven_builder {
    use classes::JavaClass;

    use crate::java_structs::*;
    use std::fs::{create_dir, create_dir_all, remove_dir_all, write};

    struct MavenCodebase {
        test_folder: String,
        code_folder: String,
        res_folder: String,
        //group_id.artifact_id
        root_package_path: String,
    }

    impl MavenCodebase {
        pub fn new(pom_xml: PomXml) -> Self {
            todo!();
            //populate the initial fields
        }

        fn init_mvn_project() {
            //create folders and main function
            //verify it compiles
            //give helper fns to user so he can create classes in the respective folders
        }
    }
    pub fn create_maven_project_folders(pom: PomXml, root_folder: &str) -> bool {
        let main_default = root_folder.to_string() + "/src/main";
        let project_path = pom.group_id.to_owned() + "." + &pom.artifact_id;
        let package_path = project_path.replace(".", "/");
        //the order those folders are being created matters,
        //create_dir_all fails if any of the parents exist
        let code_folder = main_default.to_owned() + &"/java/" + &package_path;
        if let Err(e) = create_dir_all(&code_folder) {
            assert!(false, "Failed to create main/java folder, err {}", e);
        }

        let test_folder = root_folder.to_string() + &"/src/test/java/" + &package_path;
        if let Err(e) = create_dir_all(test_folder) {
            assert!(false, "Failed to create test folder, err {}", e);
        }

        let resources_folder = main_default.to_owned() + &"/resources";
        if let Err(e) = create_dir(resources_folder) {
            assert!(false, "Failed to create resources folder,err {}", e);
        }

        let entrypoint = create_spring_main_class(pom);
        let mainfile = write(
            code_folder + "/" + &entrypoint.class_name + ".java",
            entrypoint.generate_code(),
        );
        if let Err(e) = mainfile {
            assert!(false, "Main file could not be written err {}", e);
        }

        true
    }
    //https://nick.groenen.me/notes/capitalize-a-string-in-rust/
    pub fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
    pub fn create_spring_main_class(pom: PomXml) -> JavaClass {
        let class_name = capitalize(&pom.project_name);
        let package = pom.group_id.to_owned() + "." + &pom.artifact_id;
        let jclass = JavaClass::new(class_name.clone(), package)
            .import(imports::Import::new(
                "org.springframework.boot".to_owned(),
                "SpringApplication".to_owned(),
            ))
            .annotation(annotations::Annotation::new("SpringApplication".into()))
            .public()
            .method(
                methods::Method::new(types::TypeName::new("void".into()), "main".to_owned())
                    .static_()
                    .public()
                    .param(VariableParam::new(
                        types::TypeName::new("String[]".into()),
                        "args".into(),
                    ))
                    .code(format!("SpringApplication.run({},args);", class_name)),
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
            result += &(r#"<description>"#.to_owned() + &self.description + &"</description>\n");
            result += &(r#"<name>"#.to_owned() + &self.project_name + &"</name>\n");
            result += &("<groupId>".to_owned() + &self.group_id + &"</groupId>\n");
            result += &self.dependencies.generate();
            //will write a macro for this
            //

            result += &("<artifactId>".to_owned() + &self.artifact_id + &"</artifactId>");
            result += "<properties>\n";
            result += &("<java.version>".to_owned() + &self.java + &"</java.version>");
            result += "\n</properties>";

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
}

// pub mod docs_builder {}
