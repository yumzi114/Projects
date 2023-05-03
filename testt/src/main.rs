
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod window_frame;
use eframe::egui;
use std::path::{Path, PathBuf};
use egui::{epaint::text::TextWrapping, *};
fn main()->Result<(),eframe::Error>{
    env_logger::init();
    let option = eframe::NativeOptions{
        decorated:false,
        transparent:true,
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
        let temp = self.sysname.clone();
        let name = [temp," : ".to_string()].join("");
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
            
        })
    }
}
