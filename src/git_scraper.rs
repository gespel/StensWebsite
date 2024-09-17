use reqwest;
use serde_json::{Value};

pub(crate) struct GitScraper {
    pub repo_number: usize
}

impl GitScraper {
    pub(crate) async fn new() -> GitScraper {
        let mut number_repos = 0;
        let body = reqwest::Client::new()
            .get("https://api.github.com/users/gespel")
            .header("User-Agent", "StensWebsiteServer")
            .send()
            .await.unwrap()
            .text()
            .await
            .unwrap();

        let git_json: Value = serde_json::from_str(&body).unwrap();

        println!("{:#?}", git_json);

        if let Some(number) = git_json["public_repos"].as_u64() {
            number_repos = number;
        }

        GitScraper {
            repo_number: number_repos as usize,

        }
    }

}