use serde::Deserialize;
use std::error::Error;

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

#[derive(Deserialize, Debug)]
pub struct Story {
    pub by: String,
    pub descendants: u32,
    pub kids: Vec<u32>,
    pub score: u32,
    pub time: u32,
    pub title: String,
    pub r#type: String,
    pub url: String,
}

pub fn get_story(id: usize) -> Result<Story, Box<dyn Error>> {
    let url = format!("{}/item/{}.json", BASE_URL, id);
    let response = reqwest::blocking::get(&url)?;
    let story: Story = response.json()?;
    Ok(story)
}

pub fn get_topstories() -> Result<Vec<Story>, Box<dyn Error>> {
    let url = format!("{}/topstories.json", BASE_URL);
    let ids: Vec<usize> = reqwest::blocking::get(url)?.json()?;
    let stories = ids
        .into_iter()
        .take(5)
        .map(|id| get_story(id))
        .collect::<Result<Vec<Story>, Box<dyn Error>>>()?;
    Ok(stories)
}
