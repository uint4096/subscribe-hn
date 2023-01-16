use reqwest;
use serde::Deserialize;

pub struct HN;

#[derive(Deserialize)]
pub struct News {
    pub title: String,
    pub url: String,
    pub dead: bool,
    pub id: u16,
    pub text: String,
}

impl<'a> HN {
    const BASE_URL: &'a str = "https://hacker-news.firebaseio.com/v0/";
    const NEW_STORIES: &'a str = "newstories.json";
    const ITEM: &'a str = "item";

    pub async fn get_story_ids() -> Vec<u16> {
        if let Ok(response) = reqwest::get(format!("{}/{}", HN::BASE_URL, HN::NEW_STORIES)).await {
            if let Ok(ids) = response.json::<Vec<u16>>().await {
                return ids;
            }
        }

        panic!("Unable to fetch story ids!");
    }

    pub async fn get_story(id: u16) -> News {
        if let Ok(response) = reqwest::get(format!(
            "{}/{}/{}.json",
            HN::BASE_URL,
            HN::ITEM,
            id.to_string()
        ))
        .await
        {
            if let Ok(news) = response.json::<News>().await {
                return news;
            }
        }

        panic!("Unable to fetch story!");
    }
}
