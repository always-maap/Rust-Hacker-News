use eframe::epaint::Vec2;
use hacker_news_gui::hacker_news::HackerNews;

fn main() {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .init();

    let hacker_news = HackerNews::default();
    let mut win_option = eframe::NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    eframe::run_native(
        "Hacker News",
        win_option,
        Box::new(|cc| Box::new(hacker_news.init(cc))),
    );
}
