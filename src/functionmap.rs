//creating an actor map with all the actor names we want to read 

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::functionmap::cooking::draw_cooking_state;
use crate::functionmap::damagezones::draw_damage_zones;
use crate::functionmap::instantladder::do_instant_ladder;
use crate::playermodel::LocalPlayer;
use crate::gameservice::GameService;
use crate::memory::Memory;
use crate::functionmap::players::*;
use crate::functionmap::uactor::get_actor_root_comp;
use crate::functionmap::ship::draw_ship;
use crate::functionmap::treasure::draw_treasure;
use crate::gametypes::Service;
use crate::viewtypes::Actor;

mod ship;
pub mod treasure;
pub mod players;
mod cannon;
pub(crate) mod uactor;
mod quarticprediction;
pub mod damagezones;
pub mod storagebarrel;
pub mod xmarks;
pub mod vaults;
pub mod islandservice;
pub mod riddles;
pub mod cooking;
pub mod instantladder;

type DrawFunction = fn(u64, &mut dyn Service, raw_name : String);

pub struct FunctionMap {
    pub map: HashMap<&'static str, DrawFunction>
}

impl FunctionMap {
    pub fn new() -> FunctionMap {
        let mut map: HashMap<&'static str, DrawFunction> = HashMap::new();

        //Actor name and its functionality
        //Object name is the one between first '_' and second '_'

        //Ships
        map.insert("SmallShipTemplate", draw_ship);
        map.insert("SmallShipNetProxy", draw_ship);
        map.insert("MediumShipTemplate", draw_ship);
        map.insert("MediumShipNetProxy", draw_ship);
        map.insert("LargeShipTemplate", draw_ship);
        map.insert("LargeShipNetProxy", draw_ship);

        //AI Ships
        map.insert("AISmallShipTemplate", draw_ship);
        map.insert("AISmallShipNetProxy", draw_ship);
        map.insert("AILargeShipTemplate", draw_ship);
        map.insert("AILargeShipNetProxy", draw_ship);

        //Players
         map.insert("PlayerPirate", draw_player);

        //Loot
        map.insert("TreasureChest", draw_treasure);
        map.insert("BountyRewardSkull", draw_treasure);
        //map.insert("MerchantCrate", draw_treasure);
        map.insert("TreasureArtifact", draw_treasure);

        //Other
        map.insert("DamageZone", draw_damage_zones);
        map.insert("ShipCookingPot", draw_cooking_state);

        FunctionMap{
            map,
        }
    }
}