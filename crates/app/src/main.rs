#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::*;
use eframe::egui;
use egui::*;
use egui_file_dialog::{FileDialog};
use pixelation;
#[derive(Default)]
struct MyApp {
    file_dialog: FileDialog,
    selected_file: Option<PathBuf>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            // Create a new file dialog object
            file_dialog: FileDialog::new(),
            selected_file: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Select file").clicked() {
                // Open the file dialog to select a file.
                self.file_dialog.select_file();
            }
            
            ui.label(format!("Selected file: {:?}", self.selected_file
            ));
                      
            // Update the dialog and check if the user selected a file
            if let Some(path) = self.file_dialog.update(ctx).selected() {
                    self.selected_file = Some(path.to_path_buf());
               
                    // window 1
                    ui.horizontal(|ui| { 
                    ui.add( 
                        egui::Image::new(format!("{:?}",path))
                        .max_width(400.0)
                        .max_height(400.0));
                    

                    
                    //pixelation
                    let image = image::open(path);
                    match image::open(path) {
                        Ok(image) => {
                            image.save(r"./1.png").unwrap();
                            let vec_colors = pixelation::dominant_colors(image.clone()); 
                            let sqauare = pixelation::generate_squares(10.0, image::open(path.clone()).unwrap());
                            let line = pixelation::line(image.clone(), 10);
                            let img_out = pixelation::paint_coordinats(sqauare.clone(), vec_colors.clone(), image.clone(), line.clone());
                                img_out.clone().save(r".\out_1.png").unwrap();
                            // window 2
                            ui.add( 
                                egui::Image::new(r".\out_1.png")
                                .max_width(400.0)
                                .max_height(400.0));
                            
                        }
                        Err(err) => {
                            
                        }
                    }
                });     
            }
            
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Pixelation",
        eframe::NativeOptions::default(),
        Box::new(|ctx| Box::new(MyApp::new(ctx))),
    )
}