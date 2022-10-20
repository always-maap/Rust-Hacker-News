pub mod hacker_news;

use eframe::egui::{self, CentralPanel, ScrollArea, Spinner};
use hacker_news::HackerNews;

impl eframe::App for HackerNews {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
