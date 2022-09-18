
use std::{borrow::Cow, sync::mpsc::{Receiver, SyncSender}, option};
use eframe::{egui::{FontDefinitions, FontFamily, Color32, Label, Layout, Hyperlink, Separator, Ui, TopBottomPanel, Context, TextStyle, self, Button, Window, CentralPanel, FontData, RichText}};
use serde::{Serialize,Deserialize};

const PADDING : f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 250, 250);
const BLACK : Color32 =  Color32::from_rgb(0, 0, 0) ;

pub enum Msg {
    ApiKeySet(String),
    Refresh
}

#[derive(Serialize,Deserialize)]
pub struct HeadlinesConfig {
   pub  dark_mode: bool,
   pub api_key: String
}

impl Default for HeadlinesConfig{
    fn default() -> Self {
        Self { 
            dark_mode: Default::default(),
            api_key : String::new()
         }
    }
}

pub struct Headlines{
    pub articles : Vec<NewsCardData>,
    pub config : HeadlinesConfig,
    pub api_key_initialized: bool,
    pub news_rx : Option<Receiver<NewsCardData>>,
    pub app_tx : Option<SyncSender<Msg>>
}
impl Headlines {
    pub fn new() -> Headlines {
        let config : HeadlinesConfig = confy::load("headlines").unwrap_or_default();
        Headlines {
            api_key_initialized : !config.api_key.is_empty(),
            articles : vec![],
            config,
            news_rx : None,
            app_tx : None
         }
    }
    pub fn configure_fonts(&self,ctx: &eframe::egui::Context) -> () {
        // create font def object
        let mut font_def = FontDefinitions::default();
        // then we we'll load the font
        font_def.font_data.insert("Spartan".to_string(), 
            FontData::from_static(include_bytes!("../../Spartan-VariableFont_wght.ttf")));
        // then set the sowe of different text styles

        //deprecated
        //font_def.family_and_size.insert(eframe::egui::TextStyle::Heading,(FontFamily::Proportional,35.));
        //.family_and_size.insert(eframe::egui::TextStyle::Body,(FontFamily::Proportional,16.));
        
        font_def.families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0,"Spartan".to_string());
        // load font using context object
        ctx.set_fonts(font_def)   ;                      

    }

    

    pub fn render_news_cards(&self,ui : &mut eframe::egui::Ui) -> () {
        for a in &self.articles{
            //Add padding top
            ui.add_space(PADDING);
            //print!("title : {}",a.title);
            //render title
            let title = format!("🔹 {}",a.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            }else{
                ui.colored_label(BLACK, title);
            }
            
            //render desc
            ui.add_space(PADDING);
            let desc = Label::new(RichText::new(&a.desc).text_style(eframe::egui::TextStyle::Button));
            ui.add(desc);
            
            //render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui|{
                ui.add(Hyperlink::from_label_and_url("read more 🔜",&a.url));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
    pub(crate) fn render_top_panel(&mut self,ctx : &Context,frame : &eframe::epi::Frame) -> () {
        //define a topBottomPanel wodget
        TopBottomPanel::top("top_panel").show(ctx, |ui|{
            //then two layout widgets
            ui.add_space(10.);
            egui::menu::bar(ui,|ui|{
                //render the logo on the left
                ui.with_layout(Layout::left_to_right(), |ui|{
                    ui.add(Label::new(RichText::new("📓").text_style(egui::TextStyle::Heading)));
                });
                //button controller on the right
                ui.with_layout(Layout::right_to_left(), |ui|{
                    
                    if !cfg!(target_arch = "wasm32") {
                        let close_btn = ui.add(Button::new(RichText::new("❌").text_style(egui::TextStyle::Body)));
                        if close_btn.clicked(){
                            frame.quit();
                        }
                    }

                    let refresh_btn = ui.add(Button::new(RichText::new("🔄").text_style(egui::TextStyle::Body)));
                    if refresh_btn.clicked() {
                        if let Some(tx)  = &self.app_tx{
                            self.articles.clear();
                            tx.send(Msg::Refresh);
                        }
                    }
                    

                    let theme_btn = ui.add(Button::new(RichText::new({
                        if self.config.dark_mode {
                            "🌞"
                        }else{
                            "🌙"
                        }
                    })));
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

    pub fn preload_articles(&mut self){
        if let Some(rx) = &self.news_rx {
            match rx.try_recv() {
                Ok(news_data) => {
                    self.articles.push(news_data)
                },
                Err(e) => {
                    tracing::warn!("Error receiving msg : {}",e);
                }
            }
        }
    }
    
    pub fn render_config(&mut self,ctx:&Context){
        CentralPanel::default().show(ctx, |ui|{
            Window::new("Configuration").show(ctx,|ui|{
                ui.label("Enter your API_KEY for newsapi.org");
                let text_input = ui.text_edit_singleline(&mut self.config.api_key);
                tracing::error!("{}",&self.config.api_key);
                ui.label("If you havn-t registered for the API_KEY,head over to");
            if text_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter){
                if let Err(e) = confy::store("headlines", HeadlinesConfig {
                        dark_mode: self.config.dark_mode,
                        api_key: self.config.api_key.to_string()
                }){
                     tracing::error!("failed saving app state : {}",e);
                }
                self.api_key_initialized = true;
    
                if let Some(tx) =  &self.app_tx {
                    tx.send(Msg::ApiKeySet(self.config.api_key.to_string()));
                }
    
                tracing::error!("api key set");
            }
                ui.hyperlink("https://newsapi.org");
            });
        });
    }
}

pub fn render_footer(ctx : &Context) -> () {
    TopBottomPanel::bottom("footer").show(ctx, |ui|{
        ui.vertical_centered(|ui|{
            ui.add_space(10.);
            //add api source
            ui.add(
                Label::new(RichText::new("API source : newsapi.org")
                    .small()
                    .text_style(TextStyle::Monospace)
            )); 
                    
            //add link to egui framwork
            ui.add(Hyperlink::from_label_and_url(
                "Made with egui",
                 "https://github.com/emilk/egui")     
            );
            //put github link to source code
            ui.add(Hyperlink::from_label_and_url(
                "GhosTHaise/Gui_NewsApi_Rust",
                 "https://github.com/GhosTHaise/Gui_NewsApi_Rust")
            );
            
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
pub struct NewsCardData{
    pub title : String,
    pub desc : String,
    pub url : String
}