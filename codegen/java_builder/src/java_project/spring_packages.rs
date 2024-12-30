pub mod spring_packages {
    use crate::java_project::pom_xml::PomXml;
    impl PomXml {
        pub fn spring_boot_starter_actuator(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-actuator".into(),
            );
            self
        }

        pub fn spring_boot_starter_batch(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-batch".into(),
            );
            self
        }

        pub fn spring_boot_starter_data_jdbc(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-data-jdbc".into(),
            );
            self
        }

        pub fn spring_boot_starter_data_jpa(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-data-jpa".into(),
            );
            self
        }

        pub fn spring_boot_starter_data_ldap(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-data-ldap".into(),
            );
            self
        }

        pub fn spring_boot_starter_data_rest(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-data-rest".into(),
            );
            self
        }

        pub fn spring_boot_starter_mail(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-mail".into(),
            );
            self
        }

        pub fn spring_boot_starter_oauth2_authorization_server(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-oauth2-authorization-server".into(),
            );
            self
        }

        pub fn spring_boot_starter_oauth2_client(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-oauth2-client".into(),
            );
            self
        }

        pub fn spring_boot_starter_thymeleaf(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-thymeleaf".into(),
            );
            self
        }

        pub fn spring_boot_starter_web(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-web".into(),
            );
            self
        }

        pub fn spring_kafka(mut self) -> Self {
            self = self.add_library("org.springframework.kafka".into(), "spring-kafka".into());
            self
        }

        pub fn thymeleaf_extras_springsecurity6(mut self) -> Self {
            self = self.add_library(
                "org.thymeleaf.extras".into(),
                "thymeleaf-extras-springsecurity6".into(),
            );
            self
        }

        pub fn spring_boot_devtools(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-devtools".into(),
            );
            self
        }

        pub fn spring_boot_docker_compose(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-docker-compose".into(),
            );
            self
        }

        pub fn postgresql(mut self) -> Self {
            self = self.add_library("org.postgresql".into(), "postgresql".into());
            self
        }

        pub fn lombok(mut self) -> Self {
            self = self.add_library("org.projectlombok".into(), "lombok".into());
            self
        }

        pub fn spring_boot_starter_test(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-starter-test".into(),
            );
            self
        }

        pub fn spring_boot_testcontainers(mut self) -> Self {
            self = self.add_library(
                "org.springframework.boot".into(),
                "spring-boot-testcontainers".into(),
            );
            self
        }

        pub fn spring_batch_test(mut self) -> Self {
            self = self.add_library(
                "org.springframework.batch".into(),
                "spring-batch-test".into(),
            );
            self
        }

        pub fn spring_kafka_test(mut self) -> Self {
            self = self.add_library(
                "org.springframework.kafka".into(),
                "spring-kafka-test".into(),
            );
            self
        }

        pub fn spring_restdocs_mockmvc(mut self) -> Self {
            self = self.add_library(
                "org.springframework.restdocs".into(),
                "spring-restdocs-mockmvc".into(),
            );
            self
        }

        pub fn spring_security_test(mut self) -> Self {
            self = self.add_library(
                "org.springframework.security".into(),
                "spring-security-test".into(),
            );
            self
        }

        pub fn junit_jupiter(mut self) -> Self {
            self = self.add_library("org.testcontainers".into(), "junit-jupiter".into());
            self
        }

        pub fn kafka(mut self) -> Self {
            self = self.add_library("org.testcontainers".into(), "kafka".into());
            self
        }

        pub fn testcontainers_postgresql(mut self) -> Self {
            self = self.add_library("org.testcontainers".into(), "postgresql".into());
            self
        }
    }
}
