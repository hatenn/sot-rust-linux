use maths_rs::Vec3d;
use maths_rs::vec::vec3d;
use crate::functionmap::islandservice::get_island_location;
use crate::functionmap::uactor::draw_actor;
use crate::gameservice::GameService;
use crate::gametypes::{FVector, FWorldMapIslandDataCaptureParams, Service, TArray, WideName};
use crate::offsets::offsets;
use crate::playerservice::PlayerService;
use crate::viewtypes::{ObjectType, RED, WHITE};

pub fn draw_vault_location(torn_map_actor : u64, game : &GameService){
    let (island_pos, island_name) = get_vault_island_location(game, torn_map_actor);

    let world_map_data = get_vault_island_from_full_name(game, island_name);

    draw_vault_key(island_pos, game, torn_map_actor, world_map_data);
}

fn draw_vault_key(island_pos : FVector, game : &GameService, torn_map_actor : u64, map_data : FWorldMapIslandDataCaptureParams){
    let map_center_x = 0.5;
    let map_center_y = 0.5;

    let map_render_data : TArray = game.get_mem().read(torn_map_actor + offsets::RENDER_DATA).unwrap_or_default();
    let render_data_0 = game.get_mem().read_u64(map_render_data.data_ptr);

    let transforms_array : TArray = game.get_mem().read(render_data_0 + offsets::TRANSFORMS).unwrap_or_default();

    let island_size : FVector = game.get_mem().read(transforms_array.data_ptr + 0x14).unwrap_or_default();

    let render_data_1 = game.get_mem().read_u64(map_render_data.data_ptr + 0x20);
    let transforms_array_rd1 : TArray = game.get_mem().read(render_data_1 + offsets::TRANSFORMS).unwrap_or_default();

    let mut mark_corner_map_location: FVector = game.get_mem().read(transforms_array_rd1.data_ptr + 0x08).unwrap_or_default();

    let mut mark_offset_from_corner: FVector = game.get_mem().read(transforms_array_rd1.data_ptr + 0x14).unwrap_or_default();
    mark_offset_from_corner.x *= 0.5;
    mark_offset_from_corner.y *= 0.5;

    let island_scale = map_data.CameraOrthoWidth / 1.041669;

    let x_mark_offset = FVector {
        x: (mark_corner_map_location.x + mark_offset_from_corner.x) - map_center_x,
        y: (mark_corner_map_location.y + mark_offset_from_corner.y) - map_center_y,
        z : 0.0,
    };

    let mark_map_offset_scaled = FVector {
        x: x_mark_offset.x / island_size.x,
        y: x_mark_offset.y / island_size.y,
        z: 0.0,
    };

    let offset_from_island = FVector {
        x: mark_map_offset_scaled.x * island_scale,
        y: mark_map_offset_scaled.y * island_scale,
        z: 0.0,
    };

    let position = FVector {
        x: island_pos.x + offset_from_island.x,
        y: island_pos.y + offset_from_island.y,
        z: 0.0,
    };

    let island_loc_vec= Vec3d::new(position.x as f64,position.y as f64,position.z as f64);

    draw_actor(game, &island_loc_vec, "X", RED, ObjectType::Text);
}

fn get_vault_map_name(game : &dyn Service, torn_map_actor : u64) -> String{
    let map_render_data : TArray = game.get_mem().read(torn_map_actor + offsets::RENDER_DATA).unwrap_or_default();
    let render_data = game.get_mem().read_u64(map_render_data.data_ptr);
    let render_resource = game.get_mem().read_u64(render_data);

    let island_id = game.get_mem().read_i32(render_resource + 0x28);
    let island_name = game.get_mem().read_gname(island_id, game.get_gnames()).unwrap_or_default();
    return island_name;
}

fn get_vault_island_from_full_name(game : &GameService, target_island : String) -> FWorldMapIslandDataCaptureParams {
    let island_data_asset : u64 = game.get_mem().read(game.island_service + offsets::ISLAND_DATA_ASSET).unwrap_or_default();
    //let f_island_array : TArray = game.get_mem().read(game.island_service + offsets::ISLAND_ARRAY).unwrap_or_default();

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

fn get_vault_island_location(game : &GameService, torn_map_actor : u64) -> (FVector, String) {
    let target_island_name = get_vault_map_name(game, torn_map_actor);

    let island_data_asset : u64 = game.get_mem().read(game.island_service + offsets::ISLAND_DATA_ASSET).unwrap_or_default();
    let f_island_array : TArray = game.get_mem().read(game.island_service + offsets::ISLAND_ARRAY).unwrap_or_default();

    let island_data_entries : TArray = game.get_mem().read(island_data_asset + offsets::ISLAND_DATA_ENTRIES).unwrap_or_default();

    for island_index in 0..island_data_entries.count {
        let c_island = game.get_mem().read_u64(island_data_entries.data_ptr + (island_index * 0x8) as u64);
        let island_id = game.get_mem().read_i32(c_island + offsets::ISLAND_NAME);
        let island_name = game.get_mem().read_gname(island_id, game.get_gnames()).unwrap_or_default();

        let island_text_ptr = game.get_mem().read_u64(c_island + offsets::ISLAND_LOCAL_NAME);
        let island_text = game.get_mem().read_u64(island_text_ptr);
        let island_txt_value : WideName = game.get_mem().read(island_text).unwrap_or_default();

        let island_string = crate::functionmap::islandservice::convert_wide_name_to_string(island_txt_value.word);

        if island_string == target_island_name {
            let island_location = get_v_island_location(game, f_island_array, island_name.clone());
            let island_loc_vec = Vec3d::new(island_location.x as f64, island_location.y as f64,island_location.z as f64);
            draw_actor(game, &island_loc_vec, "TargetIsland", WHITE, ObjectType::Text);
            return (island_location, island_name);
        }
    }
    return (FVector::default(), String::from(""));
}

pub fn get_v_island_location(game: &GameService, f_island_array : TArray, target_island_name : String) -> FVector {

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