use std::{
    env, fs,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};
use tree_sitter::Parser;
pub fn sample_class(pom_xml: &PomXml) -> JavaClass {
    let customer_class = JavaClass::new(
        "Customer".to_owned(),
        pom_xml.get_root_package() + "Customer",
    )
    .public()
    .field(Field::n("firstName".into(), TypeName::new("String".into())))
    .field(Field::n("lastName".into(), TypeName::new("String".into())))
    .field(Field::n("email".into(), TypeName::new("String".into())))
    .field(Field::n("age".into(), TypeName::new("int".into())));
    customer_class
}
fn maven_is_installed() -> bool {
    let mvn = Command::new("mvn")
        .arg("--version")
        .stdout(Stdio::null())
        .status();
    if let Ok(command_result) = mvn {
        return command_result.success();
    } else {
        println!("Something went wrong when executing xmllint");
        return false;
    }
}
pub fn mvn_project_compiles(project_root: &str) -> bool {
    assert!(
        maven_is_installed(),
        "mvn command is not present, install the maven package from your package manager"
    );
    let pom_path =
        folder_pom_xml_file(project_root).expect("There was no pom.xml in the folder provided");

    let mut mvn = Command::new("mvn");
    mvn.arg("-f").arg(pom_path).arg("clean").arg("compile");
    if let Ok(res) = mvn.status() {
        return res.success();
    } else {
        false
    }
}
fn xml_lint_is_installed() -> bool {
    //just running the command returns a help message and exit code == 1
    let xmllint = Command::new("xmllint")
        .arg("--version")
        .stdout(Stdio::null())
        .status();
    if let Ok(command_result) = xmllint {
        return command_result.success();
    } else {
        println!("Something went wrong when executing xmllint");
        return false;
    }
}
pub fn assert_xml_structure_with_xsd(res: &str) {
    assert!(xml_lint_is_installed(), "Xmllint command is not present");
    let _ = fs::write("tmp.xml", res).expect("writing to temp file failed");
    let xmllint = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg("./maven-4.0.0.xsd")
        .arg("./tmp.xml")
        .status()
        .expect("Something went wrong when executing the xmllint command");
    assert!(xmllint.success(), "Xml linting failed");
}
pub fn assert_dir_exists(dir: &str) {
    assert!(
        Path::new(dir).is_dir(),
        "{} folder was not created properly",
        dir
    );
}
fn make_java_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_java::LANGUAGE.into())
        .expect("Error loading java grammar");
    parser
}

pub fn assert_program_is_syntactically_correct(java_str: &str) {
    let mut parser = make_java_parser();
    let tree = parser.parse(java_str, None).unwrap();
    assert!(!tree.root_node().has_error());
}

use java_builder::{
    classes::JavaClass,
    fields::Field,
    maven_builder::DBInfo,
    pom_xml::{PomXml, ProjectInfo},
    types::TypeName,
};
use std::net::SocketAddr;

fn run_postgres_container(db: &DBInfo) -> ExitStatus {
    let pg_user = "-e POSTGRES_USER=".to_owned() + &db.username;
    let pg_passwd = "-e POSTGRES_PASSWORD=".to_owned() + &db.password;
    let pg_db = "-e POSTGRES_DB=".to_owned() + &db.db;

    let res = Command::new("docker")
        .arg("run")
        .arg("--name")
        .arg("rust-postgres-container")
        .arg("--replace")
        .arg(pg_user)
        .arg(pg_passwd)
        .arg(pg_db)
        .arg("-p")
        .arg("5432:5432")
        .arg("-d")
        .arg("postgres")
        .status()
        .expect("Could not spin up docker postgresql");
    res
}

fn kill_postgres_container(_db: &DBInfo) -> std::io::Result<ExitStatus> {
    Command::new("docker")
        .arg("stop")
        .arg("rust-postgres-container")
        .status()
}

fn folder_pom_xml_file(project_root: &str) -> Option<PathBuf> {
    let cwd = env::current_dir().unwrap();
    let pom_path = cwd.join(project_root).join("pom.xml");
    if pom_path.exists() {
        return Some(pom_path);
    } else {
        return None;
    }
}

pub fn assert_spring_server_is_up(server_location: SocketAddr, db: &DBInfo, project_root: &str) {
    assert!(
        maven_is_installed(),
        "mvn command is not present, install the maven package from your package manager"
    );
    let pom_path =
        folder_pom_xml_file(project_root).expect("There was no pom.xml in the folder provided");
    let res = run_postgres_container(db);
    println!("Result of docker run postgres {res}");
    let mut mvn = Command::new("mvn");
    mvn.arg("-f").arg(pom_path).arg("spring-boot:run");
    let mut result = mvn
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run mvn spring-boot:run for project");
    println!("Command output was: {:?}", &result);
    match result.stdout.as_mut() {
        Some(out) => {
            let buf_reader = BufReader::new(out);
            for line in buf_reader.lines() {
                match line {
                    Ok(s) => {
                        if s.contains("Tomcat started on port") {
                            break;
                        }
                    }
                    Err(e) => println!("An error occurred : {e}"),
                }
            }
        }
        None => return,
    }
    let curl_localhost = Command::new("curl")
        .arg(server_location.to_string())
        .status()
        .expect("Something went wrong when executing curl")
        .success();
    match kill_postgres_container(db) {
        Ok(ex_status) => {
            println!("Docker postgres container was stopped properly with status {ex_status}")
        }
        Err(e) => println!("There was an error when stopping postgres container {e}"),
    }
    match result.kill() {
        Ok(r) => println!("Spring server was properly shutdown"),
        Err(e) => println!("Spring server could not be killed"),
    };
    assert!(
        curl_localhost,
        "Curl command to localhost failed, spring server errored"
    );
}
fn find_java_files_in_recursively(path: impl AsRef<Path>) -> Vec<PathBuf> {
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
                return find_java_files_in_recursively(entry.path());
            }
            if meta.is_file() && entry.path().extension().unwrap() == "java" {
                return vec![entry.path()];
            }
            vec![]
        })
        .collect()
}

pub fn sample_project_info() -> ProjectInfo {
    ProjectInfo {
            name: "TempContRvTool".to_owned(),
            version: "".to_owned(),
            group_id: "org.javacodegen".to_owned(),
            artifact_id: "rvtool".to_owned(),
            description:"This is a project to showcase the methodology of runtime verification in the context of event based systems".to_owned()
        }
}
pub fn assert_a_class_file_exists_in_that(path: impl AsRef<Path>, f: impl Fn(String) -> bool) {
    let java_files = find_java_files_in_recursively(path.as_ref());
    dbg!("Java Files in {}  {}", path.as_ref(), &java_files);
    let pth_str = path.as_ref().display();
    if java_files.is_empty() {
        assert!(
            false,
            "No files were found in path {} to match against the predicate provided",
            pth_str
        );
    } else {
        assert!(
            java_files
                .iter()
                .any(|file| f(fs::read_to_string(file).unwrap())),
            "No java file found to match the predicate in path {}",
            pth_str
        );
    }
}
pub fn cleanup_folder(dir: &str) {
    if let Err(e) = fs::remove_dir_all(dir) {
        assert!(false, "Removing all files from folder failed");
    }
}
