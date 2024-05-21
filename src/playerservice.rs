use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::{ptr, thread};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;
use crate::apiserver::menusettings::MenuSettings;
use crate::functionmap::FunctionMap;
use crate::functionmap::instantladder::do_instant_ladder;
use crate::functionmap::players::*;
use crate::gametypes::{Service, TArray};
use crate::memory::Memory;
use crate::offsets::offsets;
use crate::playermodel::LocalPlayer;
use crate::viewtypes::{Actor, GREEN, ObjectType};

pub struct PlayerService {
    pub rx : Receiver<MenuSettings>,
    pub mem : Memory,
    pub to_draw : Arc<Mutex<Vec<Actor>>>,
    pub u_world : u64,
    pub g_names : u64,
    pub g_objects : u64,
    pub u_level : u64,
    pub l_player : Arc<RwLock<LocalPlayer>>,
    pub actor_name_map : HashMap<i32, String>,
    pub function_map : FunctionMap,
    pub menu_settings : MenuSettings,
    old_player : u64,
}

impl PlayerService {
    pub fn new(rx : Receiver<MenuSettings>, mem : Memory, to_draw : Arc<Mutex<Vec<Actor>>>, local_player_clone : Arc<RwLock<LocalPlayer>>) -> PlayerService {
        let u_world = mem.read_u64(mem.base_address + offsets::U_WORLD);
        let g_names = mem.read_u64(mem.base_address + offsets::G_NAMES);
        let g_objects = mem.read_u64(mem.base_address + offsets::G_OBJECTS);
        let u_level = mem.read_u64(u_world + offsets::PERSISTENT_LEVEL);

        let l_player = local_player_clone;
        let actor_name_map : HashMap<i32, String> = HashMap::new();
        let function_map = FunctionMap::new();
        let menu_settings = MenuSettings::default();

        let old_player= 0;

            PlayerService {
                rx,
                mem,
                to_draw,
                u_world,
                g_names,
                g_objects,
                u_level,
                l_player,
                actor_name_map,
                function_map,
                old_player,
                menu_settings,
            }
    }
}

impl Service for PlayerService {
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

    fn read_actors(&mut self){

        match self.rx.try_recv() {
            Ok(menu_settings) => {
                self.menu_settings = menu_settings;
            }
            _ => {}
        }

        let mut l_player = self.l_player.write().unwrap();
        l_player.update(&self.mem);
        l_player.fov_multiplier = self.menu_settings.fovMultiplier;
        drop(l_player);

        let mut l_player = self.l_player.read().unwrap();
        let current_local_pawn = l_player.player_pawn;
        // Player updated (Died in game || changed server)
        if current_local_pawn != self.old_player {

            if self.menu_settings.extendedReach{
                extend_interaction_range(self, current_local_pawn);
            }

            self.old_player = current_local_pawn;
        }

        if self.menu_settings.instantLadder{
            do_instant_ladder(self, current_local_pawn);
        }

        set_locked_fov(self, l_player.player_controller, l_player.fov);

        let char_move_comp = self.mem.read_u64(l_player.player_pawn + offsets::CHARACTER_MOVEMENT_COMP);

        // Small speedhack
        if self.menu_settings.increaseSpeed {
            let client_prediction_data = self.mem.read_u64(char_move_comp + offsets::PREDICTION_CLIENT_DATA);
            let client_time_stamp = self.mem.read_f32(client_prediction_data + 0xc);
            self.mem.write(client_prediction_data + 0xc, client_time_stamp + 0.00011446).unwrap_or_default();
        }

        set_max_simulations(self, char_move_comp, self.menu_settings.maxSimNum);

        if self.menu_settings.forceMovement {
            // Enum: 0 - None, 1 - Walking, Falling, Swimming, Flying...
            set_movement_mode(self, char_move_comp, self.menu_settings.movementMode);
        }

        //Draw crosshair
        let my_actor = Actor::new(l_player.window_width / 2.0, l_player.window_height / 2.0, 8.0, 8.0, String::from(""), 8, ObjectType::Circle, GREEN);
        self.to_draw.lock().unwrap().push(my_actor);

        thread::sleep(Duration::from_millis(1));
    }
}
