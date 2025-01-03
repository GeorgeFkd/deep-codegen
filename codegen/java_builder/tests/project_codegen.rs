mod common;
#[cfg(test)]
mod java_project_tests {
    use java_builder::{
        maven_builder::MavenCodebase,
        pom_xml::{Generate, PomXml},
    };

    use common::{
        assert_a_class_file_exists_in_that, assert_dir_exists, assert_xml_structure_with_xsd,
        cleanup_folder, mvn_project_compiles,
    };

    #[test]
    fn can_config_application_properties() {
        let top_folder = "generated4";
        let project_info = sample_project_info();
        let mut pom_xml = PomXml::new(project_info.clone());
        let java_version = "17".to_owned();
        pom_xml = pom_xml.java_version(java_version);
        pom_xml = pom_xml.spring_boot();
        let pom = pom_xml.generate();
        assert!(pom.contains("openapi"), "Pom.xml does not contain openapi");
        let mut mvn_code = MavenCodebase::new(pom_xml, top_folder);
        let app = mvn_code.create_application_properties();
        println!("Application properties:\n {app}");
        let openapi_requirements =
            app.contains("springdoc.swagger-ui.path") && app.contains("springdoc.api-docs.path");
        assert!(
            openapi_requirements,
            "application.properties is not properly configured for openapi"
        );
    }

    #[test]
    fn can_generate_crud_classes() {
        let top_folder = "generated";
        let project_info = sample_project_info();
        let mut pom_xml = PomXml::new(project_info.clone());
        let java_version = "17".to_owned();
        pom_xml = pom_xml.java_version(java_version.clone());
        pom_xml = pom_xml.spring_boot();

        let example = sample_class(&pom_xml);
        let mut mvn_code = MavenCodebase::new(pom_xml, top_folder);
        mvn_code = mvn_code.add_entity(example.clone());
        mvn_code.generate_code();
        //there is a file that contains the name of the class  i provided
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

        assert_a_class_file_exists_in_that(top_folder, |content| {
            content.contains("@RestController")
                && content.contains("@RequestMapping")
                && content.contains("@PostMapping")
                && content.contains("@GetMapping")
                && content.contains("@PutMapping")
                && content.contains("@DeleteMapping")
        });
        mvn_project_compiles(top_folder);
        // cleanup_folder(top_folder);
    }

    use std::path::Path;

    use crate::common::{self, sample_class, sample_project_info};
    #[test]
    fn can_create_maven_folders() {
        let top_folder = "generated2";
        let project_info = sample_project_info();
        let mut pom_xml = PomXml::new(project_info.clone());
        let java_version = "17".to_owned();
        pom_xml = pom_xml.java_version(java_version.clone());
        pom_xml = pom_xml.spring_boot();
        let mut mvn_codebase = MavenCodebase::new(pom_xml, &top_folder);
        mvn_codebase.write_initial_files();
        let code_folder = &(top_folder.to_owned() + &"/src/main/java/org/javacodegen/rvtool");
        assert_dir_exists(code_folder);

        let resources_folder = &(top_folder.to_owned() + &"/src/main/resources");
        assert_dir_exists(resources_folder);

        let test_folder = &(top_folder.to_owned() + &"/src/test/java/org/javacodegen/rvtool");
        assert_dir_exists(test_folder);

        let mainfile = code_folder.to_owned() + "/" + &project_info.name + ".java";

        assert!(
            Path::new(&mainfile).exists(),
            "Main file {} was not created correctly",
            mainfile
        );
        mvn_project_compiles(top_folder);
        //cleanup_folder(top_folder);
    }

    fn create_pom_xml() -> PomXml {
        let project_info = sample_project_info();
        let mut pom_xml = PomXml::new(project_info.clone());
        let java_version = "17".to_owned();
        pom_xml = pom_xml.java_version(java_version.clone());
        let sb_conf_library = (
            "org.springframework.boot",
            "spring-boot-configuration-processor",
        );
        pom_xml = pom_xml.add_library(sb_conf_library.0.into(), sb_conf_library.1.into());

        pom_xml = pom_xml.spring_boot();
        pom_xml = pom_xml.postgresql();
        pom_xml = pom_xml.lombok();
        pom_xml = pom_xml.spring_boot_devtools();
        pom_xml
    }
    #[test]
    fn can_create_pom_xml() {
        let project_info = sample_project_info();
        let mut pom_xml = PomXml::new(project_info.clone());
        let java_version = "17".to_owned();
        pom_xml = pom_xml.java_version(java_version.clone());
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
            result.contains(&project_info.description),
            "Description is not properly included"
        );

        assert!(
            result.contains(&project_info.name),
            "Project name is not properly included in pom.xml"
        );
        assert!(
            result.contains(&("<java.version>".to_owned() + &java_version + &"</java.version>")),
            "Java version is not properly included in pom.xml"
        );

        assert!(
            result.contains(&project_info.group_id),
            "Group id is not properly included in pom.xml"
        );

        assert!(
            result.contains(&project_info.artifact_id),
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
