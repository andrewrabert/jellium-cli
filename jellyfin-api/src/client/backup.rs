use crate::Client;
use crate::error::Error;
use crate::types;

impl Client {
    #[doc = "Gets a list of all currently present backups in the backup directory\n\nSends a `GET` request to `/Backup`\n\n"]
    pub async fn list_backups(&self) -> Result<Vec<types::BackupManifestDto>, Error> {
        self.request(reqwest::Method::GET, "/Backup".into())
            .send()
            .await
    }

    #[doc = "Creates a new Backup\n\nSends a `POST` request to `/Backup/Create`\n\nArguments:\n- `body`: The backup options.\n"]
    pub async fn create_backup(
        &self,
        body: &types::BackupOptionsDto,
    ) -> Result<types::BackupManifestDto, Error> {
        self.request(reqwest::Method::POST, "/Backup/Create".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Gets the descriptor from an existing archive is present\n\nSends a `GET` request to `/Backup/Manifest`\n\nArguments:\n- `path`: The data to start a restore process.\n"]
    pub async fn get_backup(&self, path: &str) -> Result<types::BackupManifestDto, Error> {
        self.request(reqwest::Method::GET, "/Backup/Manifest".into())
            .query("path", path)
            .send()
            .await
    }

    #[doc = "Restores to a backup by restarting the server and applying the backup\n\nSends a `POST` request to `/Backup/Restore`\n\nArguments:\n- `body`: The data to start a restore process.\n"]
    pub async fn start_restore_backup(
        &self,
        body: &types::BackupRestoreRequestDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Backup/Restore".into())
            .json_body(body)
            .send_no_content()
            .await
    }
}
