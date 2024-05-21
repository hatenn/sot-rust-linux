use std::collections::HashMap;
use std::ops::Sub;
use std::sync::{Arc, Mutex, RwLock};
use maths_rs::Vec3d;
use crate::memory::Memory;
use crate::playermodel::LocalPlayer;
use crate::shiptracker::ShipTracker;
use crate::viewtypes::Actor;

pub trait Service{
    fn get_mem(&self) -> &Memory;
    fn get_draw_list(&self) -> &Arc<Mutex<Vec<Actor>>>;
    fn get_uworld(&self) -> u64;
    fn get_gnames(&self) -> u64;
    fn get_objects(&self) -> u64;
    fn get_lplayer(&self) -> &Arc<RwLock<LocalPlayer>>;
    fn get_actor_map(&self) -> &HashMap<i32, String>;
    fn read_actors(&mut self);

    fn get_ship_tracker(&mut self) -> Option<&mut ShipTracker>{
        None
    }

    fn run(&mut self){
        loop {
            self.read_actors();
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FQuat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TArray {
    pub data_ptr: u64,
    pub count: i32,
    pub max: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FRotator {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FLocation {
    pub location: FVector,
    pub rotation: FRotator,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct FCameraCacheEntry {
    pub time_stamp: f32,
    pub unknown_data_4: [u8; 0xc],
    pub pov: FMinimalViewInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FMinimalViewInfo {
    pub location: FVector,
    pub rotation: FRotator,
    pub unknown_data_18: [u8; 0x10],
    pub fov: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACannon {
    pub projectile_speed : f32,
    pub gravity_scale : f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FRepMovement {
    pub linear_velocity: FVector,
    pub angular_velocity: FVector,
    pub location: FVector,
    pub rotation: FRotator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FWorldMapIslandDataCaptureParams {
    pub CameraPosition : FVector,
    pub CameraOrientation : FRotator,
    pub WorldSpaceCameraPosition : FVector,
    pub CameraFOV : f32,
    pub  CameraAspect : f32,
    pub  CameraOrthoWidth : f32,
    pub  CameraNearClip : f32,
    pub  CameraFarClip : f32,
    pub  TextureWidth : i32,
    pub  TextureHeight : i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WideName{
    pub word : [u16; 64]
}

impl Default for FWorldMapIslandDataCaptureParams {
    fn default() -> Self {
        FWorldMapIslandDataCaptureParams {
             CameraPosition : FVector::default(),
             CameraOrientation : FRotator::default(),
             WorldSpaceCameraPosition : FVector::default(),
             CameraFOV : 0.0,
             CameraAspect : 0.0,
              CameraOrthoWidth : 0.0,
              CameraNearClip : 0.0,
              CameraFarClip : 0.0,
              TextureWidth : 0,
              TextureHeight : 0,
        }
    }
}

impl Default for WideName {
    fn default() -> Self {
        WideName{
            word : [0u16; 64]
        }
    }
}

impl Default for FRepMovement {
    fn default() -> Self {
        FRepMovement {
            linear_velocity: FVector::default(),
            angular_velocity: FVector::default(),
            location: FVector::default(),
            rotation: FRotator::default(),
        }
    }
}

impl Default for FVector {
    fn default() -> Self {
        FVector { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Default for FRotator {
    fn default() -> Self {
        FRotator { pitch: 0.0, yaw: 0.0, roll: 0.0 }
    }
}

impl Default for TArray {
    fn default() -> Self {
        TArray { data_ptr: 0, count : 0, max : 0 }
    }
}

impl Default for FLocation {
    fn default() -> Self {
        FLocation {
            location: FVector::default(),
            rotation: FRotator::default(),
        }
    }
}

impl Default for FCameraCacheEntry {
    fn default() -> Self {
        FCameraCacheEntry {
            time_stamp: 0.0,
            unknown_data_4: [0u8; 12],
            pov: FMinimalViewInfo::default(),
        }
    }
}

impl Default for FMinimalViewInfo {
    fn default() -> Self {
        FMinimalViewInfo {
            location: FVector::default(),
            rotation: FRotator::default(),
            unknown_data_18: [0u8; 16],
            fov: 0.0,
        }
    }
}

impl Sub for FVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        FVector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FVector {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        FVector { x, y, z }
    }

    pub(crate) fn size_xy(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn unit(&self) -> FVector {
        let mag = self.size_xy(); // Assuming size_xy gives the magnitude for the XY plane
        FVector::new(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn to_vec3d(&self) -> Vec3d {
        Vec3d::new(self.x as f64, self.y as f64, self.z as f64)
    }
}