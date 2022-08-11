mod headlines;

use eframe::{epi::App, egui::{CentralPanel, ScrollArea, Vec2}, NativeOptions, run_native};
use headlines::{Headlines, render_header, render_footer};

impl App for Headlines{
    fn setup(
            &mut self,
            ctx: &eframe::egui::CtxRef,
            _frame: &mut eframe::epi::Frame<'_>,
            _storage: Option<&dyn eframe::epi::Storage>,
        ) {
        self.configure_fonts(ctx);
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        self.render_top_panel();
        CentralPanel::default().show(ctx, |ui|{
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui|{
                self.render_news_cards(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "Headlines by GhosT v1.0"
    }
}


fn main() -> () {
    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(600., 650.));
    run_native(Box::new(app),win_options);
}
