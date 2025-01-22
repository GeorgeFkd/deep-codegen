use std::{
    fs::{self, create_dir, create_dir_all, DirEntry, File},
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};

use zip::{result::ZipError, write::SimpleFileOptions};

use crate::{classes::JavaClass, interfaces::Interface, Codegen};

pub struct OutputDirs {
    package_path: String,
    output_dir: String,
    controllers_suffix: String,
    repos_suffix: String,
    services_suffix: String,
    dtos_suffix: String,
    models_suffix: String,
    code_folder: PathBuf,
}
impl OutputDirs {
    pub fn new(output_dir: String, package_path: String) -> Self {
        let mut code_folder = PathBuf::from(".");
        code_folder.push(&output_dir);
        code_folder.push("src");
        code_folder.push("main");
        code_folder.push("java");
        for pth in package_path.split(".") {
            code_folder.push(pth);
        }

        Self {
            package_path,
            output_dir,
            code_folder,
            dtos_suffix: "".to_owned(),
            repos_suffix: "".to_owned(),
            controllers_suffix: "".to_owned(),
            services_suffix: "".to_owned(),
            models_suffix: "".to_owned(),
        }
    }

    pub fn generate_classes_in(&self, classes: &[JavaClass], folder: &str) {
        classes.iter().for_each(|cls| {
            let mut path = PathBuf::from(folder);
            path.push(&cls.class_name);
            path.set_extension("java");

            match fs::write(path.as_path(), cls.generate_code()) {
                Ok(r) => println!("Entities were successfully generated"),
                Err(e) => println!("An error occurred when generating entities {}", e),
            }
        });
    }
    pub fn generate_interfaces_in(&self, interfaces: &[Interface], folder: &str) {
        interfaces.into_iter().for_each(|cls| {
            let mut path = PathBuf::from(folder);
            path.push(&cls.name);
            path.set_extension("java");
            match fs::write(path, cls.generate_code()) {
                Ok(r) => println!("Entities were successfully generated"),
                Err(e) => println!("An error occurred when generating entities {}", e),
            }
        });
    }
    pub fn dtos(mut self, suffix: String) -> Self {
        self.dtos_suffix = suffix;
        self
    }
    pub fn repos(mut self, suffix: String) -> Self {
        self.repos_suffix = suffix;
        self
    }
    pub fn controllers(mut self, suffix: String) -> Self {
        self.controllers_suffix = suffix;
        self
    }
    pub fn services(mut self, suffix: String) -> Self {
        self.services_suffix = suffix;
        self
    }
    pub fn models(mut self, suffix: String) -> Self {
        self.models_suffix = suffix;
        self
    }

    pub fn code_folder(&self) -> &PathBuf {
        &self.code_folder
    }

    fn code_folder_str(&self) -> &str {
        self.code_folder.to_str().unwrap()
    }
    pub fn controllers_folder(&self) -> PathBuf {
        let mut controllers_folder = PathBuf::from(self.code_folder_str());
        controllers_folder.push(&self.controllers_suffix);
        controllers_folder
    }
    pub fn models_folder(&self) -> PathBuf {
        let mut controllers_folder = PathBuf::from(self.code_folder_str());
        controllers_folder.push(&self.models_suffix);
        controllers_folder
    }
    pub fn services_folder(&self) -> PathBuf {
        let mut controllers_folder = PathBuf::from(self.code_folder_str());
        controllers_folder.push(&self.services_suffix);
        controllers_folder
    }
    pub fn repos_folder(&self) -> PathBuf {
        let mut controllers_folder = PathBuf::from(self.code_folder_str());
        controllers_folder.push(&self.repos_suffix);
        controllers_folder
    }
    pub fn dtos_folder(&self) -> PathBuf {
        let mut controllers_folder = PathBuf::from(self.code_folder_str());
        controllers_folder.push(&self.dtos_suffix);
        controllers_folder
    }

    pub fn tests_folder(&self) -> PathBuf {
        let mut test_folder = PathBuf::new();
        test_folder.push(".");
        test_folder.push(&self.output_dir);
        test_folder.push("src");
        test_folder.push("test");
        test_folder.push("java");
        for pth in self.package_path.split(".") {
            test_folder.push(pth);
        }
        test_folder
    }
    pub fn resources_folder(&self) -> PathBuf {
        let mut res_folder = PathBuf::new();
        res_folder.push(".");
        res_folder.push(&self.output_dir);
        res_folder.push("src");
        res_folder.push("main");
        res_folder.push("resources");
        res_folder
    }
    pub fn extract_to_zip(&self) -> PathBuf {
        let output_path = Path::new("generated-new.zip");
        let files = find_files_in_dir_recursive(Path::new(&self.output_dir));
        let new_file = File::create(output_path).unwrap();
        let _ = zip_dir(
            &mut files.into_iter(),
            &self.output_dir,
            new_file,
            zip::CompressionMethod::Bzip2,
        );
        output_path.to_owned()
    }

    pub fn create_folders(&self) {
        //the order those folders are being created matters,
        //create_dir_all fails if any of the parents exist
        match create_dir_all(&self.code_folder()) {
            Ok(r) => println!("Created {:?} folder successfully", &self.code_folder()),
            Err(e) => assert!(
                false,
                "Failed to create {:?} folder\nError: {e},exiting",
                &self.code_folder()
            ),
        };
        create_dirs_for(vec![
            &self.repos_folder(),
            &self.models_folder(),
            &self.services_folder(),
            &self.controllers_folder(),
            &self.dtos_folder(),
            &self.resources_folder(),
        ]);
        match create_dir_all(&self.tests_folder()) {
            Ok(r) => println!("Created {:?} folder successfully", &self.tests_folder()),
            Err(e) => assert!(
                false,
                "Failed to create {:?} folder\nError: {e},exiting",
                &self.tests_folder()
            ),
        };
    }

    fn controllers_suffix(&self) -> &str {
        &self.controllers_suffix
    }

    fn repos_suffix(&self) -> &str {
        &self.repos_suffix
    }

    fn services_suffix(&self) -> &str {
        &self.services_suffix
    }

    fn dtos_suffix(&self) -> &str {
        &self.dtos_suffix
    }

    fn models_suffix(&self) -> &str {
        &self.models_suffix
    }
}

fn create_dirs_for(dirs: Vec<&PathBuf>) {
    dirs.iter().for_each(|d| {
        create_dir_of(d);
    });
}
fn find_files_in_dir_recursive(path: impl AsRef<Path>) -> Vec<DirEntry> {
    let Ok(entries) = fs::read_dir(path) else {
        return vec![];
    };
    entries
        .flatten()
        .flat_map(|entry| {
            let Ok(meta) = entry.metadata() else {
                return vec![];
            };
            if meta.is_dir() {
                return find_files_in_dir_recursive(entry.path());
            }
            if meta.is_file() {
                return vec![entry];
            }
            vec![]
        })
        .collect()
}

fn zip_dir<T, U: Iterator<Item = DirEntry>>(
    it: &mut U,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> Result<(), ZipError>
where
    T: std::io::Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    let prefix = Path::new(prefix);
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();
        let path_as_str = name.to_str().map(str::to_owned).unwrap();
        if path.is_file() {
            println!("adding file {:?} as {:?}", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            println!("adding dir {path_as_str:?} as {name:?} ...");
            zip.add_directory(path_as_str, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

fn create_dir_of(folder: &PathBuf) {
    match create_dir(folder) {
        Ok(r) => println!("Created {:?} folder successfully", folder),
        Err(e) => assert!(
            false,
            "Failed to create {:?} folder\nError: {e},exiting",
            folder
        ),
    }
}
