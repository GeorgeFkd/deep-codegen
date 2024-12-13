use java_builder::java_builder::{Codegen, Field, JavaClass, JavaEnum, TypeName};
use openapiv3::*;
use serde_json;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;

struct CliParams {}

#[derive(Debug)]
struct FileOutput {
    relative_path: PathBuf,
    file_code: String,
}

//2 parts: OpenAPI transformations and Codegen
fn main() {
    println!("OpenAPI Entities generator");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: <program> <openapi-file>");
        exit(1);
    }

    let openapi_file = args.get(1).unwrap();
    println!("Generating code for OpenAPI File: {}", openapi_file);

    let openapi_contents = read_to_string(openapi_file);
    if let Err(file_reading_error) = openapi_contents {
        println!("Error reading file: {}", file_reading_error);
        exit(1);
    }
    let openapi_contents = openapi_contents.unwrap();
    let openapi: OpenAPI =
        serde_json::from_str(openapi_contents.as_str()).expect("OpenAPI could not be deserialized");
    let components = openapi.components.unwrap();
    let extensions = components.extensions;
    if let Some(cli_params) = extensions.get("x-cli-params") {
        println!("My cli params are: {}", cli_params);
    } else {
        println!("No cli params were provided");
    }

    dbg!(args);
}

#[test]
fn no_files_have_the_same_name() {}

#[test]
fn file_paths_constructed_correctly() {
    //test the generation of filepaths
    //result.parse().unwrap should not return an error in any case
    //can use a randomiser library
}

//i will need helper code, to generate fields from properties
//just pass in a builder and augment it and return it.

//the package openapiv3 is not maintained
//need to jump to openapiv3-extended
//which is also unmaintained
