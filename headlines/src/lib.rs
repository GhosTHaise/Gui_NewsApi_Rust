mod headlines;

use std::{thread, sync::mpsc::{self, sync_channel, Sender}};

use eframe::{epi::App, egui::{CentralPanel, ScrollArea,Visuals}};
pub use headlines::{Headlines, render_header, render_footer, NewsCardData};
use newsApi::NewsApi;

use crate::headlines::Msg;

fn fetch_news(api_key: &str,news_tx : &mut std::sync::mpsc::Sender<NewsCardData>) -> () {
    //dbg!(NewsApi::new(&api_key).fetch());
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
}
#[cfg(target_arch = "wasm32")]
async fn fetch_web(api_key: String,news_tx : std::sync::mpsc::Sender<NewsCardData>) -> () {
    if let Ok(response) = NewsApi::new(&api_key).fetch_web().await {
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
}
impl App for Headlines{
    fn setup(
            &mut self,
            ctx: &eframe::egui::Context,
            _frame: &eframe::epi::Frame,
            _storage: Option<&dyn eframe::epi::Storage>,
        ) {
        //println!("start to fetch {:?}",NewsApi::new(&self.config.api_key).fetch());
        let (mut news_tx,news_rx) = mpsc::channel();
        let (app_tx,app_rx) = sync_channel(1);
        let api_key = self.config.api_key.to_string();

        self.app_tx = Some(app_tx);

        self.news_rx = Some(news_rx);    
        
        let api_key_web : String = api_key.clone();
        let news_tx_web : Sender<NewsCardData> = news_tx.clone();    
       
        #[cfg(not(target_arch = "wasm32"))]
        thread::spawn(move || {
            if !api_key.is_empty() {
                fetch_news(&api_key, &mut news_tx);
            }else{
                loop{
                    match app_rx.recv(){
                        Ok(Msg::ApiKeySet(api_key)) => {
                            fetch_news(&api_key, &mut news_tx)
                        }
                        Ok(Msg::Refresh) => {
                            fetch_news(&api_key, &mut news_tx)
                        }
                        Err(e) => {
                            tracing::error!("failed receiving msg : {}",e)
                        }
                    }
                }
            }
    });

        
        //load only on web
        #[cfg(target_arch = "wasm32")]
        gloo_timers::callback::Timeout::new(10,move || {
            wasm_bindgen_futures::spawn_local(async{
                fetch_web(api_key_web,news_tx_web).await;
            });
        }).forget();
        #[cfg(target_arch = "wasm32")]
        gloo_timers::callback::Interval::new(500,move || {
            match app_rx.try_recv(){
                Ok(Msg::ApiKeySet(api_key)) => {
                    wasm_bindgen_futures::spawn_local(fetch_web(api_key.clone(),news_tx.clone()));
                }
                Ok(Msg::Refresh) => {
                    wasm_bindgen_futures::spawn_local(fetch_web(api_key.clone(),news_tx.clone()));
                }
                Err(e) => {
                    tracing::error!("failed receiving msg : {}",e);
                }
            }
        }).forget();

        println!("end to fectch");
        self.configure_fonts(ctx);
    }
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {

        ctx.request_repaint();

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

                if self.articles.is_empty(){
                    ui.vertical_centered_justified(|ui|{
                        ui.heading("Loading ⏳")
                    });
                }else{
                    render_header(ui);
                    ScrollArea::vertical().show(ui, |ui|{
                        self.render_news_cards(ui);
                    });
                    //ui.add_space(20.);
                    render_footer(ctx);
                }
        });
        }   
    }

    fn name(&self) -> &str {
        "Headlines by GhosT v1.0"
    }
}


#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self,prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main_web(canvas_id : &str){
    let headlines = Headlines::new();
    tracing_wasm::set_as_global_default();
    eframe::start_web(canvas_id,Box::new( |cc| Box::new(headlines)));
}