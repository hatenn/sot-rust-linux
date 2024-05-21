use maths_rs::{Vec2d, Vec3d};
use crate::gametypes::{ACannon, FLocation, FVector, Service};
use crate::functionmap::cannon::do_cannon_prediction;
use crate::functionmap::uactor::get_replicated_movement;
use crate::offsets::offsets;
use crate::shiptracker::ShipTracker;
use crate::viewtypes::{Actor, BROWN, ObjectType, RED, WHITE};

const DISPLAY_NAME: &str = "Ship";

pub fn draw_ship(actor_address : u64, game: &mut dyn Service, raw_name : String){

    let top_location_offset : f32 = 1000.0;
    let root_comp = game.get_mem().read_u64(actor_address + offsets::ROOT_COMPONENT);
    let loc_rot : FLocation = game.get_mem().read(root_comp + offsets::RELATIVE_LOCATION).unwrap();
    let mut target_loc = Vec3d::new(loc_rot.location.x as f64, loc_rot.location.y as f64, loc_rot.location.z as f64);


    let local_player_read = game.get_lplayer().read().unwrap();
    let target_distance = local_player_read.distance_to(&target_loc);
    drop(local_player_read);

    if target_distance < 5000.0 { //My current ship
        let ship_movement = get_replicated_movement(game, actor_address);
        match game.get_ship_tracker(){
            Some(ship_tracker) => {
                ship_tracker.my_ship_movement = ship_movement;
            }
            None => {}
        }
    }

    if target_distance < 60000.0 && target_distance > 5000.0 {
        do_cannon_prediction(actor_address, game);
    }


    target_loc.z += top_location_offset as f64;
    let mut screen_pos = Vec2d::default();
    let local_player_read = game.get_lplayer().read().unwrap();
    if local_player_read.world_to_screen(&target_loc, &mut screen_pos) {
        let my_actor = Actor::new(screen_pos.x, screen_pos.y, 15.0, 15.0, String::from((target_distance / 100.0).round().to_string()), 20, ObjectType::Text, WHITE);
        game.get_draw_list().lock().unwrap().push(my_actor);
    }
}




