#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JishoResponse {
    pub meta: Meta,
    pub data: Vec<WordData>,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub status: i32,
}

#[derive(Deserialize, Debug)]
pub struct WordData {
    pub slug: String,
    pub is_common: Option<bool>,
    pub tags: Vec<String>,
    pub jlpt: Vec<String>,
    pub japanese: Vec<JapaneseSource>,
    pub senses: Vec<Sense>,
    pub attribution: Attribution,
}

#[derive(Deserialize, Debug)]
pub struct Sense {
    pub english_definitions: Vec<String>,
    pub parts_of_speech: Vec<String>,
    pub links: Vec<Link>,
    pub tags: Vec<String>,
    pub restrictions: Vec<String>,
    pub see_also: Vec<String>,
    pub antonyms: Vec<String>,
    pub source: Vec<Source>,
    pub info: Vec<String>,
    pub sentences: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct JapaneseSource {
    pub word: Option<String>,
    pub reading: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Attribution {
    pub jmdict: bool,
    pub jmnedict: bool,
    pub dbpedia: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub language: String,
    pub word: String,
}

impl JishoResponse {
    pub async fn search(keyword: &str) -> Result<Self, reqwest::Error> {
        let url = format!("https://jisho.org/api/v1/search/words?keyword={}", keyword);

        let client = reqwest::Client::new();
        let response = client.get(url).send().await?.json::<Self>().await?;

        Ok(response)
    }
}
