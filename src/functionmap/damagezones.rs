use maths_rs::{Vec2d, Vec3d};
use crate::gameservice::GameService;
use crate::gametypes::{FLocation, FVector, Service};
use crate::offsets::offsets;
use crate::viewtypes::{Actor, ObjectType, WHITE};

pub fn draw_damage_zones(actor_address : u64, game: &mut dyn Service, raw_name : String){
    let local_player_read = game.get_lplayer().read().unwrap();
    let root_comp = game.get_mem().read_u64(actor_address + offsets::ROOT_COMPONENT);
    let loc_rot : FLocation = game.get_mem().read(root_comp + offsets::RELATIVE_LOCATION).unwrap();
    let target_loc = Vec3d::new(loc_rot.location.x as f64, loc_rot.location.y as f64, loc_rot.location.z as f64);

    if local_player_read.distance_to(&target_loc) > 50000.0 {
        return;
    }

    let damage_level : i32 = game.get_mem().read_i32(actor_address + offsets::DAMAGE_LEVEL);

    if damage_level <= 0 {
        return;
    }

    let mut screen_pos = Vec2d::default();
    if local_player_read.world_to_screen(&target_loc, &mut screen_pos) {
        let my_actor = Actor::new(screen_pos.x, screen_pos.y, 15.0, 15.0, String::from("H"), 14, ObjectType::Text, WHITE);
        game.get_draw_list().lock().unwrap().push(my_actor);
    }

    ()
}