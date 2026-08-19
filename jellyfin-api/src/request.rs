use crate::Client;
use crate::error::Error;
use crate::pagination::Paginated;

enum RequestBody {
    Json(serde_json::Value),
    Raw {
        body: reqwest::Body,
        content_type: &'static str,
    },
}

pub struct ApiRequest<'a> {
    client: &'a Client,
    method: reqwest::Method,
    path: String,
    query: Vec<(&'static str, String)>,
    body: Option<RequestBody>,
}

impl Client {
    pub fn request(&self, method: reqwest::Method, path: String) -> ApiRequest<'_> {
        ApiRequest {
            client: self,
            method,
            path,
            query: Vec::new(),
            body: None,
        }
    }
}

impl<'a> ApiRequest<'a> {
    /// Add a required query parameter.
    pub fn query(mut self, key: &'static str, value: impl std::fmt::Display) -> Self {
        self.query.push((key, value.to_string()));
        self
    }

    /// Add an optional query parameter (skipped if None).
    pub fn query_opt(mut self, key: &'static str, value: Option<impl std::fmt::Display>) -> Self {
        if let Some(v) = value {
            self.query.push((key, v.to_string()));
        }
        self
    }

    /// Add each element of a slice as a repeated query parameter.
    pub fn query_list(mut self, key: &'static str, values: &[impl std::fmt::Display]) -> Self {
        for v in values {
            self.query.push((key, v.to_string()));
        }
        self
    }

    /// Add each element of an optional slice as a repeated query parameter.
    pub fn query_list_opt(
        mut self,
        key: &'static str,
        values: Option<&Vec<impl std::fmt::Display>>,
    ) -> Self {
        if let Some(vs) = values {
            for v in vs {
                self.query.push((key, v.to_string()));
            }
        }
        self
    }

    /// Set a JSON request body.
    pub fn json_body(mut self, body: &impl serde::Serialize) -> Self {
        self.body = Some(RequestBody::Json(
            serde_json::to_value(body).expect("failed to serialize body"),
        ));
        self
    }

    /// Set a raw request body with a content type.
    pub fn raw_body(mut self, body: impl Into<reqwest::Body>, content_type: &'static str) -> Self {
        self.body = Some(RequestBody::Raw {
            body: body.into(),
            content_type,
        });
        self
    }

    fn build(self) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.client.baseurl, self.path);
        let mut req = self.client.client.request(self.method, &url);
        req = req.header(reqwest::header::ACCEPT, "application/json");
        for (k, v) in &self.query {
            req = req.query(&[(k, v)]);
        }
        match self.body {
            Some(RequestBody::Json(v)) => {
                req = req.json(&v);
            }
            Some(RequestBody::Raw { body, content_type }) => {
                req = req
                    .header(reqwest::header::CONTENT_TYPE, content_type)
                    .body(body);
            }
            None => {}
        }
        req
    }

    async fn handle_error(response: reqwest::Response) -> Error {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Error::Status { status, body }
    }

    /// Send and deserialize the JSON response.
    pub async fn send<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        let response = self.build().send().await?;
        if response.status().is_success() {
            let body = response.text().await?;
            serde_json::from_str(&body).map_err(|source| Error::Deserialization { source, body })
        } else {
            Err(Self::handle_error(response).await)
        }
    }

    /// Send expecting no response body (204 etc).
    pub async fn send_no_content(self) -> Result<(), Error> {
        let response = self.build().send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Self::handle_error(response).await)
        }
    }

    /// Send and return the raw response (for streaming/binary).
    pub async fn send_response(self) -> Result<reqwest::Response, Error> {
        let response = self.build().send().await?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Self::handle_error(response).await)
        }
    }

    /// Send with automatic pagination, collecting all items across pages.
    ///
    /// Fetches pages of `page_size` items, advancing `startIndex` until all
    /// items are retrieved. Any existing `startIndex`/`limit` query params
    /// are replaced.
    pub async fn send_paginated<C>(self, page_size: i32) -> Result<Vec<C::Item>, Error>
    where
        C: Paginated + serde::de::DeserializeOwned,
    {
        let client = self.client;
        let method = self.method;
        let path = self.path;
        let base_query: Vec<_> = self
            .query
            .into_iter()
            .filter(|(k, _)| *k != "startIndex" && *k != "limit")
            .collect();

        let mut all_items = Vec::new();
        let mut start_index: i32 = 0;

        loop {
            let url = format!("{}{}", client.baseurl, path);
            let mut req = client.client.request(method.clone(), &url);
            req = req.header(reqwest::header::ACCEPT, "application/json");
            for (k, v) in &base_query {
                req = req.query(&[(k, v)]);
            }
            req = req.query(&[("startIndex", start_index.to_string())]);
            req = req.query(&[("limit", page_size.to_string())]);

            let response = req.send().await?;
            if !response.status().is_success() {
                return Err(Self::handle_error(response).await);
            }

            let body = response.text().await?;
            let mut page: C = serde_json::from_str(&body)
                .map_err(|source| Error::Deserialization { source, body })?;

            let items = page.items();
            let count = items.len() as i32;
            all_items.extend(items);

            let total = page.total_record_count().unwrap_or(0);
            start_index += count;
            if start_index >= total || count == 0 {
                break;
            }
        }

        Ok(all_items)
    }
}
