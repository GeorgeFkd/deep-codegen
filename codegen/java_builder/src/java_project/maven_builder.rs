use crate::java_structs::*;
use annotations::Annotation;
use classes::JavaClass;
use imports::Import;
use interfaces::Interface;
use methods::Method;
use std::{
    collections::HashMap,
    fs::{self, create_dir, create_dir_all, remove_dir_all, write, DirEntry, File},
    io::{Read, Seek},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
};
use types::TypeName;
use zip::{
    self,
    result::ZipError,
    write::{FileOptions, SimpleFileOptions},
    ZipWriter,
};
// use zip::{
//     result::{ZipError, ZipResult},
//     write::FileOptions,
//     ZipWriter,
// };

use super::{
    crud_builder::{
        CrudBuilder, // controller_from_class, dto_from_class, jpa_repository_of, service_from_class,
                     // spring_boot_entity,
    },
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

struct OutputDirs {
    package_path: String,
    output_dir: String,
    controllers_suffix: String,
    repos_suffix: String,
    services_suffix: String,
    dtos_suffix: String,
    models_suffix: String,
    code_folder: PathBuf,
}

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
fn find_files_in_dir_recursive(path: impl AsRef<Path>) -> Vec<DirEntry> {
    let Ok(entries) = fs::read_dir(path) else {
        return vec![];
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let Ok(meta) = entry.metadata() else {
                return vec![];
            };
            if meta.is_dir() {
                return find_files_in_dir_recursive(entry.path());
            }
            if meta.is_file() {
                return vec![entry];
            }
            vec![]
        })
        .collect()
}
use std::io::Write;
fn zip_dir<T, U: Iterator<Item = DirEntry>>(
    it: &mut U,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> Result<(), ZipError>
where
    T: std::io::Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    let prefix = Path::new(prefix);
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();
        let path_as_str = name.to_str().map(str::to_owned).unwrap();
        if path.is_file() {
            println!("adding file {:?} as {:?}", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            println!("adding dir {path_as_str:?} as {name:?} ...");
            zip.add_directory(path_as_str, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

impl OutputDirs {
    pub fn new(output_dir: String, package_path: String) -> Self {
        let mut code_folder = PathBuf::new();
        code_folder.push(".");
        code_folder.push(&output_dir);
        code_folder.push("src");
        code_folder.push("main");
        code_folder.push("java");
        for pth in package_path.split(".") {
            code_folder.push(pth);
        }

        Self {
            package_path,
            output_dir,
            code_folder,
            dtos_suffix: "".to_owned(),
            repos_suffix: "".to_owned(),
            controllers_suffix: "".to_owned(),
            services_suffix: "".to_owned(),
            models_suffix: "".to_owned(),
        }
    }
    pub fn dtos(mut self, suffix: String) -> Self {
        self.dtos_suffix = suffix;
        self
    }
    pub fn repos(mut self, suffix: String) -> Self {
        self.repos_suffix = suffix;
        self
    }
    pub fn controllers(mut self, suffix: String) -> Self {
        self.controllers_suffix = suffix;
        self
    }
    pub fn services(mut self, suffix: String) -> Self {
        self.services_suffix = suffix;
        self
    }
    pub fn models(mut self, suffix: String) -> Self {
        self.models_suffix = suffix;
        self
    }

    pub fn code_folder(&self) -> &PathBuf {
        &self.code_folder
    }
    pub fn controllers_folder(&self) -> PathBuf {
        let mut controllers_folder = self.code_folder.clone();
        controllers_folder.push(&self.controllers_suffix);
        controllers_folder
    }
    pub fn models_folder(&self) -> PathBuf {
        let mut controllers_folder = self.code_folder.clone();
        controllers_folder.push(&self.models_suffix);
        controllers_folder
    }
    pub fn services_folder(&self) -> PathBuf {
        let mut controllers_folder = self.code_folder.clone();
        controllers_folder.push(&self.services_suffix);
        controllers_folder
    }
    pub fn repos_folder(&self) -> PathBuf {
        let mut controllers_folder = self.code_folder.clone();
        controllers_folder.push(&self.repos_suffix);
        controllers_folder
    }
    pub fn dtos_folder(&self) -> PathBuf {
        let mut controllers_folder = self.code_folder.clone();
        controllers_folder.push(&self.dtos_suffix);
        controllers_folder
    }

    pub fn tests_folder(&self) -> PathBuf {
        let mut test_folder = PathBuf::new();
        test_folder.push(".");
        test_folder.push(&self.output_dir);
        test_folder.push("src");
        test_folder.push("test");
        test_folder.push("java");
        for pth in self.package_path.split(".") {
            test_folder.push(pth);
        }
        test_folder
    }
    pub fn resources_folder(&self) -> PathBuf {
        let mut res_folder = PathBuf::new();
        res_folder.push(".");
        res_folder.push(&self.output_dir);
        res_folder.push("src");
        res_folder.push("main");
        res_folder.push("resources");
        res_folder
    }
    pub fn extract_to_zip(&self) -> PathBuf {
        let output_path = Path::new("generated-new.zip");
        let mut files = find_files_in_dir_recursive(Path::new(&self.output_dir));
        let new_file = File::create(output_path).unwrap();
        let _ = zip_dir(
            &mut files.into_iter(),
            &self.output_dir,
            new_file,
            zip::CompressionMethod::Bzip2,
        );
        output_path.to_owned()
    }
    pub fn create_folders(&self) {
        //the order those folders are being created matters,
        //create_dir_all fails if any of the parents exist

        if let Err(e) = create_dir_all(&self.code_folder()) {
            assert!(false, "Failed to create main/java folder, err {}", e);
        }

        match create_dir(&self.repos_folder()) {
            Ok(r) => println!("Created repositories folder successfully"),
            Err(e) => println!("Failed to create {:?} folder {}", &self.repos_folder(), e),
        }
        match create_dir(&self.models_folder()) {
            Ok(r) => println!("Created models folder successfully"),
            Err(e) => println!("Failed to create {:?} folder {}", &self.models_folder(), e),
        }
        match create_dir(&self.services_folder()) {
            Ok(r) => println!("Created services folder successfully"),
            Err(e) => println!(
                "Failed to create {:?} folder {}",
                &self.services_folder(),
                e
            ),
        }

        if let Err(e) = create_dir(&self.dtos_folder()) {
            assert!(
                false,
                "Failed to create {:?} folder, err {}",
                &self.dtos_folder(),
                e
            );
        }

        if let Err(e) = create_dir(self.controllers_folder()) {
            assert!(
                false,
                "Failed to create {:?} folder, err {}",
                &self.controllers_folder(),
                e
            );
        }

        if let Err(e) = create_dir_all(&self.tests_folder()) {
            assert!(
                false,
                "Failed to create {:?} folder, err {}",
                &self.tests_folder(),
                e
            );
        }
        if let Err(e) = create_dir(&self.resources_folder()) {
            assert!(
                false,
                "Failed to create {:?} folder,err {}",
                &self.resources_folder(),
                e
            );
        }
    }

    fn controllers_suffix(&self) -> &str {
        &self.controllers_suffix
    }

    fn repos_suffix(&self) -> &str {
        &self.repos_suffix
    }

    fn services_suffix(&self) -> &str {
        &self.services_suffix
    }

    fn dtos_suffix(&self) -> &str {
        &self.dtos_suffix
    }

    fn models_suffix(&self) -> &str {
        &self.models_suffix
    }
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

        let mut pom_path = self.root_folder.clone();
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
        format!("{}.models", self.pom_xml.get_root_package())
    }

    fn repositories_package(&self) -> String {
        format!("{}.repositories", self.pom_xml.get_root_package())
    }

    fn services_package(&self) -> String {
        format!("{}.services", self.pom_xml.get_root_package())
    }

    fn controllers_package(&self) -> String {
        format!("{}.controllers", self.pom_xml.get_root_package())
    }

    fn dto_package(&self) -> String {
        format!("{}.dto", self.pom_xml.get_root_package())
    }

    //adds an entity model and the respective service and repo
    pub fn add_entity(mut self, jclass: JavaClass) -> Self {
        let model_import = Import::new(self.models_package(), jclass.clone().class_name);
        let crud_build = CrudBuilder::new(jclass);
        let entity = crud_build.spring_boot_entity();
        let jpa_repo = crud_build.jpa_repository_of(model_import.clone());

        let service = crud_build.service_from_class(Import::new(
            self.repositories_package(),
            jpa_repo.name.clone(),
        ));

        let dto = crud_build.dto_from_class(model_import.clone());
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

        //theses calls could be completely written in the OutputDirs class
        self.generate_classes_in(&self.entities, self.out_dirs.models_folder());

        self.generate_interfaces_in(&self.jpa_repos, self.out_dirs.repos_folder());

        self.generate_classes_in(&self.services, self.out_dirs.services_folder());

        self.generate_classes_in(&self.controller_classes, self.out_dirs.controllers_folder());

        self.generate_classes_in(&self.dto_classes, self.out_dirs.dtos_folder());
    }
    fn generate_classes_in(&self, classes: &[JavaClass], folder: PathBuf) {
        classes.iter().for_each(|cls| {
            let mut path = folder.clone();
            path.push(&cls.class_name);
            path.set_extension("java");

            match fs::write(path, cls.generate_code()) {
                Ok(r) => println!("Entities were successfully generated"),
                Err(e) => println!("An error occurred when generating entities {}", e),
            }
        });
    }
    fn generate_interfaces_in(&self, interfaces: &[Interface], folder: PathBuf) {
        interfaces.into_iter().for_each(|cls| {
            let mut path = folder.clone();
            path.push(&cls.name);
            path.set_extension("java");
            match fs::write(path, cls.generate_code()) {
                Ok(r) => println!("Entities were successfully generated"),
                Err(e) => println!("An error occurred when generating entities {}", e),
            }
        });
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
