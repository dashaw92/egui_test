mod model;

use eframe::egui;
use model::*;

fn main() {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
                .with_inner_size([800.0, 600.0])
                .with_min_inner_size([800.0, 600.0]),
        vsync: false,
        ..Default::default()
    };

    eframe::run_native(
        "Level Editor", 
        opts, 
        Box::new(|_| Box::<LevelEditorApp>::default())
    ).expect("Whoops!");
}

struct LevelEditorApp {
    active: ScreenType,
}

impl Default for LevelEditorApp {
    fn default() -> Self {
        Self {
            active: ScreenType::Home,
        }
    }
}

impl eframe::App for LevelEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Menu buttons for navigating the various editors
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for id in SealedPair::NAMES {
                    if ui.button(id.name()).clicked() {
                        self.active = id.screen();
                    }
                }
            });
        });
        
        //Delegate actual content rendering to each screen type
        egui::CentralPanel::default().show(ctx, |ui| self.active.render(ui));
    }
}
