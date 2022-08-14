mod headlines;

use eframe::{epi::App, egui::{CentralPanel, ScrollArea, Vec2, Visuals}, NativeOptions, run_native};
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

        if self.config.dark_mode  {
            ctx.set_visuals(Visuals::dark());
        }else{
            ctx.set_visuals(Visuals::light());
        }

        self.render_config(ctx);

        self.render_top_panel(ctx,frame); 
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
    tracing_subscriber::fmt::init();
    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(600., 650.));
    run_native(Box::new(app),win_options);
}
