pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
//inspiration from: https://github.com/palantir/javapoet/

#[cfg(test)]
mod java_builder {
    use std::cmp::Ordering;
    use std::collections::HashSet;
    use std::fs;
    use std::hash::Hash;
    use std::path::PathBuf;
    use std::process::{Command, ExitStatus};
    use crate::java_builder::class_builder::Builder;

    #[derive(Debug)]
    pub struct ErrMessage {
        msg: String,
        fix: String,
    }

    //https://github.com/palantir/javapoet/blob/develop/javapoet/src/main/java/com/palantir/javapoet/FieldSpec.java
    /*
    /*
* Copyright (C) 2015 Square, Inc.
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/
package com.palantir.javapoet;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.lang.reflect.Type;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Set;
import javax.lang.model.SourceVersion;
import javax.lang.model.element.Modifier;

/** A generated field declaration. */
public final class FieldSpec {
private final TypeName type;
private final String name;
private final List<AnnotationSpec> annotations;
private final Set<Modifier> modifiers;
private final CodeBlock initializer;

private FieldSpec(Builder builder) {
    this.type = checkNotNull(builder.type, "type == null");
    this.name = checkNotNull(builder.name, "name == null");
    this.javadoc = builder.javadoc.build();
    this.annotations = Util.immutableList(builder.annotations);
    this.modifiers = Util.immutableSet(builder.modifiers);
    this.initializer = (builder.initializer == null) ? CodeBlock.builder().build() : builder.initializer;
}
}
*/

    fn get_default_class_builder_for_test() -> class_builder::Builder {
        let class_name = "FieldSpec";
        let package_name = "com.palantir.javapoet";
        let field1 = "type";
        let result = class_builder::Builder::new()
            .package(package_name.to_owned())
            .import(Import { class_name: "IOException".to_string(), package_name: "java.io".to_string() })
            .import(Import { class_name: "UncheckedIOException".to_string(), package_name: "java.io".to_string() })
            .import(Import { class_name: "List".to_string(), package_name: "java.util".to_string() })
            .import(Import { class_name: "SourceVersion".to_string(), package_name: "javax.lang.model".to_string() })
            .import(Import { class_name: "TemplateEngine".to_string(), package_name: "org.openapi.tools".to_string() })
            .class_name(class_name.to_string())
            .field(Field {
                annotation: vec![Annotation { qualified_name: "Autowired".to_string(), params_list: None }],
                modifiers: vec![AccessModifiers::Final, AccessModifiers::Private],
                name: "type".to_string(),
                type_: "TypeName".to_string(),
                initializer: None,
            })
            .field(Field {
                name: "name".to_string(),
                type_: "String".to_string(),
                modifiers: vec![AccessModifiers::Private, AccessModifiers::Final],
                initializer: None,
                annotation: vec![
                    Annotation {
                        qualified_name: "XmlRootElement".to_string(),
                        params_list: Some(vec![
                            ("name".to_string(), "phone-number".to_string())])
                    }],
            });
        result
    }

    #[test]
    fn can_generate_class_to_file() {

    }


    #[test]
    fn can_generate_class() {


        let class_name = "FieldSpec";
        let package_name = "com.palantir.javapoet";
        let field1 = "type";

        //todo generate an ANTLR4 java parser and use that in order to ensure that syntax is correct
        //i can generate the java one and just write a simple program that uses it and prints errors
        //to stdout.
        //we care about correct syntax in this library and offering a proper api
        //as usages grow things will be added
        let result = get_default_class_builder_for_test().generate_class();

        // println!("@XmlRootElement(name={}{}{})",'"',"phone-number",'"');
        assert!(result.len() > 0);
        println!("Result is: \n{result}");
        assert!(result.contains(class_name));
        assert!(result.contains(field1));
        //private,public,protected > abstract > final > static
        //i could do the asserts in a more property-based testing manner
        //but right now i wont
        assert!(!result.contains("final private"));
        assert!(!result.contains("static public"));
    }

