use std::sync::{Arc, Mutex};
use maths_rs::{Vec2d, Vec3d};
use crate::playermodel::LocalPlayer;
use crate::gameservice::GameService;
use crate::gametypes::{FLocation, Service};
use crate::memory::Memory;
use crate::offsets::offsets;
use crate::viewtypes::{Actor, ObjectType, RED, WHITE};

const DISPLAY_NAME: &str = "Treasure";

pub fn draw_treasure(actor_address : u64, game: &mut dyn Service, raw_name : String){

    let root_comp = game.get_mem().read_u64(actor_address + offsets::ROOT_COMPONENT);
    let loc_rot : FLocation = game.get_mem().read(root_comp + offsets::RELATIVE_LOCATION).unwrap();
    let target_loc = Vec3d::new(loc_rot.location.x as f64, loc_rot.location.y as f64, loc_rot.location.z as f64);

    let local_player_read = game.get_lplayer().read().unwrap();
    let mut screen_pos = Vec2d::default();
    if local_player_read.world_to_screen(&target_loc, &mut screen_pos) {
        let my_actor = Actor::new(screen_pos.x, screen_pos.y, 15.0, 15.0, String::from(raw_name), 8, ObjectType::Text, WHITE);
        game.get_draw_list().lock().unwrap().push(my_actor);
    }

    ()
}

pub fn draw_treasure_name(actor_address : u64, game: &mut dyn Service, name : &str){

    let root_comp = game.get_mem().read_u64(actor_address + offsets::ROOT_COMPONENT);
    let loc_rot : FLocation = game.get_mem().read(root_comp + offsets::RELATIVE_LOCATION).unwrap();
    let target_loc = Vec3d::new(loc_rot.location.x as f64, loc_rot.location.y as f64, loc_rot.location.z as f64);

    let local_player_read = game.get_lplayer().read().unwrap();
    let mut screen_pos = Vec2d::default();
    if local_player_read.world_to_screen(&target_loc, &mut screen_pos) {
        let my_actor = Actor::new(screen_pos.x, screen_pos.y, 15.0, 15.0, String::from(name), 8, ObjectType::Text, WHITE);
        game.get_draw_list().lock().unwrap().push(my_actor);
    }

    ()
}