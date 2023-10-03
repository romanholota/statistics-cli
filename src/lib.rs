use exitfailure::ExitFailure;
use reqwest::{Url};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Name {
    pub value: String,
    #[serde(rename = "validFrom")]
    valid_from: String,
    #[serde(rename = "validTo")]
    valid_to: Option<String>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Company {
    #[serde(rename = "fullNames")]
    pub full_names: Vec<Name>
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Response {
    pub results: Vec<Company>,
    license: String
}

impl Response {
    pub async fn get(ico: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.statistics.sk/rpo/v1/search?identifier={}",
            ico
        );

        let url = Url::parse(&url)?;
        let res = reqwest::get(url).await?.json::<Response>().await?;

        Ok(res)
    }
}