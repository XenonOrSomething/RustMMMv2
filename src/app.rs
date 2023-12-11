use std::process::exit;
use std::{path::PathBuf, fs::File, io::Write, string, process::Command, };
use std::path;
use downloader::Downloader;
use egui::{Context, Checkbox, Widget};
use rfd::FileDialog;
use std::thread::spawn;
use std::thread;
use std::fs;
use std::io::{prelude::*, Cursor};
use zip_extract;
//use egui_file::FileDialog;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/// use downloader::Downloader;
 // if we add new fields, give them default values when deserializing old state
struct SimpleReporterPrivate {
    last_update: std::time::Instant,
    max_progress: Option<u64>,
    message: String,
}
struct SimpleReporter {
    private: std::sync::Mutex<Option<SimpleReporterPrivate>>,
}

impl SimpleReporter {
    #[cfg(not(feature = "tui"))]
    fn create() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            private: std::sync::Mutex::new(None),
        })
    }
}

impl downloader::progress::Reporter for SimpleReporter {
    fn setup(&self, max_progress: Option<u64>, message: &str) {
        let private = SimpleReporterPrivate {
            last_update: std::time::Instant::now(),
            max_progress,
            message: message.to_owned(),
        };

        let mut guard = self.private.lock().unwrap();
        *guard = Some(private);
    }

    fn progress(&self, current: u64) {
        if let Some(p) = self.private.lock().unwrap().as_mut() {
            let max_bytes = match p.max_progress {
                Some(bytes) => format!("{:?}", bytes),
                None => "{unknown}".to_owned(),
            };
            if p.last_update.elapsed().as_millis() >= 1000 {
                println!(
                    "test file: {} of {} bytes. [{}]",
                    current, max_bytes, p.message
                );
                p.last_update = std::time::Instant::now();
            }
        }
    }

    fn set_message(&self, message: &str) {
        println!("test file: Message changed to: {}", message);
    }

    fn done(&self) {
        let mut guard = self.private.lock().unwrap();
        *guard = None;
        println!("test file: [DONE]");
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
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
    addedModsamt: i32,
    checkboxes: Vec<bool>,
    notFirstFrame: bool,
    isUtilla: bool,
    mods: Vec<String>,

    //checkboxList: Vec<Checkbox>,
    //opened_file: Option<PathBuf>,
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
            addedModsamt: 0,
            checkboxes: vec![false, false],
            notFirstFrame: false,
            isUtilla: false,
            mods: Vec::new(),
           // checkboxList: Vec::new(),
            //opened_file: Option::PathBuf::new(),
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
                        fs::create_dir_all("C:\\RustMMMTemp");
                        let mut downloader = Downloader::builder()
                        .download_folder(std::path::Path::new("C:\\RustMMMTemp"))
                        .parallel_requests(1)
                        .build()
                        .unwrap();

                        let dl = downloader::Download::new("https://github.com/BepInEx/BepInEx/releases/download/v5.4.22/BepInEx_x64_5.4.22.0.zip");

                        #[cfg(not(feature = "tui"))]
                        let dl = dl.progress(SimpleReporter::create());

                        

            let result = downloader.download(&[dl]).unwrap();

            for r in result {
                match r {
                    Err(e) => println!("Error: {}", e.to_string()),
                    Ok(s) => println!("Success: {}", &s),
                };
            }
            
            if(self.isUtilla){
                fs::create_dir_all("C:\\RustMMMTemp");
                        let mut downloader = Downloader::builder()
                        .download_folder(std::path::Path::new("C:\\RustMMMTemp"))
                        .parallel_requests(1)
                        .build()
                        .unwrap();

                        let dl = downloader::Download::new("https://github.com/legoandmars/Utilla/releases/download/v1.6.10/Utilla.zip");

                        #[cfg(not(feature = "tui"))]
                        let dl = dl.progress(SimpleReporter::create());

                        

            let result = downloader.download(&[dl]).unwrap();

            for r in result {

                match r {
                    Err(e) => println!("Error: {}", e.to_string()),
                    Ok(s) => println!("Success: {}", &s),
                };
            }
            let bytes = "C:\\RustMMMTemp\\BepInEx_x64_5.4.22.0.zip".as_bytes().to_vec();
            let archive: Vec<u8> = bytes;
            let target_dir = PathBuf::from(&self.label);
            let ex1 = zip_extract::extract(Cursor::new(archive), &target_dir, true);
            println!("extracted");
            let bytes = "C:\\RustMMMTemp\\Utilla.zip".as_bytes().to_vec();
            let archive: Vec<u8> = bytes;
            let target_dir = PathBuf::from(&self.label);
            let extractedzip2 = zip_extract::extract(Cursor::new(archive), &target_dir, true);
            println!("extracted");
            }
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
                        ui.label("Help not implemented yet. dosent pull from any repos yet");
                        /* 
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
                            ui.label("- In the textbox, Paste in your Repository URL, and click 'Add', add one of the trusted repos");
                            ui.add_space(16.0);
                            ui.label("- TODO: PLACE AN IMGE HERE")
                            
                            
                        });
                        */
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
            if(!self.notFirstFrame){
                self.notFirstFrame = true;
                self.mods.append(&mut vec!["BepInEx".to_string()]);
                self.mods.append(&mut vec!["Utilla".to_string()]);
                self.mods.append(&mut vec!["Bark".to_string()]);
                
                
            } 
            
                println!("{:?}", self.mods);
                println!("checkboxes");
                for (i, el) in self.mods.iter().enumerate() {
                    self.addedModsamt += 1;
                    println!("Mods in database: {}", el);
                    println!("The current index is {}", i);
                    
        
                 
                    self.checkboxes.append(&mut vec![false]);
                    ui.checkbox(&mut self.checkboxes[i], el);
                    println!("added");
                    
                
            }
            //checkboxes(ui);
            
                
                    
                    
                    
                
            
            
       
            
            

            ui.separator();

            self.checkboxes.append(&mut vec![true]);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                
                egui::warn_if_debug_build(ui);
                ui.label("Selected Mods: 0");
            });
        });
    }

    

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}


fn saveDir(path: &[u8]) -> std::io::Result<()> {
    

    let mut file = File::create("GameDirPath.ini")?;
    file.write_all(path)?;
    Ok(())
}



