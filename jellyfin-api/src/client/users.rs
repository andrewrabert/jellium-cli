use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Gets a list of users\n\nSends a `GET` request to `/Users`\n\nArguments:\n- `is_disabled`: Optional filter by IsDisabled=true or false.\n- `is_hidden`: Optional filter by IsHidden=true or false.\n"]
    pub async fn get_users(
        &self,
        is_disabled: Option<bool>,
        is_hidden: Option<bool>,
    ) -> Result<Vec<types::UserDto>, Error> {
        self.request(reqwest::Method::GET, "/Users".into())
            .query_opt("isDisabled", is_disabled)
            .query_opt("isHidden", is_hidden)
            .send()
            .await
    }

    #[doc = "Updates a user\n\nSends a `POST` request to `/Users`\n\nArguments:\n- `user_id`: The user id.\n- `body`: The updated user model.\n"]
    pub async fn update_user(
        &self,
        user_id: Option<&uuid::Uuid>,
        body: &types::UserDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Users".into())
            .query_opt("userId", user_id)
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets a user by Id\n\nSends a `GET` request to `/Users/{userId}`\n\nArguments:\n- `user_id`: The user id.\n"]
    pub async fn get_user_by_id(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<types::UserDto, Error> {
        self.request(reqwest::Method::GET, format!("/Users/{}", encode_path(&user_id.to_string())))
            .send()
            .await
    }

    #[doc = "Deletes a user\n\nSends a `DELETE` request to `/Users/{userId}`\n\nArguments:\n- `user_id`: The user id.\n"]
    pub async fn delete_user(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, format!("/Users/{}", encode_path(&user_id.to_string())))
            .send_no_content()
            .await
    }

    #[doc = "Updates a user policy\n\nSends a `POST` request to `/Users/{userId}/Policy`\n\nArguments:\n- `user_id`: The user id.\n- `body`: The new user policy.\n"]
    pub async fn update_user_policy(
        &self,
        user_id: &uuid::Uuid,
        body: &types::UserPolicy,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/Users/{}/Policy", encode_path(&user_id.to_string())))
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Authenticates a user by name\n\nSends a `POST` request to `/Users/AuthenticateByName`\n\nArguments:\n- `body`: The M:Jellyfin.Api.Controllers.UserController.AuthenticateUserByName(Jellyfin.Api.Models.UserDtos.AuthenticateUserByName) request.\n"]
    pub async fn authenticate_user_by_name(
        &self,
        body: &types::AuthenticateUserByName,
    ) -> Result<types::AuthenticationResult, Error> {
        self.request(reqwest::Method::POST, "/Users/AuthenticateByName".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Authenticates a user with quick connect\n\nSends a `POST` request to `/Users/AuthenticateWithQuickConnect`\n\nArguments:\n- `body`: The Jellyfin.Api.Models.UserDtos.QuickConnectDto request.\n"]
    pub async fn authenticate_with_quick_connect(
        &self,
        body: &types::QuickConnectDto,
    ) -> Result<types::AuthenticationResult, Error> {
        self.request(reqwest::Method::POST, "/Users/AuthenticateWithQuickConnect".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Updates a user configuration\n\nSends a `POST` request to `/Users/Configuration`\n\nArguments:\n- `user_id`: The user id.\n- `body`: The new user configuration.\n"]
    pub async fn update_user_configuration(
        &self,
        user_id: Option<&uuid::Uuid>,
        body: &types::UserConfiguration,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Users/Configuration".into())
            .query_opt("userId", user_id)
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Initiates the forgot password process for a local user\n\nSends a `POST` request to `/Users/ForgotPassword`\n\nArguments:\n- `body`: The forgot password request containing the entered username.\n"]
    pub async fn forgot_password(
        &self,
        body: &types::ForgotPasswordDto,
    ) -> Result<types::ForgotPasswordResult, Error> {
        self.request(reqwest::Method::POST, "/Users/ForgotPassword".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Redeems a forgot password pin\n\nSends a `POST` request to `/Users/ForgotPassword/Pin`\n\nArguments:\n- `body`: The forgot password pin request containing the entered pin.\n"]
    pub async fn forgot_password_pin(
        &self,
        body: &types::ForgotPasswordPinDto,
    ) -> Result<types::PinRedeemResult, Error> {
        self.request(reqwest::Method::POST, "/Users/ForgotPassword/Pin".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Gets the user based on auth token\n\nSends a `GET` request to `/Users/Me`\n\n"]
    pub async fn get_current_user(
        &self,
    ) -> Result<types::UserDto, Error> {
        self.request(reqwest::Method::GET, "/Users/Me".into())
            .send()
            .await
    }

    #[doc = "Creates a user\n\nSends a `POST` request to `/Users/New`\n\nArguments:\n- `body`: The create user by name request body.\n"]
    pub async fn create_user_by_name(
        &self,
        body: &types::CreateUserByName,
    ) -> Result<types::UserDto, Error> {
        self.request(reqwest::Method::POST, "/Users/New".into())
            .json_body(body)
            .send()
            .await
    }

    #[doc = "Updates a user's password\n\nSends a `POST` request to `/Users/Password`\n\nArguments:\n- `user_id`: The user id.\n- `body`: The M:Jellyfin.Api.Controllers.UserController.UpdateUserPassword(System.Nullable{System.Guid},Jellyfin.Api.Models.UserDtos.UpdateUserPassword) request.\n"]
    pub async fn update_user_password(
        &self,
        user_id: Option<&uuid::Uuid>,
        body: &types::UpdateUserPassword,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Users/Password".into())
            .query_opt("userId", user_id)
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets a list of publicly visible users for display on a login screen\n\nSends a `GET` request to `/Users/Public`\n\n"]
    pub async fn get_public_users(
        &self,
    ) -> Result<Vec<types::UserDto>, Error> {
        self.request(reqwest::Method::GET, "/Users/Public".into())
            .send()
            .await
    }
}
