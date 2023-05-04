
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod window_frame;
use eframe::egui;
use std::{path::{Path, PathBuf}, ops::Add};
use egui::{RichText,Color32,collapsing_header::CollapsingState,InnerResponse,Ui,Response};

use egui_extras::{Size, StripBuilder};
fn main()->Result<(),eframe::Error>{
    env_logger::init();
    let option = eframe::NativeOptions{
        decorated:false,
        transparent:true,
        resizable:true,
        
        min_window_size: Some(egui::vec2(400.1, 100.0)),
        initial_window_size: Some(egui::vec2(400.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My Test App", 
        option,
        Box::new(|_cc|Box::<MyApp>::default()),
    )
}
struct MyInfo{
    list : Vec<System>,
}
struct System{
    sysname:String,
    used:bool,
    path:String,
}
impl System{
    fn new(filepath:&str, sysname:&str)->System{
        System{
            sysname:sysname.to_string(),
            used:Path::new(filepath).is_file(),
            path:filepath.to_string(),
        }
    }
    fn menu(&self)->String{
        let name = self.sysname.clone().add(" : ");
        // let name = [temp," : ".to_string()].join("");
        name
    }
}
impl MyInfo{
    fn new()->MyInfo{
        MyInfo { 
            list:vec![
                System::new("/etc/systemd/system.conf","system.conf"),
                System::new("/etc/pacman.conf","pacman"),
                System::new("/bin/yay","yay"),
                System::new("/etc/modprobe.d/vfio.conf","vfio")
                ]
        }
    }
    
}

#[derive(Default)]
struct  MyApp{
}
impl  eframe::App for MyApp{
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        window_frame::custom_window_frame(ctx, frame, "My Test App",|ui|{
            let my_system = MyInfo::new();
            ui.heading("Check System Files");
            ui.horizontal_wrapped(|ui|{
                for i in my_system.list {
                    ui.label(i.menu().as_str());
                    if i.used {
                        ui.label(RichText::new("Used").color(Color32::from_rgb(110, 255, 110)));
                        let btn = ui.small_button("📋")
                            .on_hover_text("copy path");
                        if btn.clicked(){
                            ui.output_mut(|o| o.copied_text = i.path.into());
                        }
                    }else {
                        ui.label(RichText::new("Undefined").color(Color32::from_rgb(244, 4, 4)));
                    }
                }
            });
            let cmd = "cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585";
            ui.horizontal(|ui| {
                ui.monospace(cmd);
                if ui.small_button("📋").clicked() {
                    ui.output_mut(|o| o.copied_text = cmd.into());
                }
            });
            ui.separator();
            ui.heading("Show Today News Headlines");
            let mut news_view = collaps_head("news",ui);
            let news_header_res = collaps_head_respone(ui,&mut news_view,"show!");
            news_view.show_body_indented(&news_header_res.response, ui, |ui| ui.label("Body"));
            ui.separator();
            ui.heading("Postgresql DB View and Migration");
            let mut db_view = collaps_head("dbv",ui);
            let db_header_res = collaps_head_respone(ui,&mut db_view,"show!");
            db_view.show_body_indented(&db_header_res.response, ui, |ui| ui.label("Body"));
            ui.separator();
            ui.heading("stream video view");
            let mut stream_view = collaps_head("stream",ui);
            let stream_header_res = collaps_head_respone(ui,&mut stream_view,"show!");
            stream_view.show_body_indented(&stream_header_res.response, ui, |ui| ui.label("Body"));
            ui.separator();
            
        })
    }
}
fn circle_icon(ui: &mut Ui, openness: f32, response: &Response) {
    let stroke = ui.style().interact(&response).fg_stroke;
    let radius = egui::lerp(6.0..=8.0, openness);
    ui.painter().circle_filled(response.rect.center(), radius, stroke.color);
}
fn collaps_head (id:&str, ui: &mut Ui)->CollapsingState{
    let head = CollapsingState::load_with_default_open(
        ui.ctx(),
        ui.make_persistent_id(id),
        false,
    );
    head
}
fn collaps_head_respone(ui: &mut egui::Ui,statename:&mut CollapsingState,showname:&str)->InnerResponse<()>{
    let respone =ui.horizontal(|ui| {
        ui.label(showname);
        statename.show_toggle_button(ui, circle_icon);
    });
    respone
}