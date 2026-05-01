use crate::types;
use crate::error::Error;
use crate::Client;

impl Client {
    #[doc = "Get Default directory browser\n\nSends a `GET` request to `/Environment/DefaultDirectoryBrowser`\n\n"]
    pub async fn get_default_directory_browser(
        &self,
    ) -> Result<types::DefaultDirectoryBrowserInfoDto, Error> {
        self.request(reqwest::Method::GET, "/Environment/DefaultDirectoryBrowser".into())
            .send()
            .await
    }

    #[doc = "Gets the contents of a given directory in the file system\n\nSends a `GET` request to `/Environment/DirectoryContents`\n\nArguments:\n- `include_directories`: An optional filter to include or exclude folders from the results. true/false.\n- `include_files`: An optional filter to include or exclude files from the results. true/false.\n- `path`: The path.\n"]
    pub async fn get_directory_contents(
        &self,
        include_directories: Option<bool>,
        include_files: Option<bool>,
        path: &str,
    ) -> Result<Vec<types::FileSystemEntryInfo>, Error> {
        self.request(reqwest::Method::GET, "/Environment/DirectoryContents".into())
            .query_opt("includeDirectories", include_directories)
            .query_opt("includeFiles", include_files)
            .query("path", path)
            .send()
            .await
    }

    #[doc = "Gets available drives from the server's file system\n\nSends a `GET` request to `/Environment/Drives`\n\n"]
    pub async fn get_drives(
        &self,
    ) -> Result<Vec<types::FileSystemEntryInfo>, Error> {
        self.request(reqwest::Method::GET, "/Environment/Drives".into())
            .send()
            .await
    }

    #[doc = "Gets network paths\n\nSends a `GET` request to `/Environment/NetworkShares`\n\n"]
    pub async fn get_network_shares(
        &self,
    ) -> Result<Vec<types::FileSystemEntryInfo>, Error> {
        self.request(reqwest::Method::GET, "/Environment/NetworkShares".into())
            .send()
            .await
    }

    #[doc = "Gets the parent path of a given path\n\nSends a `GET` request to `/Environment/ParentPath`\n\nArguments:\n- `path`: The path.\n"]
    pub async fn get_parent_path(
        &self,
        path: &str,
    ) -> Result<String, Error> {
        self.request(reqwest::Method::GET, "/Environment/ParentPath".into())
            .query("path", path)
            .send()
            .await
    }

    #[doc = "Validates path\n\nSends a `POST` request to `/Environment/ValidatePath`\n\nArguments:\n- `body`: Validate request object.\n"]
    pub async fn validate_path(
        &self,
        body: &types::ValidatePathDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Environment/ValidatePath".into())
            .json_body(body)
            .send_no_content()
            .await
    }
}
