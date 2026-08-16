use crate::Client;
use crate::error::Error;
use crate::types;

impl Client {
    #[doc = "Gets branding configuration\n\nSends a `GET` request to `/Branding/Configuration`\n\n"]
    pub async fn get_branding_options(&self) -> Result<types::BrandingOptionsDto, Error> {
        self.request(reqwest::Method::GET, "/Branding/Configuration".into())
            .send()
            .await
    }

    #[doc = "Gets branding css\n\nSends a `GET` request to `/Branding/Css`\n\n"]
    pub async fn get_branding_css(&self) -> Result<String, Error> {
        self.request(reqwest::Method::GET, "/Branding/Css".into())
            .send()
            .await
    }

    #[doc = "Gets branding css\n\nSends a `GET` request to `/Branding/Css.css`\n\n"]
    pub async fn get_branding_css_file(&self) -> Result<String, Error> {
        self.request(reqwest::Method::GET, "/Branding/Css.css".into())
            .send()
            .await
    }

    #[doc = "Generates or gets the splashscreen\n\nSends a `GET` request to `/Branding/Splashscreen`\n\nArguments:\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `tag`: Supply the cache tag from the item object to receive strong caching headers.\n"]
    pub async fn get_splashscreen(
        &self,
        format: Option<types::ImageFormat>,
        tag: Option<&str>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, "/Branding/Splashscreen".into())
            .query_opt("format", format)
            .query_opt("tag", tag)
            .send_response()
            .await
    }

    #[doc = "Uploads a custom splashscreen.\r\nThe body is expected to the image contents base64 encoded\n\nSends a `POST` request to `/Branding/Splashscreen`\n\n"]
    pub async fn upload_custom_splashscreen<B: Into<reqwest::Body>>(
        &self,
        body: B,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Branding/Splashscreen".into())
            .raw_body(body, "application/octet-stream")
            .send_no_content()
            .await
    }

    #[doc = "Delete a custom splashscreen\n\nSends a `DELETE` request to `/Branding/Splashscreen`\n\n"]
    pub async fn delete_custom_splashscreen(&self) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, "/Branding/Splashscreen".into())
            .send_no_content()
            .await
    }
}
