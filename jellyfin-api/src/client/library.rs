use crate::Client;
use crate::error::Error;
use crate::types;

impl Client {
    #[doc = "Gets the library options info\n\nSends a `GET` request to `/Libraries/AvailableOptions`\n\nArguments:\n- `is_new_library`: Whether this is a new library.\n- `library_content_type`: Library content type.\n"]
    pub async fn get_library_options_info(
        &self,
        is_new_library: Option<bool>,
        library_content_type: Option<types::CollectionType>,
    ) -> Result<types::LibraryOptionsResultDto, Error> {
        self.request(reqwest::Method::GET, "/Libraries/AvailableOptions".into())
            .query_opt("isNewLibrary", is_new_library)
            .query_opt("libraryContentType", library_content_type)
            .send()
            .await
    }

    #[doc = "Reports that new movies have been added by an external source\n\nSends a `POST` request to `/Library/Media/Updated`\n\nArguments:\n- `body`: The update paths.\n"]
    pub async fn post_updated_media(&self, body: &types::MediaUpdateInfoDto) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Library/Media/Updated".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets all user media folders\n\nSends a `GET` request to `/Library/MediaFolders`\n\nArguments:\n- `is_hidden`: Optional. Filter by folders that are marked hidden, or not.\n"]
    pub async fn get_media_folders(
        &self,
        is_hidden: Option<bool>,
    ) -> Result<types::BaseItemDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Library/MediaFolders".into())
            .query_opt("isHidden", is_hidden)
            .send()
            .await
    }

    #[doc = "Reports that new movies have been added by an external source\n\nSends a `POST` request to `/Library/Movies/Added`\n\nArguments:\n- `imdb_id`: The imdbId.\n- `tmdb_id`: The tmdbId.\n"]
    pub async fn post_added_movies(
        &self,
        imdb_id: Option<&str>,
        tmdb_id: Option<&str>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Library/Movies/Added".into())
            .query_opt("imdbId", imdb_id)
            .query_opt("tmdbId", tmdb_id)
            .send_no_content()
            .await
    }

    #[doc = "Reports that new movies have been added by an external source\n\nSends a `POST` request to `/Library/Movies/Updated`\n\nArguments:\n- `imdb_id`: The imdbId.\n- `tmdb_id`: The tmdbId.\n"]
    pub async fn post_updated_movies(
        &self,
        imdb_id: Option<&str>,
        tmdb_id: Option<&str>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Library/Movies/Updated".into())
            .query_opt("imdbId", imdb_id)
            .query_opt("tmdbId", tmdb_id)
            .send_no_content()
            .await
    }

    #[doc = "Gets a list of physical paths from virtual folders\n\nSends a `GET` request to `/Library/PhysicalPaths`\n\n"]
    pub async fn get_physical_paths(&self) -> Result<Vec<String>, Error> {
        self.request(reqwest::Method::GET, "/Library/PhysicalPaths".into())
            .send()
            .await
    }

    #[doc = "Starts a library scan\n\nSends a `POST` request to `/Library/Refresh`\n\n"]
    pub async fn refresh_library(&self) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Library/Refresh".into())
            .send_no_content()
            .await
    }

    #[doc = "Reports that new episodes of a series have been added by an external source\n\nSends a `POST` request to `/Library/Series/Added`\n\nArguments:\n- `tvdb_id`: The tvdbId.\n"]
    pub async fn post_added_series(&self, tvdb_id: Option<&str>) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Library/Series/Added".into())
            .query_opt("tvdbId", tvdb_id)
            .send_no_content()
            .await
    }

    #[doc = "Reports that new episodes of a series have been added by an external source\n\nSends a `POST` request to `/Library/Series/Updated`\n\nArguments:\n- `tvdb_id`: The tvdbId.\n"]
    pub async fn post_updated_series(&self, tvdb_id: Option<&str>) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Library/Series/Updated".into())
            .query_opt("tvdbId", tvdb_id)
            .send_no_content()
            .await
    }

    #[doc = "Gets all virtual folders\n\nSends a `GET` request to `/Library/VirtualFolders`\n\n"]
    pub async fn get_virtual_folders(&self) -> Result<Vec<types::VirtualFolderInfo>, Error> {
        self.request(reqwest::Method::GET, "/Library/VirtualFolders".into())
            .send()
            .await
    }

    #[doc = "Adds a virtual folder\n\nSends a `POST` request to `/Library/VirtualFolders`\n\nArguments:\n- `collection_type`: The type of the collection.\n- `name`: The name of the virtual folder.\n- `paths`: The paths of the virtual folder.\n- `refresh_library`: Whether to refresh the library.\n- `body`: The library options.\n"]
    pub async fn add_virtual_folder(
        &self,
        collection_type: Option<types::CollectionTypeOptions>,
        name: Option<&str>,
        paths: Option<&Vec<String>>,
        refresh_library: Option<bool>,
        body: &types::AddVirtualFolderDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Library/VirtualFolders".into())
            .query_opt("collectionType", collection_type)
            .query_opt("name", name)
            .query_list_opt("paths", paths)
            .query_opt("refreshLibrary", refresh_library)
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Removes a virtual folder\n\nSends a `DELETE` request to `/Library/VirtualFolders`\n\nArguments:\n- `name`: The name of the folder.\n- `refresh_library`: Whether to refresh the library.\n"]
    pub async fn remove_virtual_folder(
        &self,
        name: Option<&str>,
        refresh_library: Option<bool>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, "/Library/VirtualFolders".into())
            .query_opt("name", name)
            .query_opt("refreshLibrary", refresh_library)
            .send_no_content()
            .await
    }

    #[doc = "Update library options\n\nSends a `POST` request to `/Library/VirtualFolders/LibraryOptions`\n\nArguments:\n- `body`: The library name and options.\n"]
    pub async fn update_library_options(
        &self,
        body: &types::UpdateLibraryOptionsDto,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            "/Library/VirtualFolders/LibraryOptions".into(),
        )
        .json_body(body)
        .send_no_content()
        .await
    }

    #[doc = "Renames a virtual folder\n\nSends a `POST` request to `/Library/VirtualFolders/Name`\n\nArguments:\n- `name`: The name of the virtual folder.\n- `new_name`: The new name.\n- `refresh_library`: Whether to refresh the library.\n"]
    pub async fn rename_virtual_folder(
        &self,
        name: Option<&str>,
        new_name: Option<&str>,
        refresh_library: Option<bool>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Library/VirtualFolders/Name".into())
            .query_opt("name", name)
            .query_opt("newName", new_name)
            .query_opt("refreshLibrary", refresh_library)
            .send_no_content()
            .await
    }

    #[doc = "Add a media path to a library\n\nSends a `POST` request to `/Library/VirtualFolders/Paths`\n\nArguments:\n- `refresh_library`: Whether to refresh the library.\n- `body`: The media path dto.\n"]
    pub async fn add_media_path(
        &self,
        refresh_library: Option<bool>,
        body: &types::MediaPathDto,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            "/Library/VirtualFolders/Paths".into(),
        )
        .query_opt("refreshLibrary", refresh_library)
        .json_body(body)
        .send_no_content()
        .await
    }

    #[doc = "Remove a media path\n\nSends a `DELETE` request to `/Library/VirtualFolders/Paths`\n\nArguments:\n- `name`: The name of the library.\n- `path`: The path to remove.\n- `refresh_library`: Whether to refresh the library.\n"]
    pub async fn remove_media_path(
        &self,
        name: Option<&str>,
        path: Option<&str>,
        refresh_library: Option<bool>,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            "/Library/VirtualFolders/Paths".into(),
        )
        .query_opt("name", name)
        .query_opt("path", path)
        .query_opt("refreshLibrary", refresh_library)
        .send_no_content()
        .await
    }

    #[doc = "Updates a media path\n\nSends a `POST` request to `/Library/VirtualFolders/Paths/Update`\n\nArguments:\n- `body`: The name of the library and path infos.\n"]
    pub async fn update_media_path(
        &self,
        body: &types::UpdateMediaPathRequestDto,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            "/Library/VirtualFolders/Paths/Update".into(),
        )
        .json_body(body)
        .send_no_content()
        .await
    }
}
