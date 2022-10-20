use std::{
    sync::mpsc::{channel, Receiver},
    thread,
};

use eframe::{egui::Context, CreationContext};

use crate::fetch_stories;

pub struct StoryCardData {
    pub title: String,
    pub url: String,
    pub by: String,
    pub score: u32,
    pub time: u32,
    pub descendants: u32,
}

#[derive(Default)]
pub struct HackerNews {
    pub stories: Vec<StoryCardData>,
    pub stories_rx: Option<Receiver<StoryCardData>>,
}

impl HackerNews {
    pub fn init(mut self, cc: &CreationContext) -> Self {
        tracing::info!("successfully initialized");

        let (mut stories_tx, stories_rx) = channel();
        self.stories_rx = Some(stories_rx);
        thread::spawn(move || {
            let x = fetch_stories(&mut stories_tx);
        });

        self
    }

    fn configure_fonts(&mut self, ctx: &Context) {
        todo!()
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        let _ = &self.stories.iter().for_each(|story| {
            ui.horizontal(|ui| {
                ui.label(story.title.as_str());
            });
        });
    }
}
