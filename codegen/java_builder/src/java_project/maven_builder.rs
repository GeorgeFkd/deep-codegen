use crate::java_structs::*;
use annotations::Annotation;
use classes::JavaClass;
use imports::Import;
use interfaces::Interface;
use methods::Method;
use rand::Rng;
use std::{
    fs::{self, create_dir, create_dir_all, remove_dir_all, write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
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
    port: u16,
    test_folder: String,
    code_folder: String,
    res_folder: String,
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

//those 2 functions can be merged if there is a change in the structs
fn generate_interfaces_in(interfaces: Vec<Interface>, folder: String) {
    interfaces.into_iter().for_each(|cls| {
        let path = folder.to_owned() + "/" + &cls.name + ".java";
        match fs::write(path, cls.generate_code()) {
            Ok(r) => println!("Entities were successfully generated"),
            Err(e) => println!("An error occurred when generating entities {}", e),
        }
    });
}

fn generate_classes_in(classes: Vec<JavaClass>, folder: String) {
    classes.iter().for_each(|cls| {
        let path = folder.to_owned() + "/" + &cls.class_name + ".java";
        match fs::write(path, cls.generate_code()) {
            Ok(r) => println!("Entities were successfully generated"),
            Err(e) => println!("An error occurred when generating entities {}", e),
        }
    });
}
impl MavenCodebase {
    pub fn get_server_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port)
    }
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

    fn create_application_properties(&mut self) -> String {
        let mut application_properties_file = "".to_string();
        application_properties_file += "spring.config.validate=true\n";
        application_properties_file += "server.address=localhost\n";
        application_properties_file += "server.port=";
        application_properties_file += &self.port.to_string();
        application_properties_file.push('\n');
        if self
            .pom_xml
            .has_dependency_that(|d| d.artifact_id.contains("postgresql"))
        {
            let app_name = &self.pom_xml.project_name.to_ascii_lowercase();
            //can easily be extended for other DBs
            let db_url = "spring.datasource.url=jdbc:postgresql://localhost:5432/"
                .to_ascii_lowercase()
                + app_name
                + "_db\n";
            application_properties_file += &db_url;

            let username = "spring.datasource.username=".to_owned() + app_name;
            application_properties_file += &username;
            application_properties_file.push('\n');

            let password = "spring.datasource.password=".to_owned() + app_name;
            application_properties_file += &password;

            let pg_user = "-e POSTGRES_USER=".to_owned() + app_name;
            let pg_passwd = "-e POSTGRES_PASSWORD=".to_owned() + app_name;
            let pg_db = "-e POSTGRES_DB=".to_owned() + app_name + "_db";
            //docker run --name postgres-container -e POSTGRES_USER=myuser -e POSTGRES_PASSWORD=mypassword -e POSTGRES_DB=mydb -p 5432:5432 -d postgres
            // let res = Command::new("sudo")
            //     .arg("docker")
            //     .arg("run")
            //     .arg("--name")
            //     .arg("rust-postgres-container")
            //     .arg(pg_user)
            //     .arg(pg_passwd)
            //     .arg(pg_db)
            //     .arg("-p")
            //     .arg("5432:5432")
            //     .arg("-d")
            //     .arg("postgres")
            //     .status()
            //     .expect("Could not spin up docker postgresql");
            // println!("Status from postgresql command: {res}");
            application_properties_file.push('\n');
            application_properties_file +=
                "spring.datasource.driver-class-name=org.postgresql.Driver\n";

            application_properties_file +=
                "spring.jpa.database-platform=org.hibernate.dialect.PostgreSQLDialect\n";

            application_properties_file += "spring.jpa.hibernate.ddl-auto=update\n";

            application_properties_file += "spring.jpa.show-sql=true\n";

            application_properties_file += "spring.jpa.properties.hibernate.format_sql=true\n";

            //optionally add Hikari Connection pool stuff
        }

        application_properties_file
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
        let app_properties = self.create_application_properties();
        match write(
            self.res_folder.clone() + "/" + "application.properties",
            app_properties,
        ) {
            Ok(r) => println!("Succesfully wrote application.properties"),
            Err(e) => assert!(false, "Could not write spring application.properties file"),
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
        let mut rng = rand::thread_rng();
        rng.gen_range(8082..=9000);
        Self {
            port: rng.gen::<u16>(),
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

        let service_import = format!("{}.services", self.pom_xml.get_root_package());
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

    fn put_classes_in_packages(&mut self) {
        let root_pkg = self.pom_xml.get_root_package();
        let in_package = format!("{}.{}", root_pkg, &self.models_folder);
        for i in 0..self.entities.len() {
            let cls = &mut self.entities[i];
            cls.package_in_place(in_package.clone());
        }
        let in_package = format!("{}.{}", root_pkg, &self.services_folder);
        for i in 0..self.services.len() {
            let cls = &mut self.services[i];
            cls.package_in_place(in_package.clone());
        }
        let in_package = format!(
            "{}.{}",
            self.pom_xml.get_root_package(),
            &self.controllers_folder
        );
        for i in 0..self.controller_classes.len() {
            let cls = &mut self.controller_classes[i];
            cls.package_in_place(in_package.clone());
        }

        let in_package = format!("{}.{}", root_pkg, &self.repos_folder);
        for i in 0..self.jpa_repos.len() {
            let cls = &mut self.jpa_repos[i];
            cls.package_in_place(in_package.clone());
        }

        let in_package = format!("{}.{}", root_pkg, &self.dtos_folder);
        for i in 0..self.dto_classes.len() {
            let cls = &mut self.dto_classes[i];
            cls.package_in_place(in_package.clone());
        }
    }
    pub fn generate_code(mut self) {
        self.create_initial_folders();
        self.write_initial_files();
        self.put_classes_in_packages();
        println!("Generating code");

        generate_classes_in(
            self.entities,
            self.code_folder.to_owned() + "/" + &self.models_folder,
        );
        generate_interfaces_in(
            self.jpa_repos,
            self.code_folder.to_owned() + "/" + &self.repos_folder,
        );

        generate_classes_in(
            self.services,
            self.code_folder.to_owned() + "/" + &self.services_folder,
        );

        generate_classes_in(
            self.controller_classes,
            self.code_folder.to_owned() + "/" + &self.controllers_folder,
        );

        generate_classes_in(
            self.dto_classes,
            self.code_folder.to_owned() + "/" + &self.dtos_folder,
        );
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

//This should go inside the class and not be a public class
pub fn create_spring_main_class(pom: &PomXml) -> JavaClass {
    let class_name = capitalize(&pom.project_name);
    let package = pom.group_id.to_owned() + "." + &pom.artifact_id;
    let jclass = JavaClass::new(class_name.clone(), package)
        .import(Import::new(
            "org.springframework.boot".to_owned(),
            "SpringApplication".to_owned(),
        ))
        .import(Import::new(
            "org.springframework.boot.autoconfigure".into(),
            "SpringBootApplication".into(),
        ))
        .annotation(Annotation::new("SpringBootApplication".into()))
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

fn cleanup_folder(dir: &str) {
    if let Err(e) = remove_dir_all(dir) {
        assert!(false, "Removing all files from folder failed");
    }
}
