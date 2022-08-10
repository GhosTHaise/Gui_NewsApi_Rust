use std::{fmt::format, borrow::Cow};

const PADDING : f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 250, 250);

use eframe::{egui::{CentralPanel, ScrollArea, Vec2, FontDefinitions, FontFamily, Color32, Label, Layout, Hyperlink, Separator, Ui, TopBottomPanel, CtxRef, TextStyle},epi::App,run_native, NativeOptions};
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
    fn render_news_cards(&self,ui : &mut eframe::egui::Ui) -> () {
        for a in &self.articles{
            //Add padding top
            ui.add_space(PADDING);
            //render title
            let title = format!("‣ {}",a.title);
            ui.colored_label(WHITE, title);
            
            //render desc
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);
            
            //render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui|{
                ui.add(Hyperlink::new(&a.url).text("read more ⇾"))
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
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

fn render_footer(ctx : &CtxRef) -> () {
    TopBottomPanel::bottom("footer").show(ctx, |ui|{
        ui.vertical_centered(|ui|{
            ui.add_space(10.);
            //add api source
            ui.add(Label::new("API source : newsapi.org")
                .text_color(Color32::from_rgb(160,10,0))
                .monospace());
            //add link to egui framwork
            ui.add(Hyperlink::new("https://github.com/emilk/egui")
                    .text("Made with egui")
                    .text_style(TextStyle::Monospace));
            //put github link to source code
            ui.add(Hyperlink::new("https://github.com/GhosTHaise/Gui_NewsApi_Rust")
                    .text("GhosTHaise/Gui_NewsApi_Rust")
                    .text_style(TextStyle::Monospace));
            
            ui.add_space(10.)
        });
    });
}

fn render_header(ui : &mut Ui) -> () {
    ui.vertical_centered(|ui|{
            ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}
fn main() -> () {
    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(600., 650.));
    run_native(Box::new(app),win_options);
}
