use crate::Client;
use crate::error::Error;
use crate::types;

impl Client {
    #[doc = "Gets known countries\n\nSends a `GET` request to `/Localization/Countries`\n\n"]
    pub async fn get_countries(&self) -> Result<Vec<types::CountryInfo>, Error> {
        self.request(reqwest::Method::GET, "/Localization/Countries".into())
            .send()
            .await
    }

    #[doc = "Gets known cultures\n\nSends a `GET` request to `/Localization/Cultures`\n\n"]
    pub async fn get_cultures(&self) -> Result<Vec<types::CultureDto>, Error> {
        self.request(reqwest::Method::GET, "/Localization/Cultures".into())
            .send()
            .await
    }

    #[doc = "Gets localization options\n\nSends a `GET` request to `/Localization/Options`\n\n"]
    pub async fn get_localization_options(&self) -> Result<Vec<types::LocalizationOption>, Error> {
        self.request(reqwest::Method::GET, "/Localization/Options".into())
            .send()
            .await
    }

    #[doc = "Gets known parental ratings\n\nSends a `GET` request to `/Localization/ParentalRatings`\n\n"]
    pub async fn get_parental_ratings(&self) -> Result<Vec<types::ParentalRating>, Error> {
        self.request(reqwest::Method::GET, "/Localization/ParentalRatings".into())
            .send()
            .await
    }
}
