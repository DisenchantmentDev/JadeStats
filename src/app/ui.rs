use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use analyzer_core::player::Player;
use eframe::egui::CentralPanel;
//use eframe::egui::Window;
use egui::Context;
use egui::ScrollArea;
use egui::SidePanel;
use egui::TextStyle;
use egui::{FontFamily, FontId};

use crate::app::app_error::AppError;
pub mod home;
pub mod loading;
pub mod player_interface;
pub mod profiles_list;
pub mod stats_display;
pub mod stats_page;

pub struct App {
    username: String, //player username as they would input, ie WhaleMilk#PHUD
    region: Regions,
    state: State,
    graph_dimensions: (usize, usize), //columns / rows
    has_loaded: bool,
    indexed_players: Vec<String>,
    loaded_player: Player,
    player: Arc<Mutex<LoadingState<Player>>>,
    loading_started: bool,
    root_dir: PathBuf,
    err: Option<AppError>, // Error field that tracks if there is an error thrown by player interface
}

pub enum LoadingState<T> {
    Dormant,
    Loading,
    Loaded(T),
    Error(AppError),
}

pub struct SharedPlayer {
    pub state: LoadingState<Player>,
}

#[derive(Clone)]
pub struct PlayerLoadCtx {
    pub username: String,
    pub region: Regions,
    pub root_dir: PathBuf,
    pub indexed_players: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[allow(dead_code, clippy::allow_attributes)]
enum State {
    #[default]
    Home,
    Stats,
    Profile,
    Loading,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[allow(clippy::upper_case_acronyms, clippy::allow_attributes)]
enum GraphType {
    #[default]
    GD15,
    CSM,
    DPM,
    KP,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
#[allow(clippy::upper_case_acronyms, clippy::allow_attributes)]
pub enum Regions {
    NA,
    EUW,
    EUNE,
    KR,
    CN,
    #[default]
    NONE,
}

impl fmt::Display for GraphType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::GD15 => write!(f, "GD@15"),
            Self::CSM => write!(f, "CS/M"),
            Self::DPM => write!(f, "D/M"),
            Self::KP => write!(f, "KP%"),
        }
    }
}
impl Default for App {
    fn default() -> Self {
        let dir = project_root::get_project_root().unwrap_or_default();
        Self {
            username: String::default(),
            region: Regions::NA,
            state: State::Home,
            graph_dimensions: (2, 2),
            has_loaded: false,
            indexed_players: PlayerLoadCtx::read_indexed_players(&dir)
                .expect("Could not read indexed players on initialization"),
            loaded_player: Player::default(),
            player: Arc::new(Mutex::new(LoadingState::Dormant)),
            loading_started: false,
            root_dir: dir,
            err: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.start_loading().expect("Failed loading");
        self.ui(ctx);
    }
}

impl App {
    fn ui(&mut self, ctx: &egui::Context) {
        set_style(ctx);
        self.top_bar(ctx);
        //CentralPanel::default().show(ctx, |vi| vi.heading("JadeStats"));
        self.draw_side_panel(ctx);
        self.draw_central_panel(ctx);
        ctx.request_repaint();

        //Window::new("TestWindow").show(ctx, |ui| {
        //    ui.label("Test Window");
        //});
    }

    fn draw_central_panel(&mut self, ctx: &egui::Context) {
        /* drawing the central Panel w/ Graphs */
        CentralPanel::default().show(ctx, |ui| match self.state {
            State::Home => {
                self.player = Arc::new(Mutex::new(LoadingState::Dormant));
                self.loaded_player = Player::default();
                self.home_central_panel(ui);
            }

            State::Stats => {
                if self.err.is_none() {
                    self.draw_stats(ui);
                } else {
                    self.state = State::Home;
                }
                //graph grid
                //self.draw_stat_graph_strip(ui, sin, self.graph_dimensions);
            }
            State::Profile => {
                let _temp = 100;
            }
            State::Loading => {
                self.display_loading(ui);
            }
        });
    }
}

fn set_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(30.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Small, FontId::new(14.0, FontFamily::Monospace)),
    ]
    .into();
    ctx.set_style(style);
}
