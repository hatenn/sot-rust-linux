use graphics::types::Color;
use maths_rs::{Vec2d, Vec3d};
use crate::gameservice::GameService;
use crate::gametypes::{FLocation, FRepMovement, FVector, Service};
use crate::offsets::offsets;
use crate::viewtypes::{Actor, ObjectType, WHITE};

//These are functions that are available for all AActor structs

pub fn get_actor_root_comp(game : &dyn Service, actor_address : u64) -> (Vec3d, Vec3d){
    let root_comp = game.get_mem().read_u64(actor_address + offsets::ROOT_COMPONENT);
    let loc_rot : FLocation = game.get_mem().read(root_comp + offsets::RELATIVE_LOCATION).unwrap();
    let actor_location = Vec3d::new(loc_rot.location.x as f64, loc_rot.location.y as f64, loc_rot.location.z as f64);
    let actor_rotation = Vec3d::new(loc_rot.rotation.pitch as f64, loc_rot.rotation.yaw as f64, loc_rot.rotation.roll as f64);
    (actor_location, actor_rotation)
}

pub fn draw_actor(game : &dyn Service, target_location : &Vec3d, display_name : &str, sel_colour : Color, obj_type : ObjectType){
    let local_player_read = game.get_lplayer().read().unwrap();
    let mut screen_pos = Vec2d::default();
    if local_player_read.world_to_screen(&target_location, &mut screen_pos) {
        let my_actor = Actor::new(screen_pos.x, screen_pos.y, 15.0, 15.0, String::from(display_name), 18, obj_type, sel_colour);
        game.get_draw_list().lock().unwrap().push(my_actor);
    }
}

pub fn get_distance_between(location_one : &Vec3d, location_two : &Vec3d) -> f64{
    let dx = location_one.x - location_two.x;
    let dy = location_one.y - location_two.y;
    let dz = location_one.z - location_two.z;
    f64::sqrt(dx * dx + dy * dy + dz * dz)
}

pub fn get_attached_actor(game : &dyn Service, actor_address : u64) -> u64{
    game.get_mem().read_u64(actor_address + 0x88)
}

pub fn get_replicated_movement(game : &dyn Service, actor_address : u64) -> FRepMovement {
    game.get_mem().read(actor_address + offsets::FREP_MOVEMENT).unwrap_or_default()
}