//check what build systems are available
//implement a java project builder (for now only maven with spring boot codegen)
//select:
// - libraries
// - general options that have sensible defaults (model folder,docs folder, services folder etc. etc.)
// - extras: CRUDs + Search
mod spring_packages;

pub mod crud_builder;
pub mod maven_builder;
pub mod pom_xml;

// pub mod docs_builder {}