    pub struct Import {
        //import org.codegen.package.class_name
        class_name: String,
        package_name: String,
    }


    pub struct Annotation {
        qualified_name: String,
        //name = value
        params_list: Option<Vec<(String, String)>>,
    }

    impl PartialEq<Self> for Annotation {
        fn eq(&self, other: &Self) -> bool {
            self.qualified_name.eq(&other.qualified_name)
        }
    }

    impl Eq for Annotation {}


    impl Hash for Annotation {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.qualified_name.hash(state)
        }
    }


    #[derive(Copy, Clone, Eq)]
    pub enum AccessModifiers {
        Public,
        Private,
        Protected,
        Static,
        Abstract,
        Final,
        //Will not use those
        //Native
        //Synchronised
        //Transient
        //Volatile
        //strictfp
    }
    impl Hash for AccessModifiers {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            <AccessModifiers as Into<String>>::into(*self).hash(state)
        }
    }


    impl PartialEq<AccessModifiers> for AccessModifiers {
        fn eq(&self, other: &Self) -> bool {
            <AccessModifiers as Into<String>>::into(*self) ==
                <AccessModifiers as Into<String>>::into(*other)
        }
    }

    impl PartialOrd<AccessModifiers> for AccessModifiers {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if <AccessModifiers as Into<String>>::into(*self) ==
                <AccessModifiers as Into<String>>::into(*other) { return Some(Ordering::Equal) }
            //private,public,protected > abstract > final > static
            if self.eq(&AccessModifiers::Private) || self.eq(&AccessModifiers::Public) || self.eq(&AccessModifiers::Protected) {
                return Some(Ordering::Greater)
            }
            if other.eq(&AccessModifiers::Protected) || other.eq(&AccessModifiers::Public) || other.eq(&AccessModifiers::Private) {
                return Some(Ordering::Less)
            }

            if self.eq(&AccessModifiers::Abstract){
                return Some(Ordering::Greater)
            }
            if other.eq(&AccessModifiers::Abstract){
                return Some(Ordering::Less)
            }

            if self.eq(&AccessModifiers::Final){
                return Some(Ordering::Greater)
            }
            if other.eq(&AccessModifiers::Final){
                return Some(Ordering::Less)
            }

            Some(Ordering::Equal)
        }
    }

    impl Ord for AccessModifiers {
        fn cmp(&self, other: &Self) -> Ordering {
            if <AccessModifiers as Into<String>>::into(*self) ==
                <AccessModifiers as Into<String>>::into(*other) { return Ordering::Equal }
            //private,public,protected > abstract > fina
            if self.eq(&AccessModifiers::Private) || self.eq(&AccessModifiers::Public) || self.eq(&AccessModifiers::Protected) {
                return Ordering::Greater
            }
            if other.eq(&AccessModifiers::Protected) || other.eq(&AccessModifiers::Public) || other.eq(&AccessModifiers::Private) {
                return Ordering::Less
            }

            if self.eq(&AccessModifiers::Abstract){
                return Ordering::Greater
            }
            if other.eq(&AccessModifiers::Abstract){
                return Ordering::Less
            }

            if self.eq(&AccessModifiers::Final){
                return Ordering::Greater
            }
            if other.eq(&AccessModifiers::Final){
                return Ordering::Less
            }

            Ordering::Equal


        }
    }


    impl Into<&str> for AccessModifiers {
        fn into(self) -> &'static str {
            match self {
                AccessModifiers::Public => "public",
                AccessModifiers::Private => "private",
                AccessModifiers::Protected => "protected",
                AccessModifiers::Static => "static",
                AccessModifiers::Abstract => "abstract",
                AccessModifiers::Final => "final",
            }
        }
    }

    impl Codegen for Vec<AccessModifiers> {
        fn generate_code(&self) -> String {
            "".to_string()
        }
    }

    impl Into<String> for AccessModifiers {
        fn into(self) -> String {
            match self {
                AccessModifiers::Public => "public".to_owned(),
                AccessModifiers::Private => "private".to_owned(),
                AccessModifiers::Protected => "protected".to_owned(),
                AccessModifiers::Static => "static".to_owned(),
                AccessModifiers::Abstract => "abstract".to_owned(),
                AccessModifiers::Final => "final".to_owned(),
            }
        }
    }

    impl Codegen for Vec<Import> {
        fn generate_code(&self) -> String {
            let mut result = "".to_string();
            result.push('\n');
            for import in self.iter() {
                result.push_str(&*import.generate_code());
                result.push('\n');
            }
            result
        }
    }
    
    impl Codegen for Import {
        fn generate_code(&self) -> String {
            format!("import {}.{}",self.package_name,self.class_name)
        }
    }

    trait Codegen {
        fn generate_code(&self) -> String;
    }



    impl PartialEq for Field {
        fn eq(&self, other: &Self) -> bool {
            self.name.eq(&other.name)
        }
    }

    #[derive(Hash, Eq)]
    pub struct Field {
        //might be empty but we dont care
        annotation: Vec<Annotation>,
        //i want to make this a hashset to avoid duplicates but i dont think someone would
        //accidentally input duplicate stuff
        modifiers: Vec<AccessModifiers>,
        name: String,
        type_: String,
        //this type can be stricter
        initializer: Option<String>,
    }


    pub type Implements = String;
    type Extends = Option<Vec<String>>;

    mod interface_builder {}
    mod enum_builder {}

    fn format_java_file(path:PathBuf) -> Result<PathBuf,String> {
        //i can get this stricter
        let file_exists = fs::metadata(&path).unwrap().is_file();
        if !file_exists {
            return Err("File provided does not exist".to_string());
        }
        let formatter_exists = fs::metadata("./google-java-format-1.25.0-all-deps.jar").unwrap().is_file();
        if !formatter_exists {
            return Err("./google-java-format-1.25.0-all-deps.jar was not found, \
            check where the executable is run from \
            or download it from: https://github.com/google/google-java-format/releases".to_string());
        }
        let result = Command::new("java")
            .arg("-jar")
            .arg("google-java-format-1.25.0-all-deps.jar")
            .arg("--replace")
            .output()
            .expect("google-java-format-1.25.0-all-deps.jar was not found or something else went wrong");
        if !result.status.success() {
            return Err("Failed to format file".to_string());
        }
        Ok(path)
    }

    pub mod class_builder {
        use super::*;

        // #[derive(Copy,Clone)]
        //i should read this: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
        pub struct Builder {
            pub imports: Option<Vec<Import>>,
            pub implements: Option<Vec<Implements>>,
            pub class_annotations: Option<Vec<Annotation>>,
            pub fields: HashSet<Field>,
            pub class_name: String,
            pub class_modifiers: Option<HashSet<AccessModifiers>>,
            pub superclass: Option<String>,
            pub package: String,
        }

        //todo, find a way to share the logic of
        //producing code from imports, modifiers etc. etc.
        //a common trait called Codegen
        //and instead of doing weird result.push_str(asdasd)
        //we do result.push_str(fields.generate());
        //and each type of file can just do an impl Generate for Class
        //and then use the generate/emitCode common method from the other parts

        impl Builder {
            pub fn new() -> Builder {
                Builder {
                    imports: None,
                    class_name: "".to_owned(),
                    superclass: None,
                    class_annotations: None,
                    class_modifiers: None,
                    implements: None,
                    fields: HashSet::new(),
                    package: "".to_owned(),
                }
            }
            pub fn class_modifiers(mut self, modifiers: HashSet<AccessModifiers>) ->  Builder {
                match self.class_modifiers {
                    Some(ref mut builder_modifiers) => {
                        modifiers
                            .into_iter()
                            .for_each(|m| {
                                builder_modifiers.insert(m);
                            });
                        self
                    }
                    None => {
                        self.class_modifiers = Some(modifiers);
                        self
                    }
                }
            }

            pub fn package(mut self, package: String) -> Builder {
                self.package = package;
                self
            }

            pub fn class_name(mut self, name: String) ->  Builder {
                self.class_name = name;
                self
            }

            pub fn extends(mut self, extends: String) -> Builder {
                self.superclass = Some(extends);
                self
            }

            pub fn import(mut self, imp: Import) -> Builder {
                match self.imports {
                    Some(ref mut imports) => {
                        imports.push(imp);
                        self
                    }
                    None => {
                        self.imports = Some(vec![imp]);
                        self
                    }
                }
            }
            pub fn field(mut self, f: Field) -> Builder {
                self.fields.insert(f);
                self
            }

            pub fn annotation(mut self, a: Annotation) -> Builder {
                match self.class_annotations {
                    Some(ref mut annotations) => {
                        annotations.push(a);
                        self
                    }
                    None => {
                        self.class_annotations = Some(vec![a]);
                        self
                    }
                }
            }
            pub fn implements(mut self, interface: Implements) -> Builder {
                match self.implements {
                    Some(ref mut implements) => {
                        implements.push(interface);
                        self
                    }
                    None => {
                        self.implements = Some(vec![interface]);
                        self
                    }
                }
            }

            pub fn generate_class_to_file(&self, path_buf: PathBuf) -> Result<String,String> {
                let result = self.generate_class();
                fs::write(path_buf, self.generate_class()).expect("TODO: panic message");
                Ok(result)
            }

            pub fn generate_class(&self) -> String {
                //i could refactor to more immutability in this method
                let mut result: String = "".to_string();
                result.push_str(&format!("package {};\n", self.package));
                result.push_str("\n");


                if let Some(ref imports) = self.imports {
                    imports.iter().for_each(|import| {
                        result.push_str(&format!("import {}.{};\n", import.package_name, import.class_name));
                    })
                }
                result.push_str("\n");

                if let Some(ref modifiers) = self.class_modifiers {
                    for elem in modifiers.iter() {
                        //my rust is rusty
                        result.push_str(&format!("{} ", <AccessModifiers as Into<String>>::into(*elem)));
                    }
                    result.push(' ');
                }

                result.push_str(&format!("class {} {{ \n", self.class_name));

                if let Some(ref implements) = self.implements {
                    result.push_str("implements ");
                    for (pos, elem) in implements.iter().enumerate() {
                        result.push_str(elem);
                        if pos != implements.len() - 1 {
                            result.push_str(", ");
                        }
                    }
                }
                if let Some(ref superclass) = self.superclass {
                    result.push_str(&format!(" extends {} {{  \n", superclass))
                }

                if !self.fields.is_empty() {
                    for field in self.fields.iter() {
                        //todo run a java formatter after generation
                        //i do some basic formatting so it is not unreadable
                        result.push_str("    ");
                        for annotation in field.annotation.iter() {
                            //if something is of string type i need to wrap it in quotes so it works properly""
                            result.push_str(&format!("@{} ", annotation.qualified_name));
                            if let Some(ref params_list) = annotation.params_list {
                                result.push('(');
                                result.push('\n');
                                for param in params_list {
                                    result.push_str(&format!("{} = {}\n", param.0, param.1))
                                }
                                result.push('\n');
                                result.push(')');
                            }
                            result.push('\n');
                        }
                        result.push_str("    ");
                        //todo sort modifiers so i dont have funny errors
                        let mut sorted_modifiers = field.modifiers.to_owned();

                        sorted_modifiers.sort_by(|a,b| b.cmp(a));
                        //for some reason it does not work
                        for m in sorted_modifiers {
                            result.push_str(&format!("{} ", <AccessModifiers as Into<String>>::into(m)));
                        }
                        result.push_str(&format!("{} ", field.type_));
                        result.push_str(&format!("{};\n", field.name));

                        if let Some(ref init) = field.initializer {
                            result.push_str(&format!("= {}", init));
                        }
                    }
                }

                result.push_str("\n}\n");
                result
            }

            pub fn build(self) -> Self {
                self
            }
        }
    }
    mod record_builder {}
    // not needed rn will be implemented later
    mod annotation_builder {}
}
