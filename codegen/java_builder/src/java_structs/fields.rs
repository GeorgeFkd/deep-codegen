use super::{
    annotations::Annotation,
    modifiers::{self, AccessModifiers},
    Codegen,
};
impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

#[derive(Hash, Eq, Clone)]
pub struct Field {
    //might be empty but we dont care
    pub annotation: Vec<super::annotations::Annotation>,
    //i want to make this a hashset to avoid duplicates but i dont think someone would
    //accidentally input duplicate stuff
    pub modifiers: Vec<super::modifiers::AccessModifiers>,
    pub name: String,
    pub type_: super::types::TypeName,
    //this type can be stricter
    pub initializer: Option<String>,
}
//TODO make the default modifier be Private
impl Field {
    pub fn new(
        name: String,
        type_: super::types::TypeName,
        modifier: super::modifiers::AccessModifiers,
    ) -> Self {
        Self {
            name,
            type_,
            modifiers: vec![modifier],
            annotation: vec![],
            initializer: None,
        }
    }

    pub fn n(name: String, type_: super::types::TypeName) -> Self {
        Self {
            name,
            type_,
            modifiers: vec![AccessModifiers::Private],
            initializer: None,
            annotation: vec![],
        }
    }

    pub fn annotation(mut self, a: Annotation) -> Self {
        self.annotation.push(a);
        self
    }
}

impl super::Codegen for Field {
    fn generate_code(&self) -> String {
        let mut result = "".to_string();
        //todo run a java formatter after generation
        //i do some basic formatting so it is not unreadable
        result.push_str("    ");
        for annotation in self.annotation.iter() {
            result.push_str(annotation.generate_code().as_str());
        }
        result.push_str("    ");
        let mut sorted_modifiers = self.modifiers.to_owned();
        sorted_modifiers.sort_by(|a, b| b.cmp(a));
        for m in sorted_modifiers {
            result.push_str(&format!(
                "{} ",
                <super::modifiers::AccessModifiers as Into<String>>::into(m)
            ));
        }
        result.push_str(&format!("{} ", self.type_.generate_code()));
        result.push_str(&format!("{};\n", self.name));

        if let Some(ref init) = self.initializer {
            result.push_str(&format!("= {}", init));
        }
        result
    }
}
