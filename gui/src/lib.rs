pub mod hacker_news;

use eframe::egui::{self, CentralPanel, ScrollArea, Spinner};
use hacker_news::{HackerNews, StoryCardData};
use hacker_news_api::get_topstories;

impl eframe::App for HackerNews {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        if let Some(rx) = &self.stories_rx {
            if let Ok(news_data) = rx.try_recv() {
                self.stories.push(news_data);
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            if self.stories.is_empty() {
                ui.vertical_centered_justified(|ui| ui.add(Spinner::new()));
            } else {
                ScrollArea::vertical().show(ui, |ui| {
                    self.render_news_cards(ui);
                });
            }
        });
    }
}

pub fn fetch_stories(stories_tx: &mut std::sync::mpsc::Sender<StoryCardData>) {
    if let Ok(res_stories) = get_topstories() {
        for s in res_stories {
            let story = StoryCardData {
                title: s.title,
                url: s.url,
                by: s.by,
                score: s.score,
                time: s.time,
                descendants: s.descendants,
            };
            if let Err(e) = stories_tx.send(story) {
                tracing::error!("error sending story: {}", e);
            }
        }
    } else {
        tracing::error!("error fetching stories");
    }
}
