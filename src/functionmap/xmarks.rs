use maths_rs::Vec3d;
use crate::functionmap::islandservice::convert_wide_name_to_string;
use crate::functionmap::uactor::draw_actor;
use crate::gameservice::GameService;
use crate::gametypes::{FVector, FWorldMapIslandDataCaptureParams, Service, TArray, WideName};
use crate::offsets::offsets;
use crate::viewtypes::{ObjectType, RED, WHITE};

pub fn draw_x_marks(map_actor_address : u64, game : &GameService){
    let (island_pos, island_name) = get_mark_island_location(game, map_actor_address);

    let island_loc_vec = Vec3d::new(island_pos.x as f64, island_pos.y as f64,island_pos.z as f64);
    draw_actor(game, &island_loc_vec, "TargetIsland", WHITE, ObjectType::Text);

    draw_mark_position(map_actor_address, game, island_name);
}

fn get_mark_rotation(torn_map_actor : u64, game : &GameService) -> f32{
    game.get_mem().read(torn_map_actor + offsets::MARKS_ROTATION).unwrap_or_default()
}

fn draw_mark_position(torn_map_actor : u64, game : &GameService, target_island : String){

    let world_map_data = get_mark_island_from_full_name(game, target_island);

    let marks_vec = get_marks_positions(game, torn_map_actor);

    for mark in marks_vec {
        let marks_rot = get_mark_rotation(torn_map_actor, game);
        let mut mark_pos_rot = rotate_point(mark, FVector{x: 0.5, y: 0.5, z: 0.0}, 180.0 + marks_rot, false);
        mark_pos_rot.x -= 0.5;
        mark_pos_rot.y -= 0.5;
        let island_scale = world_map_data.CameraOrthoWidth / 1.041669;
        let offset_pos = FVector{ x: mark_pos_rot.x * island_scale, y: mark_pos_rot.y * island_scale, z: 0.0 };
        let dig_spot_loc = Vec3d::new( (world_map_data.WorldSpaceCameraPosition.x - offset_pos.x) as f64, (world_map_data.WorldSpaceCameraPosition.y - offset_pos.y) as f64, 0.0);

        draw_actor(game, &dig_spot_loc, "X", RED, ObjectType::Text);
    }
}

fn get_marks_positions(game : &dyn Service, torn_map_actor : u64) -> Vec<FVector> {
    let x_marks_s : TArray = game.get_mem().read(torn_map_actor + offsets::MARKS).unwrap_or_default();

    let mut marks_vec = Vec::new();
    for mark_index in 0..x_marks_s.count {
        let mark_pos : FVector = game.get_mem().read(x_marks_s.data_ptr + mark_index as u64 * 0x10).unwrap_or_default();
        marks_vec.push(mark_pos);
    }
    marks_vec
}

fn get_x_map_name(game : &dyn Service, torn_map_actor : u64) -> String{
   let name_ptr : TArray = game.get_mem().read(torn_map_actor + offsets::MAP_TEXTURE_PATH).unwrap_or_default();

    //let island_txt : u64 = game.get_mem().read(name_ptr.data_ptr).unwrap_or_default();

    let island_name : WideName = game.get_mem().read(name_ptr.data_ptr).unwrap_or_default();
    let island_string = convert_wide_name_to_string(island_name.word);

    return island_string;
}

fn get_mark_island_location(game : &GameService, torn_map_actor : u64) -> (FVector, String) {
    let target_island_name = get_x_map_name(game, torn_map_actor);

    let island_data_asset : u64 = game.get_mem().read(game.island_service + offsets::ISLAND_DATA_ASSET).unwrap_or_default();
    let f_island_array : TArray = game.get_mem().read(game.island_service + offsets::ISLAND_ARRAY).unwrap_or_default();

    let island_data_entries : TArray = game.get_mem().read(island_data_asset + offsets::ISLAND_DATA_ENTRIES).unwrap_or_default();

    for island_index in 0..island_data_entries.count {
        let c_island = game.get_mem().read_u64(island_data_entries.data_ptr + (island_index * 0x8) as u64);
        let island_id = game.get_mem().read_i32(c_island + offsets::ISLAND_NAME);
        let island_name = game.get_mem().read_gname(island_id, game.get_gnames()).unwrap_or_default();

        if target_island_name.contains(&island_name) {
            let island_location = get_x_island_location(game, f_island_array, island_name.clone());
            let island_loc_vec = Vec3d::new(island_location.x as f64, island_location.y as f64,island_location.z as f64);
            draw_actor(game, &island_loc_vec, "TargetIsland", WHITE, ObjectType::Text);
            return (island_location, island_name);
        }
    }
    return (FVector::default(), String::from(""));
}

pub fn get_x_island_location(game: &GameService, f_island_array : TArray, target_island_name : String) -> FVector {

    for island_index in 0..f_island_array.count {
        let island_name_id = game.get_mem().read_i32(f_island_array.data_ptr + (island_index * 0x70) as u64); //Size of FIsland Struct

        let island_name = game.get_mem().read_gname(island_name_id, game.get_gnames()).unwrap_or_default();

        if island_name != target_island_name{
            continue;
        }

        let island_location : FVector = game.get_mem().read(f_island_array.data_ptr + (island_index * 0x70) as u64 + offsets::ISLAND_BOUNDS_CENTER).unwrap_or_default();
        return island_location;
    }
    FVector::default()
}

fn get_mark_island_from_full_name(game : &GameService, target_island : String) -> FWorldMapIslandDataCaptureParams {
    let island_data_asset : u64 = game.get_mem().read(game.island_service + offsets::ISLAND_DATA_ASSET).unwrap_or_default();

    let island_data_entries : TArray = game.get_mem().read(island_data_asset + offsets::ISLAND_DATA_ENTRIES).unwrap_or_default();

    for island_index in 0..island_data_entries.count {
        let c_island = game.get_mem().read_u64(island_data_entries.data_ptr + (island_index * 0x8) as u64);
        let island_id = game.get_mem().read_i32(c_island + offsets::ISLAND_NAME);
        let island_name = game.get_mem().read_gname(island_id, game.get_gnames()).unwrap_or_default();

        if target_island == island_name {
            let world_map_data = game.get_mem().read_u64(c_island + offsets::WORLD_MAP_DATA);
            let world_params : FWorldMapIslandDataCaptureParams = game.get_mem().read(world_map_data + offsets::CAPTURE_PARAMS).unwrap_or_default();
            return world_params;
        }
    }
    return FWorldMapIslandDataCaptureParams::default();
}

fn rotate_point(point_to_rotate: FVector, center_point: FVector, mut angle: f32, angle_in_radians: bool) -> FVector {
    if !angle_in_radians {
        angle = angle * (std::f32::consts::PI / 180.0);
    }
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();

    let translated_x = point_to_rotate.x - center_point.x;
    let translated_y = point_to_rotate.y - center_point.y;

    let rotated_x = cos_theta * translated_x - sin_theta * translated_y;
    let rotated_y = sin_theta * translated_x + cos_theta * translated_y;

    FVector {
        x: rotated_x + center_point.x,
        y: rotated_y + center_point.y,
        z: 0.0,
    }
}