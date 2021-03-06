mod actions;
mod assets;
mod audio;
mod events;
mod loading;
mod map;
mod menu;
mod player;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::events::EventsPlugin;
use crate::loading::LoadingPlugin;
use crate::map::{Coordinate, MapPlugin, Maps};
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;

use bevy::prelude::*;

pub struct GamePlugin;

const STAGE: &str = "game";

#[derive(Clone)]
pub enum AppState {
    Loading,
    Menu,
    InGame,
    RetryGame,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::BLACK))
            .add_resource(State::new(AppState::Loading))
            .add_resource(GameState::default())
            .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
            .add_plugin(MapPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EventsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(LoadingPlugin)
            .on_state_enter(STAGE, AppState::RetryGame, switch_to_game.system());
    }
}

pub struct GameState {
    pub health: usize,
    pub score: usize,
    pub enemy_health: i32,
    pub current_map: Maps,
    pub can_walk: bool,
    pub player_spawn: Coordinate,
    pub talking_to: Option<Coordinate>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            health: 20,
            score: 0,
            enemy_health: 1,
            can_walk: true,
            current_map: Maps::Mall,
            player_spawn: Coordinate { x: 144., y: 288. },
            talking_to: None,
        }
    }
}

fn switch_to_game(mut state: ResMut<State<AppState>>) {
    state.set_next(AppState::InGame).unwrap();
}
