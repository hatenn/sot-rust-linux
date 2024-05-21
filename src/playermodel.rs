use std::f64::consts::PI;

use maths_rs::{dot, sqrt, tan, Vec2d, Vec2f, Vec3d, Vec3f};
use maths_rs::vec::{Vec2, Vec3};
use crate::gametypes::{FCameraCacheEntry, FRepMovement};
use crate::memory::Memory;
use crate::offsets::offsets;
use crate::settings::settings::{GAME_FOV};

pub struct LocalPlayer {
    pub player_base: u64,
    pub player_pawn : u64,
    pub player_controller: u64,
    pub camera_pos: Vec3d,
    pub camera_rot: Vec3d,
    pub fov : f32,
    pub fov_multiplier : f32,
    pub window_width : f64,
    pub window_height : f64,
    pub u_world : u64,
}

impl LocalPlayer{

    pub fn new(mem : &Memory, window_width: f64, window_height : f64) -> LocalPlayer{
        let u_world = mem.read_u64(mem.base_address + offsets::U_WORLD);
        let game_instance = mem.read_u64(u_world + offsets::GAME_INSTANCE);
        let local_player_ptr = mem.read_u64(game_instance + offsets::LOCAL_PLAYERS);
        let local_player_base = mem.read_u64(local_player_ptr);
        let local_player_controller = mem.read_u64(local_player_base + offsets::PLAYER_CONTROLLER);
        let local_player_pawn = mem.read_u64(local_player_controller + offsets::LOCAL_PAWN);

        LocalPlayer{
            player_base : local_player_base,
            player_pawn : local_player_pawn,
            player_controller : local_player_controller,
            camera_pos: Vec3::default(),
            camera_rot: Vec3::default(),
            fov : GAME_FOV as f32,
            fov_multiplier : 1.0,
            window_width,
            window_height,
            u_world,
        }
    }

    //Updating local player info
    pub fn update(&mut self, mem : &Memory) {
        let game_instance = mem.read_u64(self.u_world + offsets::GAME_INSTANCE);
        let local_player_ptr = mem.read_u64(game_instance + offsets::LOCAL_PLAYERS);

        self.player_base = mem.read_u64(local_player_ptr);
        self.player_controller = mem.read_u64(self.player_base + offsets::PLAYER_CONTROLLER);
        self.player_pawn = mem.read_u64(self.player_controller + offsets::LOCAL_PAWN);

        let camera_manager = mem.read_u64(self.player_controller + offsets::CAMERA_MANAGER);
        let camera_cache: FCameraCacheEntry = mem.read(camera_manager + offsets::CAMERA_CACHE).unwrap_or_default();

        self.camera_pos.x = camera_cache.pov.location.x as f64;
        self.camera_pos.y = camera_cache.pov.location.y as f64;
        self.camera_pos.z = camera_cache.pov.location.z as f64;

        self.camera_rot.x = camera_cache.pov.rotation.pitch as f64;
        self.camera_rot.y = camera_cache.pov.rotation.yaw as f64;
        self.camera_rot.z = camera_cache.pov.rotation.roll as f64;

        self.fov = camera_cache.pov.fov * self.fov_multiplier;
    }

    pub fn distance_to(&self, target_coords: &Vec3d) -> f64 {
        let dx = self.camera_pos.x - target_coords.x;
        let dy = self.camera_pos.y - target_coords.y;
        let dz = self.camera_pos.z - target_coords.z;
        f64::sqrt(dx * dx + dy * dy + dz * dz)
    }

    pub fn world_to_screen(&self, target_player : &Vec3d, out_value: &mut Vec2d) -> bool{
        if target_player.x == 0.0 && target_player.y == 0.0 {
            return false;
        }

        let matrix = self.get_matrix();

        let v_delta = Vec3d::new(
            target_player.x - self.camera_pos.x,
            target_player.y - self.camera_pos.y,
            target_player.z - self.camera_pos.z
        );

        let mut v_transform : Vec3d = Vec3d::new( dot(v_delta, matrix[1]), dot(v_delta, matrix[2]), dot(v_delta, matrix[0]) );

        if v_transform.z < 1.0{
            v_transform.z = 1.0;
        }

        let camera_fov_angle = tan(self.fov as f64 * PI / 360.0); //90 is the fov angle, it depends on game/settings
        let screen_center_x = self.window_width / 2.0;
        let screen_center_y = self.window_height / 2.0;

        let pos = Vec2d::new(
            screen_center_x + v_transform.x * (screen_center_x / camera_fov_angle) / v_transform.z,
            screen_center_y - v_transform.y * (screen_center_x / camera_fov_angle) / v_transform.z
        );

        if pos.x > self.window_width || pos.x < 0.0 {
            return false;
        }

        if pos.y > self.window_height || pos.y < 0.0 {
            return false;
        }

        out_value.x = pos.x;
        out_value.y = pos.y;

        true
    }

    fn get_matrix(&self) -> [Vec3d; 3] {
        let mut v_matrix: [Vec3d; 3] = [Vec3d::default(), Vec3d::default(), Vec3d::default()];

        let rad_pitch: f64 = self.camera_rot.x.to_radians();
        let rad_yaw: f64 = self.camera_rot.y.to_radians();
        let rad_roll: f64 = self.camera_rot.z.to_radians();

        let sp = rad_pitch.sin() ;
        let cp = rad_pitch.cos() ;
        let sy = rad_yaw.sin() ;
        let cy = rad_yaw.cos() ;
        let sr = rad_roll.sin() ;
        let cr = rad_roll.cos() ;

        v_matrix[0].x = cp * cy;
        v_matrix[0].y = cp * sy;
        v_matrix[0].z = sp;

        v_matrix[1].x = sr * sp * cy - cr * sy;
        v_matrix[1].y = sr * sp * sy + cr * cy;
        v_matrix[1].z =  -sr * cp;

        v_matrix[2].x = -(cr * sp * cy + sr * sy);
        v_matrix[2].y = cy * sr - cr * sp * sy;
        v_matrix[2].z = cr * cp;

        return v_matrix;
    }
}

