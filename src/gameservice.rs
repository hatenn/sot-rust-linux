use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use crate::functionmap::cooking::draw_cooking_state;
use crate::functionmap::damagezones::draw_damage_zones;
use crate::playermodel::LocalPlayer;
use crate::gametypes::{Service, TArray};
use crate::memory::Memory;
use crate::functionmap::FunctionMap;
use crate::functionmap::islandservice::draw_island;
use crate::functionmap::treasure::draw_treasure_name;
use crate::functionmap::uactor::get_actor_root_comp;
use crate::functionmap::xmarks::draw_x_marks;
use crate::levelservice::LevelService;
use crate::offsets::offsets;
use crate::settings::settings::DEBUG_MODE;
use crate::shiptracker::ShipTracker;
use crate::viewtypes::Actor;

pub struct GameService{
    pub mem : Memory,
    pub to_draw : Arc<Mutex<Vec<Actor>>>,
    pub u_world : u64,
    pub g_names : u64,
    pub g_objects : u64,
    pub u_level : u64,
    pub l_player : Arc<RwLock<LocalPlayer>>,
    pub actor_name_map : HashMap<i32, String>,
    pub function_map : FunctionMap,
    pub ship_tracker : ShipTracker,
    pub island_service : u64
}

impl GameService{
    pub fn new(mem : Memory, to_draw : Arc<Mutex<Vec<Actor>>>, local_player_clone : Arc<RwLock<LocalPlayer>>) -> GameService {
        let u_world = mem.read_u64(mem.base_address + offsets::U_WORLD);
        let g_names = mem.read_u64(mem.base_address + offsets::G_NAMES);
        let g_objects = mem.read_u64(mem.base_address + offsets::G_OBJECTS);
        let u_level = mem.read_u64(u_world + offsets::PERSISTENT_LEVEL);

        let l_player = local_player_clone;

        let actor_name_map : HashMap<i32, String> = HashMap::new();
        let function_map = FunctionMap::new();
        let ship_tracker = ShipTracker::new();
        let island_service= 0;

        GameService{
            mem,
            to_draw,
            u_world,
            g_names,
            g_objects,
            u_level,
            l_player,
            actor_name_map,
            function_map,
            ship_tracker,
            island_service
        }
    }
}

impl Service for GameService {
    fn get_mem(&self) -> &Memory{
        &self.mem
    }
    fn get_draw_list(&self) -> &Arc<Mutex<Vec<Actor>>> {
        &self.to_draw
    }
    fn get_uworld(&self) -> u64 {
        self.u_world
    }
    fn get_gnames(&self) -> u64 {
        self.g_names
    }

    fn get_objects(&self) -> u64 {
        self.g_objects
    }
    fn get_lplayer(&self) -> &Arc<RwLock<LocalPlayer>> {
        &self.l_player
    }
    fn get_actor_map(&self) -> &HashMap<i32, String> {
        &self.actor_name_map
    }

    fn get_ship_tracker(&mut self) -> Option<&mut ShipTracker> {
        Some(&mut self.ship_tracker)
    }

    fn read_actors(&mut self){
        let actors : TArray = self.mem.read(self.u_level + offsets::ACTOR_ARRAY).unwrap_or_default();

        //Read all the actor addresses once so save memory reads
        let actor_addresses_byte_size = std::mem::size_of::<u64>() * (actors.count as usize);
        let mut level_actors_buffer = vec![0u8; actor_addresses_byte_size];
        let res = self.mem.read_memory(actors.data_ptr, &mut level_actors_buffer);

        let mut actor_array = Vec::with_capacity(actors.count as usize);
        for chunk in level_actors_buffer.chunks_exact(8) {
            let actor_address = u64::from_le_bytes(chunk.try_into().expect("slice with incorrect length"));
            actor_array.push(actor_address);
        }

        for actor_address in actor_array {
            // We start by getting the ActorID for a given actor, and comparing
            // that ID to a list of "known" id's we cache in self.actor_name_map
            let mut raw_name = String::new();
            let actor_id = self.mem.read_i32(actor_address + offsets::ACTOR_ID);

            if !self.actor_name_map.contains_key(&actor_id) && actor_id != 0 {
                match self.mem.read_gname(actor_id, self.g_names) {
                    Ok(name) => {
                        self.actor_name_map.insert(actor_id, name.clone());
                        raw_name = name;
                    },
                    Err(e) => {
                        eprintln!("Unable to find actor name: {}", e);
                        continue;
                    },
                }
            } else if let Some(name) = self.actor_name_map.get(&actor_id) {
                raw_name = name.clone();
            }

            if raw_name.is_empty() {
                continue;
            }

            if DEBUG_MODE {
                let lp = self.l_player.read().unwrap();
                let (actor_loc, actor_rot) = get_actor_root_comp(self, actor_address);
                if lp.distance_to(  &actor_loc) < 500.0 {
                    std::mem::drop(lp);
                    draw_treasure_name(actor_address, self, raw_name.as_str());
                }
            }

            //Here we handle the drawing to the actors we want
            let mut main_name : String = String::new();
            let actor_name_collection : Vec<&str>= raw_name.split('_').collect();
            if actor_name_collection.len() >= 3 {
                main_name.push_str( actor_name_collection[1] );
            }

            // If we find our actor in our map we execute the function associated with it
            // We use a map to find and execute the associated function.
            // In case the map fails due to complex names it goes into a classical else if structure
            // The map is preferable since it's O(1) or very close to it
            if let Some(&func) = self.function_map.map.get(main_name.as_str()) {
                func(actor_address, self, raw_name);
            }
            else if raw_name.contains("Gunpowder") {
                draw_treasure_name(actor_address, self, "Gunpowder");
            }
            else if raw_name.contains("DamageZone"){
                draw_damage_zones(actor_address, self, raw_name);
            }
            else if raw_name.contains("IslandService"){
                self.island_service = actor_address;
                //draw_island(actor_address, self, "Test".to_string());
            }
            else if raw_name.contains("BP_TreasureMap_C"){
                draw_x_marks(actor_address, self);
            }
        }
    }
}