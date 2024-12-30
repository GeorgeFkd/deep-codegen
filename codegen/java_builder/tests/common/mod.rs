use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use tree_sitter::Parser;

pub fn maven_is_installed() -> bool {
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
pub fn mvn_project_compiles(proj_dir: &str) -> bool {
    assert!(
        maven_is_installed(),
        "mvn command is not present, install the maven package from your package manager"
    );
    let cwd = env::current_dir().expect("for some reason couldnt get the current dir");
    println!("The current dir is {:?}", cwd);
    let pom_path = cwd.join(proj_dir).join("pom.xml");
    println!("The path is: {:?}", pom_path);
    assert!(
        pom_path.exists(),
        "pom.xml maven file is not present in the current working directory"
    );
    let mut mvn = Command::new("mvn");
    mvn.arg("-f").arg(pom_path).arg("clean").arg("compile");
    if let Ok(res) = mvn.status() {
        return res.success();
    } else {
        false
    }
}
pub fn xml_lint_is_installed() -> bool {
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
