pub struct PomXml {
    pub java: String,
    pub dependencies: Vec<Library>,
    pub project_info: ProjectInfo,
    //It has the same attributes thats why
    //+ a relative path ofc
    pub parent_pom: Library,
}

pub struct Library {
    group_id: String,
    pub artifact_id: String,
    version: Option<String>,
}

#[derive(Clone)]
pub struct ProjectInfo {
    pub group_id: String,
    pub artifact_id: String,
    pub name: String,
    pub description: String,
    pub version: String,
}

impl PartialEq for Library {
    fn eq(&self, other: &Self) -> bool {
        self.artifact_id.eq(&other.artifact_id)
    }
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
        self.project_info.group_id.to_owned() + "." + &self.project_info.artifact_id
    }

    pub fn has_dependency_that(&self, f: impl Fn(&Library) -> bool) -> bool {
        self.dependencies.iter().any(f)
    }

    pub fn new(project_info: ProjectInfo) -> Self {
        Self {
            project_info,
            java: "".to_owned(),
            dependencies: vec![],
            parent_pom: Library::default(),
        }
    }
    pub fn java_version(mut self, version: String) -> Self {
        self.java = version;
        self
    }

    pub fn artifact(mut self, id: String) -> Self {
        self.project_info.artifact_id = id;
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
        self = self.openapi();
        self
    }

    pub fn description(mut self, descr: String) -> Self {
        self.project_info.description = descr;
        self
    }

    pub fn project_name(mut self, name: String) -> Self {
        self.project_info.name = name;
        self
    }

    pub fn group_id(mut self, id: String) -> Self {
        self.project_info.group_id = id;
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

    pub fn openapi(mut self) -> PomXml {
        self.dependencies.push(Library::new_with_version(
            "org.springdoc".into(),
            "springdoc-openapi-starter-webmvc-ui".into(),
            "2.7.0".into(),
        ));

        self.dependencies.push(Library::new_with_version(
            "org.springdoc".into(),
            "springdoc-openapi-starter-webmvc-api".into(),
            "2.7.0".into(),
        ));
        self
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

        result +=
            &(r#"<description>"#.to_owned() + &self.project_info.description + &"</description>\n");
        result += &(r#"<name>"#.to_owned() + &self.project_info.name + &"</name>\n");
        result += &("<groupId>".to_owned() + &self.project_info.group_id + &"</groupId>\n");

        result += &("<artifactId>".to_owned() + &self.project_info.artifact_id + &"</artifactId>");
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
