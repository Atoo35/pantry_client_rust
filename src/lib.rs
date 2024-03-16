use reqwest;
use serde::{Deserialize, Serialize};
use url::Url;

pub mod utils;
pub struct PantryClient {
    base_url: String,
    pantry_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPantryResponse {
    name: String,
    description: String,
    notifications: bool,
    percent_full: i64,
    baskets: Vec<Basket>,
}

#[derive(Deserialize)]
struct Basket {
    name: String,
    ttl: i32,
}

#[derive(Serialize, Debug)]
pub struct UpdatePantryRequest {
    pub name: String,
    pub description: String,
}

impl PantryClient {
    pub fn get_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_pantry_id(&self) -> &str {
        &self.pantry_id
    }

    pub async fn get_pantry(&self) -> Result<GetPantryResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = Url::parse(&format!(
            "{}/pantry/{}",
            self.get_url(),
            self.get_pantry_id()
        ))?;

        let result = client
            .get(url)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let body = result.json::<GetPantryResponse>().await?;
        return Ok(body);
    }

    pub async fn update_pantry_details(
        &self,
        req: UpdatePantryRequest,
    ) -> Result<GetPantryResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = Url::parse(&format!(
            "{}/pantry/{}",
            self.get_url(),
            self.get_pantry_id()
        ))?;

        let result = client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await?;

        let body = result.json::<GetPantryResponse>().await?;
        return Ok(body);
    }

    pub async fn upsert_basket(
        &self,
        basket_name: &str,
        data: serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = Url::parse(&format!(
            "{}/pantry/{}/basket/{}",
            self.get_url(),
            self.get_pantry_id(),
            basket_name
        ))?;

        let result = client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await?;

        let body = result.text().await?;
        return Ok(body);
    }

    pub async fn get_basket_content(
        &self,
        basket_name: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = Url::parse(&format!(
            "{}/pantry/{}/basket/{}",
            self.get_url(),
            self.get_pantry_id(),
            basket_name
        ))?;

        let result = client
            .get(url)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let body = result.json::<serde_json::Value>().await?;
        return Ok(body);
    }

    pub async fn update_basket_content(
        &self,
        basket_name: &str,
        data: serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = Url::parse(&format!(
            "{}/pantry/{}/basket/{}",
            self.get_url(),
            self.get_pantry_id(),
            basket_name
        ))?;

        let result = client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()
            .await?;

        let body = result.text().await?;
        return Ok(body);
    }

    pub async fn delete_basket(
        &self,
        basket_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = Url::parse(&format!(
            "{}/pantry/{}/basket/{}",
            self.get_url(),
            self.get_pantry_id(),
            basket_name
        ))?;

        let result = client
            .delete(url)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let body = result.text().await?;
        return Ok(body);
    }
}

pub fn new_client(pantry_id: &str) -> PantryClient {
    PantryClient {
        base_url: utils::constants::BASE_URL.to_string(),
        pantry_id: String::from(pantry_id),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_url_works() {
//         let result = get_url();
//         assert_eq!(result, utils::constants::BASE_URL.to_string());
//     }
// }
