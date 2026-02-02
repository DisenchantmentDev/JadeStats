use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use analyzer_core::player::Player;
use eframe::egui::CentralPanel;
use egui::Context;
use egui::TextStyle;
use egui::{FontFamily, FontId};

use crate::app::app_error::AppError;
pub mod error_window;
pub mod home;
pub mod loading;
pub mod player_interface;
pub mod profiles_list;
pub mod stats_display;
pub mod stats_page;

/// App structure that holds all relevant information for application UI. Holds display user info,
/// window state, etc. which are edited throughout the lifetime of the program.
pub struct App {
    ///Username of the player the user is looking for. Should follow the format {Username}#{Tag}.
    ///For example: WhaleMilk#PHUD.
    username: String,

    ///Server Region selected by the player. When passed to the backend, this is tacked onto the
    ///username with another #. Ex: WhaleMilk#PHUD#NA
    region: Regions,

    /// The state of the UI. Tracks what is displayed when, mostly in central plane. See State
    /// enumu
    state: State,

    /// How many graphs are displayed in the Stats page. Default is 2x2. This currently does not
    /// change.
    graph_dimensions: (usize, usize),

    ///Boolean value tracking whether or not a player has been attempted to load or not. If this is
    ///true, but the program is in the state, then it knows to check for error and draw it only
    ///once. Boolean is mostly for redrawing purposes.
    has_loaded: bool,

    ///A list of players stored in ``./assets/indexed_players.json``. This is an effective list of
    ///players who's full profile json files are saved to disk in ``./assets/players/``
    indexed_players: Vec<String>,

    ///A boolean to check if the program needs to re-read ``./assets/indexed_players.json`` due to an
    ///update of some kind.
    update_index_players: bool,

    ///The player that is loaded into memory for display on the stats page. This can either be
    ///loaded from the stored json file or via the ``analyzer_core`` backend.
    loaded_player: Player,

    ///Smart pointer for async loading of a player from ``analyzer_core``. This allows for the UI to
    ///display a loading screen while ``analyzer_core`` waits for and parses large data sets from
    ///Riot's API.
    player: Arc<Mutex<LoadingState<Player>>>,

    /// A boolean value that checks if the program needs to start loading or not. If this flag is
    /// set, the program spawns a thread for async loading of player from the backend.
    loading_started: bool,

    /// Path to the root of project directory for finding assets folder easily.
    root_dir: PathBuf,

    ///Tracks possible errors from either the front end itself or from the backend.
    err: Option<AppError>, // Error field that tracks if there is an error thrown by player interface
}

/// Enum which tracks the state of loading a player from ``analyzer_core``. Once loaded, we load a
/// player in, and that is passed to the ``loaded_player`` field.
pub enum LoadingState<T> {
    Dormant,
    Loading,
    Loaded(T),
    Error(AppError),
}

/// Struct for passing data to ``analyzer_core`` asynchronously. This is needed to avoid passing
/// &self to somewhere else while inside a closure.
#[derive(Clone)]
pub struct PlayerLoadCtx {
    pub username: String,
    pub region: Regions,
    pub root_dir: PathBuf,
    pub indexed_players: Vec<String>,
}

/// Enum for describing window state. Each state has a different associated page for drawing on
/// central plane and elsewhere.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[allow(dead_code, clippy::allow_attributes)]
enum State {
    #[default]
    Home,
    Stats,
    Profile,
    Loading,
}

/// A description of different graph displays that can be displayed on the Stats Page.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[allow(clippy::upper_case_acronyms, clippy::allow_attributes)]
enum GraphType {
    #[default]
    GD15,
    CSM,
    DPM,
    KP,
}

/// The different available servers that can be pulled from
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
            update_index_players: false,
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
        self.draw_side_panel(ctx);
        self.draw_central_panel(ctx);
        ctx.request_repaint();

        //Window::new("TestWindow").show(ctx, |ui| {
        //    ui.label("Test Window");
        //});
    }

    fn draw_central_panel(&mut self, ctx: &egui::Context) {
        /* drawing the central Panel w/ Graphs */
        if self.update_index_players {
            let temp = self.indexed_players.clone();
            self.indexed_players =
                PlayerLoadCtx::read_indexed_players(&self.root_dir).unwrap_or(temp);
        }
        CentralPanel::default().show(ctx, |ui| match self.state {
            State::Home => {
                self.player = Arc::new(Mutex::new(LoadingState::Dormant));
                self.loaded_player = Player::default();
                self.home_central_panel(ctx, ui);
            }

            State::Stats => {
                if self.err.is_none() {
                    self.draw_stats(ui);
                } else {
                    self.state = State::Home;
                }
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
