use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use time::Date;
use crate::lang::Language;
use chrono::{DateTime, Local};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct MyDate(Date);

#[derive(Debug, Clone)]
pub(crate) struct Events {
    events: Arc<Vec<Arc<Event>>>,
}

#[derive(Deserialize)]
pub(crate) struct EventMeta {
    title: String,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    location: String,
    description: String,
    keywords: Vec<String>,
    image: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct Event {
    slug: String,
    lang: Language,
    idx: u32,
    title: String,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    location: String,
    description: String,
    keywords: Vec<String>,
    image: Option<String>,
    body: String,
}



impl Events {
    pub(crate) async fn load(directory: &Path) -> anyhow::Result<Self> {
        let mut events = Vec::new();

        let mut dir = tokio::fs::read_dir(directory).await?;
        while let Some(entry) = dir.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                continue;
            }

            let path = entry.path();
            let content = tokio::fs::read_to_string(path.as_path()).await?;
            let content = content.trim_start();
            let content = content.strip_prefix("---").unwrap();
            let (meta, body) = content.split_once("---").unwrap();

            let meta: EventMeta = serde_yaml::from_str(meta)?;
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.starts_with('_') {
                continue;
            }

            let (idx, lang, slug) = crate::news::parse_file_name(file_name)?;


            let event_ptr = Arc::new(Event {
                slug: slug.to_string(),
                lang,
                idx,
                title: meta.title,
                start_time: meta.start_time,
                end_time: meta.end_time,
                description: meta.description,
                keywords: meta.keywords,
                image: meta.image,
                location: meta.location,
                body: markdown::to_html(body),
            });

            events.push(event_ptr);
        }

        events.sort_by(|a, b| b.start_time.cmp(&a.start_time));

        Ok(Self {
            events: Arc::new(events)
        })
    }

    pub(crate) async fn get_future_events(&self, lang: &Language) -> Vec<Arc<Event>> {
        let current_date_time = chrono::offset::Local::now();

        self
            .events
            .iter()
            .filter(|post| post.end_time > current_date_time && post.lang == *lang)
            .cloned()
            .collect()
    }

    pub(crate) async fn get_all_events(&self, lang: &Language) -> Vec<Arc<Event>> {
        self
            .events
            .iter()
            .filter(|post| post.lang == *lang)
            .cloned()
            .collect()
    }
}