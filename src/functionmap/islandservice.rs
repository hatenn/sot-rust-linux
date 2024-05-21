use maths_rs::Vec3d;
use crate::functionmap::uactor::draw_actor;
use crate::gametypes::{FVector, Service, TArray, WideName};
use crate::offsets::offsets;
use crate::viewtypes::{ObjectType, WHITE};

pub fn draw_island(island_service_actor : u64, game: &mut dyn Service, target_island_name : String){
    let island_data_asset : u64 = game.get_mem().read(island_service_actor + offsets::ISLAND_DATA_ASSET).unwrap_or_default();
    let f_island_array : TArray = game.get_mem().read(island_service_actor + offsets::ISLAND_ARRAY).unwrap_or_default();

    let island_data_entries : TArray = game.get_mem().read(island_data_asset + offsets::ISLAND_DATA_ENTRIES).unwrap_or_default();

    for island_index in 0..island_data_entries.count {
        let c_island = game.get_mem().read_u64(island_data_entries.data_ptr + (island_index * 0x8) as u64);
        let island_id = game.get_mem().read_i32(c_island + offsets::ISLAND_NAME);
        let island_name = game.get_mem().read_gname(island_id, game.get_gnames()).unwrap_or_default();

        let island_text_ptr = game.get_mem().read_u64(c_island + offsets::ISLAND_LOCAL_NAME);
        let island_text = game.get_mem().read_u64(island_text_ptr);
        let island_txt_value : WideName = game.get_mem().read(island_text).unwrap_or_default();

        let island_string = convert_wide_name_to_string(island_txt_value.word);

        if island_string == target_island_name {
            let island_location = get_island_location(game, f_island_array, island_name);
            let island_loc_vec = Vec3d::new(island_location.x as f64, island_location.y as f64,island_location.z as f64);
            draw_actor(game, &island_loc_vec, "TargetIsland", WHITE, ObjectType::Text);
            break;
        }
    }
}

pub fn get_island_location(game: &mut dyn Service, f_island_array : TArray, target_island_name : String) -> FVector {

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

pub fn convert_wide_name_to_string(word: [u16; 64]) -> String {
    let mut result = String::new();
    for &code_unit in &word {
        if code_unit == 0 { break; } // Assuming zero termination
        if let Some(ch) = char::from_u32(code_unit as u32) {
            result.push(ch);
        } else {
            result.push('\u{FFFD}');
        }
    }
    result
}
