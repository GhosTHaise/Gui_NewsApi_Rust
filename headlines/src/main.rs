mod headlines;

use std::{thread, sync::mpsc};

use eframe::{epi::App, egui::{CentralPanel, ScrollArea, Vec2, Visuals}, NativeOptions, run_native};
use headlines::{Headlines, render_header, render_footer, NewsCardData};
use newsApi::NewsApi;

fn fetch_news(api_key: &str,articles: &mut Vec<NewsCardData>) -> () {
    
}
impl App for Headlines{
    fn setup(
            &mut self,
            ctx: &eframe::egui::CtxRef,
            _frame: &mut eframe::epi::Frame<'_>,
            _storage: Option<&dyn eframe::epi::Storage>,
        ) {
        //println!("start to fetch {:?}",NewsApi::new(&self.config.api_key).fetch());
        let (news_tx,news_rx) = mpsc::channel();
        
        let api_key = self.config.api_key.to_string();
        self.news_rx = Some(news_rx);    

        thread::spawn(move || {
            if let Ok(response) = NewsApi::new(&api_key).fetch(){
                let response_articles = response.articles();
                for a in response_articles.iter(){
                    println!("{}",a.title().to_string());
                    let news = NewsCardData{
                        title : a.title().to_string(),
                        url: a.url().to_string(),
                        //desc : a.desc().map(|s)| s.to_string()).unwrap_or("...".to_string); 
                        desc : a.desc().to_string()
                    };
                    if let Err(e) = news_tx.send(news) {
                        tracing::error!("Error sending news data : {}",e);
                    }
            }
        }else{
            println!("unable to fecth api");
        }
    });

        println!("end to fectch");
        self.configure_fonts(ctx);
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {

        if self.config.dark_mode  {
            ctx.set_visuals(Visuals::dark());
        }else{
            ctx.set_visuals(Visuals::light());
        }

        if !self.api_key_initialized{
            self.render_config(ctx);
        }else{
            self.preload_articles();
            self.render_top_panel(ctx,frame); 
            CentralPanel::default().show(ctx, |ui|{
                render_header(ui);
                ScrollArea::auto_sized().show(ui, |ui|{
                    self.render_news_cards(ui);
                });
                //ui.add_space(20.);
                render_footer(ctx);
        });
        }   
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
