#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::{config::Config, database};
use eframe::egui;
use egui::TextureHandle;
use egui_file_dialog::FileDialog;
use image::GenericImageView;
use pixelation;

use std::{path::*, sync::Arc};

#[allow(dead_code)]
struct MyApp {
    file_dialog: FileDialog,
    selected_file: Option<PathBuf>,
    db: Arc<database::DB>,
}

impl MyApp {
    pub fn new(db: Arc<database::DB>) -> Self {
        Self {
            file_dialog: FileDialog::new(),
            selected_file: None,
            db,
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

            ui.label(format!("Selected file: {:?}", self.selected_file));

            // Update the dialog and check if the user selected a file
            if let Some(path) = self.file_dialog.update(ctx).selected() {
                self.selected_file = Some(path.to_path_buf());

                // window 1
                // horizontal image
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        match image::open(&path) {
                            Ok(image) => {
                                println!("{:?}", image);
                                // Image
                                ui.label("Image in");
                            }
                            Err(error) => {
                                eprintln!("Ошибка при загрузке изображения: {:?}", error);
                                ui.label("Ошибка при загрузке изображения");
                            }
                        }
                    });
                    //pixelation
                    match image::open(path) {
                        Ok(image) => {
                            image.save(r"./1.png").unwrap();
                            let vec_colors = pixelation::dominant_colors(image.clone());
                            let sqauare =
                                pixelation::generate_squares(10.0, image::open(path).unwrap());
                            let line = pixelation::line(image.clone(), 10);
                            let img_out = pixelation::paint_coordinats(
                                sqauare.clone(),
                                vec_colors.clone(),
                                image.clone(),
                                line.clone(),
                            );
                            img_out.clone().save(r".\out_1.png").unwrap();
                            // window 2
                            ui.vertical(|ui| {
                                ui.add(
                                    egui::Image::new(r".\out_1.png")
                                        .max_width(400.0)
                                        .max_height(400.0),
                                );
                                ui.label("Image out");
                            });
                        }
                        Err(_err) => {}
                    }
                });
                // button db
                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("push db").clicked() {
                            //     let config = Arc::new(Config::load().unwrap());
                            // let db = database::connect(&config.database).await.unwrap();
                            // Open the file dialog to select a file.
                            // self.file_dialog.select_file();

                            //  models::Image::create( &image_name, vec![], &db).await?;
                        }
                        if ui.button("push db").clicked() {
                            //     let config = Arc::new(Config::load().unwrap());
                            // let db = database::connect(&config.database).await.unwrap();
                            // Open the file dialog to select a file.
                            // self.file_dialog.select_file();

                            //  models::Image::create( &image_name, vec![], &db).await?;
                        }
                        if ui.button("push db").clicked() {
                            //     let config = Arc::new(Config::load().unwrap());
                            // let db = database::connect(&config.database).await.unwrap();
                            // Open the file dialog to select a file.
                            // self.file_dialog.select_file();

                            //  models::Image::create( &image_name, vec![], &db).await?;
                        }
                    });
                });
            }
        });
    }
}

#[tokio::main]
async fn main() -> eframe::Result<(), anyhow::Error> {
    let config = Arc::new(Config::load()?);
    let db = Arc::new(database::connect(&config.database).await?);
    database::migrate(&db).await?;
    // let img_out = image::open(r"out_1.png");
    // let image_name = "img_out".to_string();
    // let image_name_1 = "img_out_1".to_string();
    // let mut decode = pixelation::decode_image(r"C:\Project\app_pixel\out_1.png");
    // println!("{:?}", decode);

    // models::Image::create( &image_name, vec![], &db).await?;
    // models::Image::update( 2,&image_name_1, &vec.clone(), &db).await?;
    // models::Image::delete( &db, 3).await?;

    let run_result = eframe::run_native(
        "Pixelation",
        eframe::NativeOptions::default(),
        Box::new(move |_ctx| Box::new(MyApp::new(db.clone()))),
    );

    if let Err(e) = run_result {
        // Логирование ошибки
        eprintln!("Failed to run eframe application: {:?}", e);
    }

    Ok(())
}
