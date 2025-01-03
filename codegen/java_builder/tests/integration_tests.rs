mod common;

#[cfg(test)]
mod integration_tests {

    use java_builder::{maven_builder::MavenCodebase, pom_xml::PomXml};

    use crate::common::{assert_spring_server_is_up, sample_class, sample_project_info};
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
}
