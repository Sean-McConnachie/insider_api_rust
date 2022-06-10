#[derive(Debug)]
pub struct Settings {
    pub sec: Sec
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            sec: Sec::default()
        }
    }
}

#[derive(Debug)]
pub struct Sec {
    pub delay_milli: u16,
    pub concurrent: u16,
    pub json_recent_url: String,
    pub json_subsequent_url: String,
    pub rss_feed_url: String,
    pub rss_count: u16,
    pub xml_url: String
}

impl Default for Sec {
    fn default() -> Self {
        Sec {
            delay_milli: 150,
            concurrent: 5,
            json_recent_url: "https://data.sec.gov/submissions/CIK{}.json".to_string(),
            json_subsequent_url: "https://data.sec.gov/submissions/{}".to_string(),
            rss_feed_url: "https://data.sec.gov/rss?cik={}&type=3,4,5&count={}".to_string(),
            rss_count: 20,
            xml_url: "https://www.sec.gov/Archives/edgar/data/{}/{}/{}".to_string()
        }
    }
}