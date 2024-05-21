use crate::gametypes::Service;
use crate::offsets::offsets;

pub fn do_instant_ladder(game : &dyn Service, actor_address : u64){
    let climbing_component = game.get_mem().read_u64(actor_address + offsets::CLIMBING_COMPONENT);
    let height : f32 = 9999.0;
    game.get_mem().write(climbing_component + offsets::SERVER_HEIGHT, height).unwrap_or_default()
}