
use std::{borrow::Cow};
use eframe::{egui::{FontDefinitions, FontFamily, Color32, Label, Layout, Hyperlink, Separator, Ui, TopBottomPanel, CtxRef, TextStyle, self, Button}};
use serde::{Serialize,Deserialize};

const PADDING : f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 250, 250);
const BLACK : Color32 =  Color32::from_rgb(0, 0, 0) ;

#[derive(Serialize,Deserialize)]
pub struct HeadlinesConfig {
   pub  dark_mode: bool
}

impl Default for HeadlinesConfig{
    fn default() -> Self {
        Self { 
            dark_mode: Default::default()
         }
    }
}

impl HeadlinesConfig {
    fn new() -> Self {
        Self { 
            dark_mode: true 
        }
    }
}

pub struct Headlines{
    articles : Vec<NewsCardData>,
    pub config : HeadlinesConfig
}
impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsCardData{
            title: format!("Title{}",a),
            desc: format!("desc{}",a),
            url: format!("https://example.com/{}",a)
        });

        let config : HeadlinesConfig = confy::load("headlines").unwrap_or_default();
        Headlines { 
            articles : Vec::from_iter(iter),
            config 
         }
    }
    pub fn configure_fonts(&self,ctx: &eframe::egui::CtxRef) -> () {
        // create font def object
        let mut font_def = FontDefinitions::default();
        // then we we'll load the font
        font_def.font_data.insert("Spartan".to_string(), Cow::Borrowed(include_bytes!("../../Spartan-VariableFont_wght.ttf")));
        // then set the sowe of different text styles
        font_def.family_and_size.insert(eframe::egui::TextStyle::Heading,(FontFamily::Proportional,35.));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Body,(FontFamily::Proportional,16.));
        
        font_def.fonts_for_family.get_mut(&FontFamily::Proportional)
                                 .unwrap()
                                 .insert(0,"Spartan".to_string());
        // load font using context object
        ctx.set_fonts(font_def)   ;                      

    }
    pub fn render_news_cards(&self,ui : &mut eframe::egui::Ui) -> () {
        for a in &self.articles{
            //Add padding top
            ui.add_space(PADDING);
            //render title
            let title = format!("🔹 {}",a.title);
            if self.config.dark_mode {
                ui.colored_label(BLACK, title);
            }else{
                ui.colored_label(WHITE, title);
            }
            
            //render desc
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);
            
            //render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui|{
                ui.add(Hyperlink::new(&a.url).text("read more 🔜"))
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
    pub(crate) fn render_top_panel(&mut self,ctx : &CtxRef,frame : &mut eframe::epi::Frame<'_>) -> () {
        //define a topBottomPanel wodget
        TopBottomPanel::top("top_panel").show(ctx, |ui|{
            //then two layout widgets
            ui.add_space(10.);
            egui::menu::bar(ui,|ui|{
                //render the logo on the left
                ui.with_layout(Layout::left_to_right(), |ui|{
                    ui.add(Label::new("📓").text_style(egui::TextStyle::Heading));
                });
                //button controller on the right
                ui.with_layout(Layout::right_to_left(), |ui|{
                    let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                    if(close_btn.clicked()){
                        frame.quit();
                    }
                    let refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                    let theme_btn = ui.add(Button::new({
                        if self.config.dark_mode {
                            "🌞"
                        }else{
                            "🌙"
                        }
                    }).text_style(egui::TextStyle::Body));
                    if(theme_btn.clicked()){
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                }); 
                //padding before after the pannel
                
            });
            ui.add_space(15.);
        });
        //add a menu bar
        
        
        
      
    }

}

pub fn render_footer(ctx : &CtxRef) -> () {
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

pub fn render_header(ui : &mut Ui) -> () {
    ui.vertical_centered(|ui|{
            ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}
struct NewsCardData{
    title : String,
    desc : String,
    url : String
}