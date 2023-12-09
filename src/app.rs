use std::{path::PathBuf, fs::File, io::Write, string, process::Command};

use egui::Context;
use rfd::FileDialog;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    hasFirstRan: bool,
    gamePathSave: String,
    multibuttonChar: String,
    isDev: bool,
    isErrorShown: bool,
    isMSGBoxShown: bool,
    MSGBox_Text: String,
    isMMIv1: bool,
    isMMIv2: bool,
    isxenonMMI: bool,
    isCommunityMMI: bool,
    
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "".to_owned(),
            value: 2.7,
            hasFirstRan: false,
            gamePathSave: "".to_string(),
            multibuttonChar: "...".to_string(),
            isDev: false,
            isErrorShown: false,
            isMSGBoxShown: false,
            MSGBox_Text: "undefined".to_string(),
            isMMIv1: false,
            isMMIv2: false,
            isxenonMMI: false,
            isCommunityMMI: false,
            

        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    /*dont want to save app state
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    } 
    */

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
                    ui.menu_button("Experimental", |ui| {
                        
                            
                        if ui.button("Launch Gorilla Tag").clicked() {
                            if(self.label == ""){
                                self.isMSGBoxShown = true;
                                self.MSGBox_Text = "ERROR: You have not selected a game path".to_string();
                            }else{
                                let output = if cfg!(target_os = "windows") {
                                    Command::new("cmd")
                                        .args(["/C", &mut self.label])
                                        .output()
                                        .expect("failed to execute process")
                                } else {
                                    Command::new("sh")
                                        .arg("-c")
                                        .arg(&mut self.label)
                                        .output()
                                        .expect("failed to execute process")
                                };
                            }
                            
                        }
                        
                        if ui.button("Enable broken mods").clicked() {
                            
                        }
                        if ui.button("Hide manager Gameobject").clicked() {
                            
                        }
                        
                        
                    });
                    ui.menu_button("Mod repos", |ui| {
                        ui.checkbox(&mut self.isMMIv1, "MonkeModInfo (legacy)");
                        ui.checkbox(&mut self.isMMIv2, "MonkeModInfo (2.0)");
                        ui.checkbox(&mut self.isxenonMMI, "RustMMM Database");
                        ui.checkbox(&mut self.isCommunityMMI, "Community Submitted");
                        if(ui.button("Custom Repos").clicked()){
                            self.isMSGBoxShown = true;
                            self.MSGBox_Text = "Not Implemented".to_string();
                        }
                    });
                    
                    ui.menu_button("Help", |ui| {
                        if ui.button("help").clicked() {
                            self.isErrorShown = !self.isErrorShown;
                        }
                    });
                    ui.add_space(16.0);
                    
                }

                
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.horizontal(|ui| {
                ui.heading("RustMMM");
                
                ui.label("v0.0.1");
                ui.label("Game path: ");
                ui.text_edit_singleline(&mut self.label);
                if(self.label != ""){
                    if(ui.button("✓").clicked()){
                        println!("not implemented");
                    }
                }else{
                    if(ui.button("...").clicked()){
                        let files = FileDialog::new()
                        .add_filter("Gorilla Tag", &["exe"])
                        
                        .set_directory("/")
                        .pick_file();
                        println!("{:?}", files);
                        let tempPath: Option<PathBuf> = files;
                        if let Some(path_buf) = tempPath {
                            let path_string: String = path_buf.to_string_lossy().into_owned();
                            let moved_string = path_string.clone();
                            self.label = path_string;
                            
                            let bytes: &[u8] = moved_string.as_bytes();
                            
                        }
                        
                        
                    }
                }
                if(self.isErrorShown){
                    egui::Window::new("Help").show(ctx, |ui| {
                        
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.heading("Selecting game folder");
                            
                            ui.label("To select a game folder, click the '...' button, as shown below");
                            ui.add_space(16.0);
                            ui.label("TODO: PLACE AN IMAGE HERE");
                            ui.add_space(16.0);
                            ui.heading("Adding custom repos:");
                            
                            ui.separator();
                            ui.add_space(16.0);
                            ui.label("If you want to add Custom repositories to widen your selection of mods, follow these steps:");
                            ui.add_space(16.0);
                            ui.label("- Open the 'Repos' context menu");
                            ui.add_space(16.0);
                            ui.label("- Select 'Add Custom Repos'");
                            ui.add_space(16.0);
                            ui.label("- TODO: PLACE AN IMAGE HERE");
                            ui.add_space(16.0);
                            ui.label("- In the textbox, Paste in your Repository URL, and click 'Add', Or drag in one of the trusted repos");
                            ui.add_space(16.0);
                            ui.label("- TODO: PLACE AN IMGE HERE")
                            
                        });
                        ui.separator();
                        if(ui.button("Ok").clicked()){
                            self.isErrorShown = false;
                        }
                    });
                }
                if(self.isMSGBoxShown){
                    egui::Window::new("Information").show(ctx, |ui| {
                        
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label(&self.MSGBox_Text);
                            
                        });
                        ui.separator();
                        if(ui.button("Ok").clicked()){
                            self.isMSGBoxShown = false;
                        }
                    });
                }
                
                
            });
            

            

            

            ui.separator();

            

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                
                egui::warn_if_debug_build(ui);
                ui.label("Selected Mods: 0");
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn saveDir(path: &[u8]) -> std::io::Result<()> {
    

    let mut file = File::create("GameDirPath.ini")?;
    file.write_all(path)?;
    Ok(())
}


