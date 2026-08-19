use crate::Client;
use crate::error::Error;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Gets available packages\n\nSends a `GET` request to `/Packages`\n\n"]
    pub async fn get_packages(&self) -> Result<Vec<types::PackageInfo>, Error> {
        self.request(reqwest::Method::GET, "/Packages".into())
            .send()
            .await
    }

    #[doc = "Gets a package by name or assembly GUID\n\nSends a `GET` request to `/Packages/{name}`\n\nArguments:\n- `name`: The name of the package.\n- `assembly_guid`: The GUID of the associated assembly.\n"]
    pub async fn get_package_info(
        &self,
        name: &str,
        assembly_guid: Option<&uuid::Uuid>,
    ) -> Result<types::PackageInfo, Error> {
        self.request(
            reqwest::Method::GET,
            format!("/Packages/{}", encode_path(name)),
        )
        .query_opt("assemblyGuid", assembly_guid)
        .send()
        .await
    }

    #[doc = "Installs a package\n\nSends a `POST` request to `/Packages/Installed/{name}`\n\nArguments:\n- `name`: Package name.\n- `assembly_guid`: GUID of the associated assembly.\n- `repository_url`: Optional. Specify the repository to install from.\n- `version`: Optional version. Defaults to latest version.\n"]
    pub async fn install_package(
        &self,
        name: &str,
        assembly_guid: Option<&uuid::Uuid>,
        repository_url: Option<&str>,
        version: Option<&str>,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/Packages/Installed/{}", encode_path(name)),
        )
        .query_opt("assemblyGuid", assembly_guid)
        .query_opt("repositoryUrl", repository_url)
        .query_opt("version", version)
        .send_no_content()
        .await
    }

    #[doc = "Cancels a package installation\n\nSends a `DELETE` request to `/Packages/Installing/{packageId}`\n\nArguments:\n- `package_id`: Installation Id.\n"]
    pub async fn cancel_package_installation(&self, package_id: &uuid::Uuid) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!(
                "/Packages/Installing/{}",
                encode_path(&package_id.to_string())
            ),
        )
        .send_no_content()
        .await
    }

    #[doc = "Gets all package repositories\n\nSends a `GET` request to `/Repositories`\n\n"]
    pub async fn get_repositories(&self) -> Result<Vec<types::RepositoryInfo>, Error> {
        self.request(reqwest::Method::GET, "/Repositories".into())
            .send()
            .await
    }

    #[doc = "Sets the enabled and existing package repositories\n\nSends a `POST` request to `/Repositories`\n\nArguments:\n- `body`: The list of package repositories.\n"]
    pub async fn set_repositories(&self, body: &Vec<types::RepositoryInfo>) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Repositories".into())
            .json_body(body)
            .send_no_content()
            .await
    }
}
