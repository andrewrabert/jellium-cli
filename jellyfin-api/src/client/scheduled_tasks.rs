use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Get tasks\n\nSends a `GET` request to `/ScheduledTasks`\n\nArguments:\n- `is_enabled`: Optional filter tasks that are enabled, or not.\n- `is_hidden`: Optional filter tasks that are hidden, or not.\n"]
    pub async fn get_tasks(
        &self,
        is_enabled: Option<bool>,
        is_hidden: Option<bool>,
    ) -> Result<Vec<types::TaskInfo>, Error> {
        self.request(reqwest::Method::GET, "/ScheduledTasks".into())
            .query_opt("isEnabled", is_enabled)
            .query_opt("isHidden", is_hidden)
            .send()
            .await
    }

    #[doc = "Get task by id\n\nSends a `GET` request to `/ScheduledTasks/{taskId}`\n\nArguments:\n- `task_id`: Task Id.\n"]
    pub async fn get_task(
        &self,
        task_id: &str,
    ) -> Result<types::TaskInfo, Error> {
        self.request(reqwest::Method::GET, format!("/ScheduledTasks/{}", encode_path(task_id)))
            .send()
            .await
    }

    #[doc = "Update specified task triggers\n\nSends a `POST` request to `/ScheduledTasks/{taskId}/Triggers`\n\nArguments:\n- `task_id`: Task Id.\n- `body`: Triggers.\n"]
    pub async fn update_task(
        &self,
        task_id: &str,
        body: &Vec<types::TaskTriggerInfo>,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/ScheduledTasks/{}/Triggers", encode_path(task_id)))
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Start specified task\n\nSends a `POST` request to `/ScheduledTasks/Running/{taskId}`\n\nArguments:\n- `task_id`: Task Id.\n"]
    pub async fn start_task(
        &self,
        task_id: &str,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/ScheduledTasks/Running/{}", encode_path(task_id)))
            .send_no_content()
            .await
    }

    #[doc = "Stop specified task\n\nSends a `DELETE` request to `/ScheduledTasks/Running/{taskId}`\n\nArguments:\n- `task_id`: Task Id.\n"]
    pub async fn stop_task(&self, task_id: &str) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, format!("/ScheduledTasks/Running/{}", encode_path(task_id)))
            .send_no_content()
            .await
    }
}
