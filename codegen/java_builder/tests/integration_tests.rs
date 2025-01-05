mod common;

#[cfg(test)]
mod integration_tests {

    use crate::common::{assert_spring_server_is_up, sample_class, sample_project_info};
    use java_builder::{maven_builder::MavenCodebase, pom_xml::PomXml};
    use std::fs::OpenOptions;
    #[test]
    fn can_configure_spring_boot_successfully() {
        let top_folder = "generated3";
        let project_info = sample_project_info();
        let mut pom_xml = PomXml::new(project_info.clone());
        let java_version = "17".to_owned();
        pom_xml = pom_xml.java_version(java_version.clone());
        pom_xml = pom_xml.spring_boot().postgresql();

        let example = sample_class(&pom_xml);
        let mut mvn_code = MavenCodebase::new(pom_xml, top_folder);
        mvn_code = mvn_code.add_entity(example.clone());
        let addr = mvn_code.get_server_addr();
        let db = mvn_code.get_db_info().to_owned();
        mvn_code.generate_code();
        //FYI this spawns and kills a postgres container
        assert_spring_server_is_up(addr, &db, top_folder);
    }

    #[test]
    fn can_output_codebase_to_zip() {
        let top_folder = "generated-w-zip";
        let project_info = sample_project_info();
        let mut pom_xml = PomXml::new(project_info.clone());
        let java_version = "17".to_owned();
        pom_xml = pom_xml.java_version(java_version.clone());
        pom_xml = pom_xml.spring_boot().postgresql();

        let example = sample_class(&pom_xml);
        let mut mvn_code = MavenCodebase::new(pom_xml, top_folder);
        mvn_code = mvn_code.add_entity(example.clone());
        mvn_code.generate_code();

        let path = mvn_code.extract_to_zip();
        assert!(
            path.to_str().unwrap().ends_with("zip"),
            "Seems like the zip dir was not properly generated"
        );

        let existing_zip = OpenOptions::new().read(true).open(path).unwrap();
        let mut zip_dir = zip::read::ZipArchive::new(existing_zip)
            .expect("Something went wrong with opening the zip file");
        for i in 0..zip_dir.len() {
            let mut file = zip_dir.by_index(i).unwrap();
            if file.is_file() {
                //extra asserts here
            }
            println!("Filename: {}", file.name());
            std::io::copy(&mut file, &mut std::io::stdout()).unwrap();
        }
    }
}
