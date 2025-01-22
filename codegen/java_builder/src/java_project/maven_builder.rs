use crate::java_structs::*;

use classes::JavaClass;
use imports::Import;
use interfaces::Interface;
use methods::Method;
use std::{
    collections::HashMap,
    fs::{remove_dir_all, write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};
use types::TypeName;

use super::{
    crud_builder::CrudBuilder,
    output::OutputDirs,
    pom_xml::{Generate, PomXml},
};

//TODO Do security in spring boot
//All paths need an auth except /login
//Also need an admin thingie

//TODO dockerise application(docker-compose and cloud)
//TODO Github Actions for uploading to container registry
//TODO Polish front facing API and upload to crates.io

struct Progress {
    pub has_written_initial_files: bool,
    pub has_created_initial_folders: bool,
    pub has_created_application_properties: bool,
}

#[derive(Clone)]
pub struct DBInfo {
    pub db_port: u16,
    pub username: String,
    pub password: String,
    pub db: String,
}

fn create_db_info(pom_xml: &PomXml) -> DBInfo {
    DBInfo {
        db_port: 5432,
        username: pom_xml.project_info.name.to_ascii_lowercase(),
        password: pom_xml.project_info.name.to_ascii_lowercase(),
        db: pom_xml.project_info.name.to_ascii_lowercase() + "_db",
    }
}

//TODO make an OutputConfig option that contains the folders and the package names and everything

pub struct MavenCodebase {
    port: u16,
    db_info: DBInfo,
    root_folder: PathBuf,
    pom_xml: PomXml,
    out_dirs: OutputDirs,
    entities: Vec<JavaClass>,
    controller_classes: Vec<JavaClass>,
    dto_classes: Vec<JavaClass>,
    services: Vec<JavaClass>,
    jpa_repos: Vec<Interface>,
    progress: Progress,
}

impl MavenCodebase {
    pub fn get_server_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port)
    }

    pub fn get_db_info(&self) -> DBInfo {
        self.db_info.clone()
    }
    pub fn get_db_port(&self) -> u16 {
        self.db_info.db_port
    }
    fn create_initial_folders(&mut self) {
        if !self.progress.has_created_initial_folders {
            self.progress.has_created_initial_folders = true;
        } else {
            return;
        }

        self.out_dirs.create_folders();
    }

    pub fn create_application_properties(&mut self) -> String {
        let mut app: HashMap<&str, &str> = HashMap::new();
        app.insert("spring.application.version", "0.0.1");
        app.insert("spring.config.validate", "true");
        app.insert("server.address", "localhost");
        let binding = self.port.to_string();
        app.insert("server.port", &binding);

        let mut db_url = "".to_string();
        if self
            .pom_xml
            .has_dependency_that(|d| d.artifact_id.contains("postgresql"))
        {
            db_url = "jdbc:postgresql://localhost:".to_owned()
                + &self.db_info.db_port.clone().to_string()
                + "/"
                + &self.db_info.db;
            app.insert("spring.datasource.url", &db_url);

            app.insert("spring.datasource.username", &self.db_info.username);

            app.insert("spring.datasource.password", &self.db_info.password);

            app.insert(
                "spring.datasource.driver-class-name",
                "org.postgresql.Driver",
            );
            app.insert(
                "spring.jpa.database-platform",
                "org.hibernate.dialect.PostgreSQLDialect",
            );

            app.insert("spring.jpa.hibernate.ddl-auto", "update");
            app.insert("spring.jpa.show-sql", "true");
            app.insert("spring.jpa.properties.hibernate.format_sql", "true");

            //optionally add Hikari Connection pool stuff
        }

        if self
            .pom_xml
            .has_dependency_that(|d| d.artifact_id == "springdoc-openapi-starter-webmvc-api")
        {
            app.insert("springdoc.api-docs.path", "/api-docs");
            app.insert("springdoc.show-actuator", "true");
        }

        if self
            .pom_xml
            .has_dependency_that(|d| d.artifact_id == "springdoc-openapi-starter-webmvc-ui")
        {
            app.insert("springdoc.swagger-ui.path", "/docs.html");
            app.insert("springdoc.show-actuator", "true");
        }

        app.into_iter()
            .filter(|(_, v)| !v.is_empty())
            .fold("".into(), |acc, (key, value)| {
                acc + key + "=" + value + "\n"
            })
    }
    pub fn write_initial_files(&mut self) {
        if self.progress.has_written_initial_files {
            println!("Have already written initial_files, skipping");
            return;
        }

        if !self.progress.has_created_initial_folders {
            self.create_initial_folders();
        }

        let entrypoint = self.create_spring_main_class();
        let mut main = self.out_dirs.code_folder().clone();
        main.push(&entrypoint.class_name);
        main.set_extension("java");
        let mainfile = write(main, (&entrypoint).generate_code());

        if let Err(e) = mainfile {
            assert!(false, "Main file could not be written err {}", e);
        }

        let mut pom_path = PathBuf::from(self.root_folder.as_path());
        pom_path.push("pom");
        pom_path.set_extension("xml");
        if let Err(e) = write(&pom_path, &self.pom_xml.generate()) {
            assert!(false, "pom.xml file could not be written err {}", e);
        }
        if !self.progress.has_created_application_properties {
            let app_properties = self.create_application_properties();
            let mut properties_location = self.out_dirs.resources_folder().clone();
            properties_location.push("application.properties");
            match write(properties_location, app_properties) {
                Ok(r) => println!("Succesfully wrote application.properties"),
                Err(e) => assert!(
                    false,
                    "Could not write spring application.properties file err: {e}"
                ),
            }
        }

        self.progress.has_created_application_properties = true;
        self.progress.has_written_initial_files = true;
    }

    pub fn new(pom_xml: PomXml, output_dir: &str) -> Self {
        let package_path = format!(
            "{}.{}",
            &pom_xml.project_info.group_id, &pom_xml.project_info.artifact_id
        );
        let controllers_suffix = "controllers".to_owned();
        let dtos_suffix = "dto".to_owned();
        let repos_suffix = "repositories".to_owned();
        let services_suffix = "services".to_owned();
        let models_suffix = "models".to_owned();
        let out_dirs = OutputDirs::new(output_dir.to_owned(), package_path.to_owned())
            .models(models_suffix)
            .controllers(controllers_suffix)
            .dtos(dtos_suffix)
            .services(services_suffix)
            .repos(repos_suffix);
        let mut root_folder = PathBuf::new();
        root_folder.push(output_dir);
        Self {
            port: 8082,
            db_info: create_db_info(&pom_xml),
            pom_xml,
            root_folder,
            out_dirs,
            services: vec![],
            jpa_repos: vec![],
            entities: vec![],
            dto_classes: vec![],
            controller_classes: vec![],
            progress: Progress {
                has_written_initial_files: false,
                has_created_initial_folders: false,
                has_created_application_properties: false,
            },
        }
    }

    fn models_package(&self) -> String {
        [&self.pom_xml.get_root_package(), "repositories"].join(".")
    }

    fn repositories_package(&self) -> String {
        [&self.pom_xml.get_root_package(), "repositories"].join(".")
    }

    fn services_package(&self) -> String {
        [&self.pom_xml.get_root_package(), "services"].join(".")
    }

    fn controllers_package(&self) -> String {
        [&self.pom_xml.get_root_package(), "services"].join(".")
    }

    fn dto_package(&self) -> String {
        [&self.pom_xml.get_root_package(), "dto"].join(".")
    }

    //adds an entity model and the respective service and repo
    pub fn add_entity(mut self, jclass: JavaClass) -> Self {
        let model_import = Import::new(self.models_package(), jclass.class_name.clone());
        let crud_build = CrudBuilder::new(jclass);
        let entity = crud_build.spring_boot_entity();
        let jpa_repo = crud_build.jpa_repository_of(model_import.clone());

        let service = crud_build.service_from_class(Import::new(
            self.repositories_package(),
            jpa_repo.name.clone(),
        ));

        let dto = crud_build.dto_from_class(model_import);
        let controller = crud_build.controller_from_class(
            Import::new(self.services_package(), service.class_name.clone()),
            Import::new(self.dto_package(), dto.class_name.clone()),
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
    fn create_spring_main_class(&self) -> JavaClass {
        let class_name = capitalize(&self.pom_xml.project_info.name);
        let package = self.pom_xml.get_root_package();
        let jclass = JavaClass::new(class_name.clone(), package)
            .import(Import::new(
                "org.springframework.boot".to_owned(),
                "SpringApplication".to_owned(),
            ))
            .import(Import::new(
                "org.springframework.boot.autoconfigure".into(),
                "SpringBootApplication".into(),
            ))
            .annotation("SpringBootApplication".into())
            .public()
            .method(
                Method::new(TypeName::new("void".into()), "main".to_owned())
                    .static_()
                    .public()
                    .param(VariableParam::new(
                        TypeName::new("String[]".into()),
                        "args".into(),
                    ))
                    .code(format!("SpringApplication.run({}.class,args);", class_name)),
            );
        jclass
    }

    fn put_classes_in_packages(&mut self) {
        let in_package = self.models_package();
        for i in 0..self.entities.len() {
            let cls = &mut self.entities[i];
            cls.package_in_place(in_package.clone());
        }
        let in_package = self.services_package();
        for i in 0..self.services.len() {
            let cls = &mut self.services[i];
            cls.package_in_place(in_package.clone());
        }
        let in_package = self.controllers_package();
        for i in 0..self.controller_classes.len() {
            let cls = &mut self.controller_classes[i];
            cls.package_in_place(in_package.clone());
        }

        let in_package = self.repositories_package();
        for i in 0..self.jpa_repos.len() {
            let cls = &mut self.jpa_repos[i];
            cls.package_in_place(in_package.clone());
        }

        let in_package = self.dto_package();
        for i in 0..self.dto_classes.len() {
            let cls = &mut self.dto_classes[i];
            cls.package_in_place(in_package.clone());
        }
    }
    pub fn generate_code(&mut self) {
        self.create_initial_folders();
        self.write_initial_files();
        self.put_classes_in_packages();
        println!("Generating code");

        self.out_dirs.generate_classes_in(
            &self.entities,
            self.out_dirs.models_folder().to_str().unwrap(),
        );

        self.out_dirs.generate_interfaces_in(
            &self.jpa_repos,
            self.out_dirs.repos_folder().to_str().unwrap(),
        );

        self.out_dirs.generate_classes_in(
            &self.services,
            self.out_dirs.services_folder().to_str().unwrap(),
        );

        self.out_dirs.generate_classes_in(
            &self.controller_classes,
            self.out_dirs.controllers_folder().to_str().unwrap(),
        );

        self.out_dirs.generate_classes_in(
            &self.dto_classes,
            self.out_dirs.dtos_folder().to_str().unwrap(),
        );
    }

    pub fn extract_to_zip(&self) -> PathBuf {
        self.out_dirs.extract_to_zip()
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

pub fn cleanup_folder(dir: &str) {
    if let Err(e) = remove_dir_all(dir) {
        assert!(false, "Removing all files from folder failed");
    }
}
