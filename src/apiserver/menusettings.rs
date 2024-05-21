use std::sync::mpsc::{Sender, SendError};
use serde::{Deserialize, ser::SerializeStruct, Serialize};

#[derive(Deserialize, Debug)]
pub struct MenuSettings {
    pub prediction: bool,
    pub treasure: bool,
    pub xMaps: bool,
    pub riddles: bool,
    pub fovMultiplier: f32,
    pub maxSimNum: i32,
    pub movementMode: u8,
    pub instantLadder: bool,
    pub extendedReach: bool,
    pub increaseSpeed: bool,
    pub forceMovement: bool,
}

impl MenuSettings {
    pub fn update_settings(tx : &Sender<MenuSettings>, request : String) -> String {
        let body_start_index = request.find("{").unwrap_or_default();
        let body_end_index = request.rfind("}").unwrap_or_default() + 1;
        let body = &request[body_start_index..body_end_index];

        let menu_settings : MenuSettings = serde_json::from_str(body).unwrap();

        match tx.send(menu_settings){
            Ok(_) => {}
            Err(err) => {
                println!("Error: {}", err);
            }
        }

        "{}".to_string()
    }
}

impl Default for MenuSettings {
    fn default() -> Self {
        MenuSettings {
            prediction: true,
            treasure: true,
            xMaps: true,
            riddles: true,
            fovMultiplier: 1.22,
            maxSimNum: 8,
            movementMode: 4,
            instantLadder: true,
            extendedReach: true,
            increaseSpeed: true,
            forceMovement: false,
        }
    }
}