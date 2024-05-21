use std::sync::{Arc, Mutex};
use maths_rs::{Vec2d, Vec3d};
use crate::playermodel::LocalPlayer;
use crate::gameservice::GameService;
use crate::gametypes::{FCameraCacheEntry, FLocation, FRepMovement, FVector, Service};
use crate::memory::Memory;
use crate::functionmap::uactor::get_replicated_movement;
use crate::offsets::offsets;
use crate::offsets::offsets::{CHARACTER_MOVEMENT_COMP, FREP_MOVEMENT};
use crate::viewtypes::{Actor, ObjectType, RED};

pub fn draw_player(actor_address : u64, game: &mut dyn Service, raw_name : String){
    let local_player_read = game.get_lplayer().read().unwrap();

    if actor_address == local_player_read.player_pawn{
        return;
    }

    let root_comp = game.get_mem().read_u64(actor_address + offsets::ROOT_COMPONENT);
    let loc_rot : FLocation = game.get_mem().read(root_comp + offsets::RELATIVE_LOCATION).unwrap();
    let head_target_loc = Vec3d::new(loc_rot.location.x as f64, loc_rot.location.y as f64, (loc_rot.location.z + 110.0) as f64);
    let feet_target_loc = Vec3d::new(loc_rot.location.x as f64, loc_rot.location.y as f64, (loc_rot.location.z - 110.0) as f64);

    let mut head_screen_pos = Vec2d::default();
    if local_player_read.world_to_screen(&head_target_loc, &mut head_screen_pos) {
        let mut feet_screen_pos = Vec2d::default();
        if local_player_read.world_to_screen(&feet_target_loc, &mut feet_screen_pos) {
            let height = feet_screen_pos.y - head_screen_pos.y;
            let width = height * 0.65;

            let my_actor = Actor::new(head_screen_pos.x, head_screen_pos.y, width, height, String::from(""), 1, ObjectType::Box, RED);
            game.get_draw_list().lock().unwrap().push(my_actor);
        }

    }
}

pub fn get_player_linear_velocity(game : &dyn Service, player_address : u64) -> Option<FVector>{
    match get_player_standing_actor(game, player_address){
        Some(actor_address) => {
            let replicated_movement : FRepMovement = get_replicated_movement(game, actor_address);
            return Some(replicated_movement.linear_velocity);
        },
        None => None,
    }
}

pub fn get_player_standing_actor(game : &dyn Service, player_address : u64) -> Option<u64>{
    let movement_base = game.get_mem().read_u64(player_address + offsets::BASED_MOVEMENT);

    if(movement_base == 0){
        return None;
    }

    let mut attached_parent_comp = game.get_mem().read_u64(movement_base + offsets::ATTACH_PARENT);

    let child_actor = game.get_mem().read_u64(attached_parent_comp + offsets::CHILD_ACTOR);
    let mut parent_actor = game.get_mem().read_u64(child_actor + offsets::PARENT_COMPONENT);

    loop{
       let  next_parent_actor = game.get_mem().read_u64(parent_actor + offsets::PARENT_COMPONENT);
        if next_parent_actor == 0 || next_parent_actor == parent_actor{
            break;
        }
        parent_actor = next_parent_actor;
    }
    return Some(parent_actor);
}

pub fn get_interactive_actor(game : &dyn Service, actor_address : u64) -> u64{
    let char_interaction_comp = game.get_mem().read_u64(actor_address + offsets::UCHAR_INTERACTION_COM);
    let interactable_area : u64 = game.get_mem().read_u64(char_interaction_comp + offsets::CURRENT_INTERACTABLE);
    game.get_mem().read_u64(interactable_area + offsets::INTERACTABLE_PARENT_ACTOR)
}

pub fn extend_interaction_range(game : &dyn Service, actor_address : u64){
    let char_interaction_comp = game.get_mem().read_u64(actor_address + offsets::UCHAR_INTERACTION_COM);
   game.get_mem().write(char_interaction_comp + offsets::BOX_EXTENT, FVector{x: 0.0, y: 1800.00, z: 6800.0}).unwrap_or_default();
}

pub fn reset_interaction_range(game : &dyn Service, actor_address : u64){
    let char_interaction_comp = game.get_mem().read_u64(actor_address + offsets::UCHAR_INTERACTION_COM);
    game.get_mem().write(char_interaction_comp + offsets::BOX_EXTENT, FVector{x: 75.0, y: 18.00, z: 68.0}).unwrap_or_default();
}

pub fn set_locked_fov(game : &dyn Service, player_controller_addr  : u64, target_fov : f32){
    let camera_manager = game.get_mem().read_u64(player_controller_addr + offsets::CAMERA_MANAGER);
    game.get_mem().write(camera_manager + offsets::DEFAULT_FOV + 4, target_fov).unwrap_or_default();
}

pub fn set_max_simulations(game : &dyn Service, character_movement_component  : u64, simulation_num : i32,){
    game.get_mem().write(character_movement_component + offsets::MAX_SIMULATION_ITER, simulation_num).unwrap_or_default();
}

pub fn set_movement_mode(game : &dyn Service, character_movement_component  : u64, movement_mode : u8,){
    game.get_mem().write(character_movement_component + offsets::MOVEMENT_MODE, movement_mode).unwrap_or_default();
}

pub fn instant_ladder(){

}