use eframe::egui::Ui;
use rainworld_level::RWLevel;
use rfd::FileDialog;

//Each screen type is a unique view available in the app
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ScreenType {
    //Overview of the project
    Home,
    //Edit the room's geometry
    Geometry,
    //Paint geometry with art
    Tiles,
    //Add effects to the room
    Effects,
    //Modify the room's lighting
    Light,
    //Place props
    Props,
    //Change room settings (size, medium, etc)
    Settings,
}

//Used as a compile time map from strings to screen types
pub struct SealedPair(&'static str, ScreenType);

impl SealedPair {
    //Associate button text to screen types for the UI
    pub const NAMES: [SealedPair; 7] = [
        SealedPair("Home", ScreenType::Home),
        SealedPair("Geometry", ScreenType::Geometry),
        SealedPair("Tiles", ScreenType::Tiles),
        SealedPair("Effects", ScreenType::Effects),
        SealedPair("Light", ScreenType::Light),
        SealedPair("Props", ScreenType::Props),
        SealedPair("Settings", ScreenType::Settings)
    ];

    pub const fn name(&self) -> &'static str {
        self.0
    }

    pub const fn screen(&self) -> ScreenType {
        self.1
    }
}

type Project<'a> = &'a mut Option<RWLevel>;

impl ScreenType {
    //Get the associated rendering function for any screen type and proxy the call to it
    pub fn render(&self, ui: &mut Ui, project: Project) {
        use ScreenType::*;

        let render_fn = match self {
            Home => homescreen,
            Geometry => geometry_editor,
            Tiles => tile_editor,
            Effects => effects_editor,
            Light => lights_editor,
            Props => props_editor,
            Settings => settings,
        };

        render_fn(ui, project)
    }
}

fn homescreen(ui: &mut Ui, project: Project) {    
    if ui.button("Open Project").clicked() {
        *project = FileDialog::new()
            .add_filter("Level Editor Projects", &["txt"])
            .add_filter("All files", &["*"])
            .set_title("Open level editor project")
            .pick_file()
            .and_then(RWLevel::load);
    }
}

fn geometry_editor(ui: &mut Ui, _project: Project) {
    ui.label("Geometry");
}

fn tile_editor(ui: &mut Ui, _project: Project) {
    ui.label("Tiles");
}

fn effects_editor(ui: &mut Ui, _project: Project) {
    ui.label("Effects");
}

fn lights_editor(ui: &mut Ui, _project: Project) {
    ui.label("Lights");
}

fn props_editor(ui: &mut Ui, _project: Project) {
    ui.label("Props");
}

fn settings(ui: &mut Ui, _project: Project) {
    ui.label("Settings");
}