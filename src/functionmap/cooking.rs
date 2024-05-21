use graphics::color::WHITE;
use crate::functionmap::uactor::{draw_actor, get_actor_root_comp};
use crate::gametypes::Service;
use crate::offsets::offsets::{COOKER_COMP, COOKING_STATE, VISIBLE_COOKED_EXTENT};
use crate::viewtypes::ObjectType;

fn get_cooking_state(game : &dyn Service, actor_address : u64) -> Option<&str> {

    let cooker_comp = game.get_mem().read_u64(actor_address + COOKER_COMP);

    let is_cooking : bool = game.get_mem().read(cooker_comp + COOKING_STATE).unwrap_or_default();

    if !is_cooking{
        return None;
    }

    let visible_cooked = game.get_mem().read_f32(cooker_comp + COOKING_STATE + VISIBLE_COOKED_EXTENT);

    if (visible_cooked < 1.0) {
        return Some("NotCooked");
    }
    else if (visible_cooked <= 2.0) {
        return Some("Cooked");
    }
    return Some("Burned");
}

pub fn draw_cooking_state(actor_address : u64, game : &mut dyn Service, actor_name : String){

    let (actor_loc, actor_rot) = get_actor_root_comp(game, actor_address);

    if game.get_lplayer().read().unwrap().distance_to(&actor_loc) > 4500.0 {
        return;
    }

    match get_cooking_state(game, actor_address){
        Some(str) => {draw_actor(game, &actor_loc, str, WHITE, ObjectType::Text)}
        None => {}
    }
}