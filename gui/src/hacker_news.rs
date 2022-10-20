use eframe::{egui::Context, CreationContext};
use hacker_news_api::Story;

#[derive(Default)]
pub struct HackerNews {
    pub stories: Vec<Story>,
}

impl HackerNews {
    pub fn init(mut self, cc: &CreationContext) -> Self {
        tracing::info!("successfully initialized");
        self
    }

    fn configure_fonts(&mut self, ctx: &Context) {
        todo!()
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        &self.stories.iter().for_each(|story| {
            ui.horizontal(|ui| {
                ui.label(story.title.as_str());
            });
        });
    }
}
