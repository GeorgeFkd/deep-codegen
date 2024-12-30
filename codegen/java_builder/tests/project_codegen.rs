mod common;
#[cfg(test)]
mod java_project_tests {
    use java_builder::{
        classes::JavaClass,
        fields::Field,
        maven_builder::MavenCodebase,
        pom_xml::{Generate, PomXml},
        types::TypeName,
    };

    use common::{
        assert_a_class_file_exists_in_that, assert_dir_exists, assert_xml_structure_with_xsd,
        cleanup_folder, mvn_project_compiles, xml_lint_is_installed,
    };

    fn sample_class(pom_xml: &PomXml) -> JavaClass {
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

    #[test]
    fn can_generate_crud_classes() {
        let top_folder = "generated";
        let mut pom_xml = PomXml::new();
        let descr = "This is a project to showcase the methodology of runtime verification in the context of event based systems".to_owned();
        let project = "TempContRvTool".to_owned();
        let java_version = "17".to_owned();
        let group_id = "org.javacodegen".to_owned();
        let artifact_id = "rvtool".to_owned();
        pom_xml = pom_xml.description(descr.clone());
        pom_xml = pom_xml.project_name(project.clone());
        pom_xml = pom_xml.java_version(java_version.clone());
        pom_xml = pom_xml.group_id(group_id.clone());
        pom_xml = pom_xml.artifact(artifact_id.clone());
        pom_xml = pom_xml.spring_boot();

        let example = sample_class(&pom_xml);
        let mut mvn_code = MavenCodebase::new(pom_xml, top_folder);
        mvn_code = mvn_code.add_entity(example.clone());
        mvn_code.generate_code();
        //there is a file that contains the name i provided
        assert_a_class_file_exists_in_that(top_folder, |content| {
            content.contains(&example.class_name)
        });

        assert_a_class_file_exists_in_that(top_folder, |content| {
            content.contains("extends JpaRepository")
        });

        assert_a_class_file_exists_in_that(top_folder, |content| content.contains("@Service"));
        assert_a_class_file_exists_in_that(top_folder, |content| content.contains("@Entity"));
        assert_a_class_file_exists_in_that(top_folder, |content| {
            content.to_lowercase().contains("dto") && content.contains(&example.class_name)
        });

        // assert_a_class_file_exists_in_that(top_folder, |content| {
        //     content.contains("@RestController")
        //         && content.contains("@RequestMapping")
        //         && content.contains("@PostMapping")
        //         && content.contains("@GetMapping")
        //         && content.contains("@PutMapping")
        //         && content.contains("@DeleteMapping")
        // });
        mvn_project_compiles(top_folder);
        // cleanup_folder(top_folder);
    }

    use std::path::Path;

    use crate::common;
    #[test]
    fn can_create_maven_folders() {
        let top_folder = "generated2";
        let mut pom_xml = PomXml::new();
        let descr = "This is a project to showcase the methodology of runtime verification in the context of event based systems".to_owned();
        let project = "TempContRvTool".to_owned();
        let java_version = "17".to_owned();
        let group_id = "org.javacodegen".to_owned();
        let artifact_id = "rvtool".to_owned();
        pom_xml = pom_xml.description(descr.clone());
        pom_xml = pom_xml.project_name(project.clone());
        pom_xml = pom_xml.java_version(java_version.clone());
        pom_xml = pom_xml.group_id(group_id.clone());
        pom_xml = pom_xml.artifact(artifact_id.clone());
        pom_xml = pom_xml.spring_boot();
        let mut mvn_codebase = MavenCodebase::new(pom_xml, &top_folder);
        mvn_codebase.write_initial_files();
        let code_folder = &(top_folder.to_owned() + &"/src/main/java/org/javacodegen/rvtool");
        assert_dir_exists(code_folder);

        let resources_folder = &(top_folder.to_owned() + &"/src/main/resources");
        assert_dir_exists(resources_folder);

        let test_folder = &(top_folder.to_owned() + &"/src/test/java/org/javacodegen/rvtool");
        assert_dir_exists(test_folder);

        let mainfile = code_folder.to_owned() + "/" + &project + ".java";

        assert!(
            Path::new(&mainfile).exists(),
            "Main file {} was not created correctly",
            mainfile
        );
        mvn_project_compiles(top_folder);
        //cleanup_folder(top_folder);
    }

    fn create_pom_xml() -> PomXml {
        let mut pom_xml = PomXml::new();
        let descr = "This is a project to showcase the methodology of runtime verification in the context of event based systems".to_owned();
        let project = "TempContRvTool".to_owned();
        let java_version = "17".to_owned();
        let group_id = "org.javacodegen".to_owned();
        let artifact_id = "rvtool".to_owned();
        pom_xml = pom_xml.description(descr.clone());
        pom_xml = pom_xml.project_name(project.clone());
        pom_xml = pom_xml.java_version(java_version.clone());
        pom_xml = pom_xml.group_id(group_id.clone());
        pom_xml = pom_xml.artifact(artifact_id.clone());
        let sb_conf_library = (
            "org.springframework.boot",
            "spring-boot-configuration-processor",
        );
        pom_xml = pom_xml.add_library(
            sb_conf_library.0.clone().into(),
            sb_conf_library.1.clone().into(),
        );

        pom_xml = pom_xml.spring_boot();
        pom_xml = pom_xml.postgresql();
        pom_xml = pom_xml.lombok();
        pom_xml = pom_xml.spring_boot_devtools();
        pom_xml
    }
    #[test]
    fn can_create_pom_xml() {
        let mut pom_xml = PomXml::new();
        let descr = "This is a project to showcase the methodology of runtime verification in the context of event based systems".to_owned();
        let project = "TempContRvTool".to_owned();
        let java_version = "17".to_owned();
        let group_id = "org.javacodegen".to_owned();
        let artifact_id = "rvtool".to_owned();
        pom_xml = pom_xml.description(descr.clone());
        pom_xml = pom_xml.project_name(project.clone());
        pom_xml = pom_xml.java_version(java_version.clone());
        pom_xml = pom_xml.group_id(group_id.clone());
        pom_xml = pom_xml.artifact(artifact_id.clone());
        let sb_conf_library = (
            "org.springframework.boot",
            "spring-boot-configuration-processor",
        );
        pom_xml = pom_xml.add_library(
            sb_conf_library.0.clone().into(),
            sb_conf_library.1.clone().into(),
        );

        pom_xml = pom_xml.spring_boot();
        pom_xml = pom_xml.postgresql();
        pom_xml = pom_xml.lombok();
        pom_xml = pom_xml.spring_boot_devtools();

        let result = pom_xml.generate();
        assert!(
            !result.is_empty(),
            "the result of pom.xml generation was an empty string"
        );
        assert_xml_structure_with_xsd(&result);
        assert!(
            result.contains(&descr),
            "Description is not properly included"
        );

        assert!(
            result.contains(&project),
            "Project name is not properly included in pom.xml"
        );
        assert!(
            result.contains(&("<java.version>".to_owned() + &java_version + &"</java.version>")),
            "Java version is not properly included in pom.xml"
        );

        assert!(
            result.contains(&group_id),
            "Group id is not properly included in pom.xml"
        );

        assert!(
            result.contains(&artifact_id),
            "Artifact id is not properly included in pom.xml"
        );

        assert!(
            result.contains("<dependencies>"),
            "Dependencies are not properly included in pom.xml"
        );
        assert!(result.contains("org.springframework.boot"));
        assert!(result.contains("org.postgresql"));
        assert!(result.contains("org.projectlombok"));
        assert!(result.contains("spring-boot-devtools"));
    }
}
