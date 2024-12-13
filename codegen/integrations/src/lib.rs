mod integrations {
    use std::path::PathBuf;
    use std::time::SystemTime;

    struct GHFileToUpload {
        pub file_path: PathBuf,
        pub contents: String,
    }


    struct GitProviderOutputSettings {
        pub owner: String,
        pub repo:String,
        pub key_getter: Box<dyn FnOnce() -> String>,
        pub host_url:Option<String>,
        pub host: GitProvider
    }
    enum GitProvider {
        Github,
        Gitlab,
        CustomGitlab
    }

    enum PossibleGitProviderErrors {
        OwnerNotFound,
        RepoNotCreated,
        AuthFailed
    }

    pub fn upload_files_to_repo(files:Vec<GHFileToUpload>,options:GitProviderOutputSettings,git_provider: GitProvider) -> Result<SystemTime, PossibleGitProviderErrors> {
        todo!("Not yet implemented")
    }

    fn upload_files_to_gh_repo(files:Vec<GHFileToUpload>, options: GitProviderOutputSettings) -> Result<SystemTime, PossibleGitProviderErrors> {
        todo!("Not yet implemented")
    }

    fn upload_files_to_gitlab_repo(files:Vec<GHFileToUpload>,options:GitProviderOutputSettings) -> Result<SystemTime, PossibleGitProviderErrors> {
        todo!("Not yet implemented")
    }

    fn check_if_owner_exists(options:GitProviderOutputSettings) -> bool {
        todo!("Not yet implemented")
    }

    fn check_if_repo_exists(options:GitProviderOutputSettings) -> bool {
        //WE DO NOT WANT TO OVERRIDE EXISTING REPOS AT LEAST FOR NOW WITHOUT PROPER TESTING
        todo!("Not yet implemented")
    }

    //https://github.com/jhipster/prettier-java
    //for java formatting

    //i need a way to format code and then publish it to git repo
    //i gotta figure out formatting so i dont have to overly waste time
    //on the java_builder library

    //In no apparent order
    //1. Echo Output in files, run prettier-java and read them back
    //2. Something with GH actions
    //3. Format with a maven command in the resulting java project (needs local files)
    //2 options really, upload to github and let it handle the formatting
    //or output files and run the formatter there. I will think about it (probably output local files)
    //sol: just run the google-java-format executable and get it done with
}

