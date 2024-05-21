mod view;
mod viewtypes;
mod memory;
mod playermodel;
mod gameservice;
mod settings;
mod offsets;
mod gametypes;

use std::sync::{Arc, mpsc, RwLock};
use std::thread;
use crate::apiserver::start_server;
use crate::gameservice::GameService;
use crate::gametypes::Service;
use crate::levelservice::LevelService;
use crate::memory::Memory;
use crate::playermodel::LocalPlayer;
use crate::playerservice::PlayerService;
use crate::settings::settings::{GAME_NAME};
use crate::view::View;
use crate::window::get_window_info;

mod functionmap;
mod window;
mod levelservice;
mod playerservice;
mod shiptracker;
mod apiserver;


fn main() {
    println!("Starting...");

    let (tx, rx) = mpsc::channel();

    thread::spawn(||{
        start_server(tx);
    });

    let (w_width, w_height, w_pos_x, w_pos_y) = get_window_info("Sea of Thieves").unwrap();

    //Creates an overlay instance
    let mut render_list = View::new(w_width, w_height, w_pos_x, w_pos_y);

    let mem = Memory::new(GAME_NAME);

    //Get local player info
    let local_player_instance = LocalPlayer::new(&mem, w_width, w_height);
    let local_player = Arc::new(RwLock::new(local_player_instance));

    //Create service to iterate only persistent level
    let render_list_clone = Arc::clone(&render_list);
    let local_player_clone = Arc::clone(&local_player);
    let mut game_service = GameService::new(mem.clone(), render_list_clone, local_player_clone);

    //Create service to iterate levels except persistent
    let render_list_clone = Arc::clone(&render_list);
    let local_player_clone = Arc::clone(&local_player);
    let mut level_service = LevelService::new(mem.clone(), render_list_clone, local_player_clone);

    //Create service to update our Localplayer data
    let render_list_clone = Arc::clone(&render_list);
    let local_player_clone = Arc::clone(&local_player);
    let mut local_player_service = PlayerService::new(rx, mem.clone(), render_list_clone, local_player_clone);

    //Services that read actors and send info to a render list
    let game_service_th = thread::spawn(move || {
        game_service.run();
    });

    let level_service_th = thread::spawn(move || {
        //level_service.run();
    });

    let player_service_thread = thread::spawn(move || {
        local_player_service.run();
    });

    game_service_th.join();
    level_service_th.join();
    player_service_thread.join();
}