use crate::gametypes::{FRepMovement, FVector};

pub struct ShipTracker{
    pub my_ship_movement: FRepMovement
}

impl ShipTracker {
    pub fn new() -> Self {
        ShipTracker{
            my_ship_movement: FRepMovement::default()
        }
    }
}

impl Default for ShipTracker {
    fn default() -> Self {
        ShipTracker{
            my_ship_movement: FRepMovement::default()
        }
    }
}