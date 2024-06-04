use eframe::egui::{Response, Ui};

#[derive(Copy, Clone, Debug)]
pub enum ScreenType {
    Home,
    Geometry,
    Tiles,
    Effects,
    Light,
    Props,
    Settings,
}

pub struct SealedId {
    name: &'static str,
    assoc_type: ScreenType,
}

impl SealedId {
    pub const NAMES: [SealedId; 7] = [
        SealedId { name: "Home", assoc_type: ScreenType::Home },
        SealedId{ name: "Geometry", assoc_type: ScreenType::Geometry  },
        SealedId{ name: "Tiles", assoc_type: ScreenType::Tiles  },
        SealedId{ name: "Effects", assoc_type: ScreenType::Effects  },
        SealedId{ name: "Light", assoc_type: ScreenType::Light  },
        SealedId{ name: "Props", assoc_type: ScreenType::Props  },
        SealedId{ name: "Settings", assoc_type: ScreenType::Settings  }
    ];

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn screen(&self) -> ScreenType {
        self.assoc_type
    }
}

impl ScreenType {

    pub fn render(&self, ui: &mut Ui) -> Response {
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

        render_fn(ui)
    }
}

fn homescreen(ui: &mut Ui) -> Response {
    ui.label("Home")
}

fn geometry_editor(ui: &mut Ui) -> Response {
    ui.label("Geometry")
}

fn tile_editor(ui: &mut Ui) -> Response {
    ui.label("Tiles")
}

fn effects_editor(ui: &mut Ui) -> Response {
    ui.label("Effects")
}

fn lights_editor(ui: &mut Ui) -> Response {
    ui.label("Lights")
}

fn props_editor(ui: &mut Ui) -> Response {
    ui.label("Props")
}

fn settings(ui: &mut Ui) -> Response {
    ui.label("Settings")
}