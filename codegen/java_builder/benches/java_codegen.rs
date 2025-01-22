use std::time::Duration;
use std::{fs, thread};

use criterion::{criterion_group, criterion_main, Criterion};

use java_builder::classes::JavaClass;
use java_builder::fields::Field;
use java_builder::maven_builder::MavenCodebase;
use java_builder::pom_xml::{PomXml, ProjectInfo};
use java_builder::types::TypeName;
pub fn sample_project_info() -> ProjectInfo {
    ProjectInfo {
            name: "TempContRvTool".to_owned(),
            version: "".to_owned(),
            group_id: "org.javacodegen".to_owned(),
            artifact_id: "rvtool".to_owned(),
            description:"This is a project to showcase the methodology of runtime verification in the context of event based systems".to_owned()
        }
}
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

pub fn cleanup_folder(dir: &str) {
    if let Err(e) = fs::remove_dir_all(dir) {
        assert!(false, "Removing all files from folder failed");
    }
}
fn generate_100_projects(c: &mut Criterion) {
    c.bench_function("bench_maven_codegen_100_projects", |b| {
        b.iter(|| {
            for i in 1..=100 {
                let top_folder = "generated".to_owned() + &i.to_string();
                let project_info = sample_project_info();
                let mut pom_xml = PomXml::new(project_info.clone());
                let java_version = "17".to_owned();
                pom_xml = pom_xml.java_version(java_version.clone());
                pom_xml = pom_xml.spring_boot();

                let example = sample_class(&pom_xml);
                let mut mvn_code = MavenCodebase::new(pom_xml, &top_folder);
                mvn_code = mvn_code.add_entity(example.clone());
                mvn_code.generate_code();
                cleanup_folder(&top_folder);
            }
        });
    });
}

criterion_group!(benches, generate_100_projects);

criterion_main!(benches);
