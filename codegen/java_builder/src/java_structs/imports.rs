use super::Codegen;

#[derive(Clone)]
pub struct Import {
    //import org.codegen.package.class_name
    pub class_name: String,
    pub package_name: String,
    //import static org.codegen.package.class_name
    pub static_import: bool,
}

impl Import {
    pub fn new(package_name: String, class_name: String) -> Self {
        // Lombok does not have a dot in the name
        //assert!(package_name.contains("."),"Package name does not have dots, the params in the ::new method are the other way around");
        Self {
            class_name,
            package_name,
            static_import: false,
        }
    }

    pub fn static_(mut self) -> Self {
        self.static_import = true;
        self
    }
}
impl Codegen for Vec<Import> {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        result.push('\n');
        for import in self.iter() {
            result.push_str(&*import.generate_code());
        }
        result
    }
}

impl Codegen for Import {
    fn generate_code(&self) -> String {
        match &self.static_import {
            false => format!("import {}.{};\n", self.package_name, self.class_name),
            true => format!("import static {}.{};\n", self.package_name, self.class_name),
        }
    }
}
