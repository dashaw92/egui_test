mod model;

use eframe::egui::{self, Color32};
use model::*;
use rainworld_level::RWLevel;

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
    _project: Option<RWLevel>,
}

impl Default for LevelEditorApp {
    fn default() -> Self {
        Self {
            active: ScreenType::Home,
            _project: None,
        }
    }
}

impl eframe::App for LevelEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Menu buttons for navigating the various editors
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for id in SealedPair::NAMES {
                    let mut button = egui::widgets::Button::new(id.name());

                    if id.screen() == self.active {
                        button = button
                            .fill(Color32::GRAY);
                    }

                    if ui.add(button).clicked() && id.screen() != self.active {
                        self.active = id.screen();
                    }
                }
            });
        });
        
        //Delegate actual content rendering to each screen type
        egui::CentralPanel::default().show(ctx, |ui| self.active.render(ui, &mut self._project));
    }
}
