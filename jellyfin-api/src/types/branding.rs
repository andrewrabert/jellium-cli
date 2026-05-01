
#[doc = "The branding options DTO for API use.\r\nThis DTO excludes SplashscreenLocation to prevent it from being updated via API."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BrandingOptionsDto {
    #[doc = "Gets or sets the custom CSS."]
    #[serde(
        rename = "CustomCss",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_css: Option<String>,
    #[doc = "Gets or sets the login disclaimer."]
    #[serde(
        rename = "LoginDisclaimer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub login_disclaimer: Option<String>,
    #[doc = "Gets or sets a value indicating whether to enable the splashscreen."]
    #[serde(
        rename = "SplashscreenEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub splashscreen_enabled: Option<bool>,
}

impl Default for BrandingOptionsDto {
    fn default() -> Self {
        Self {
            custom_css: Default::default(),
            login_disclaimer: Default::default(),
            splashscreen_enabled: Default::default(),
        }
    }
}

