use std::{fmt::format, borrow::Cow};

use eframe::{egui::{CentralPanel, ScrollArea, Vec2, FontDefinitions, FontFamily},epi::App,run_native, NativeOptions};
struct Headlines{
    articles : Vec<NewsCardData>,
}
impl Headlines {
    fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsCardData{
            title: format!("Title{}",a),
            desc: format!("desc{}",a),
            url: format!("https://example.com/{}",a)
        });
        Headlines { 
            articles : Vec::from_iter(iter)
         }
    }
    fn configure_fonts(&self,ctx: &eframe::egui::CtxRef) -> () {
        // create font def object
        let mut font_def = FontDefinitions::default();
        // then we we'll load the font
        font_def.font_data.insert("Spartan".to_string(), Cow::Borrowed(include_bytes!("../../Spartan-VariableFont_wght.ttf")));
        // then set the sowe of different text styles
        font_def.family_and_size.insert(eframe::egui::TextStyle::Heading,(FontFamily::Proportional,35.));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Body,(FontFamily::Proportional,20.));
        
        font_def.fonts_for_family.get_mut(&FontFamily::Proportional)
                                 .unwrap()
                                 .insert(0,"Spartan".to_string());
        // load font using context object
        ctx.set_fonts(font_def)   ;                      

    }
}

struct NewsCardData{
    title : String,
    desc : String,
    url : String
}

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
        CentralPanel::default().show(ctx, |ui|{
            ScrollArea::auto_sized().show(ui, |ui|{
                for a in &self.articles{
                    ui.label(&a.title);
                    ui.label(&a.desc);
                    ui.label(&a.url);
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Headlines by GhosT v1.0"
    }
}
fn main() -> () {
    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(600., 760.));
    run_native(Box::new(app),win_options);
}
