use std::f32::consts::PI;
use std::ops::{Div, Sub};
use maths_rs::{atan, atan2, Vec2d, Vec3d};
use num_complex::Complex;
use crate::gameservice::GameService;
use crate::gametypes::{ACannon, FRepMovement, FVector, Service};
use crate::functionmap::players::{get_interactive_actor, get_player_linear_velocity, get_player_standing_actor};
use crate::functionmap::quarticprediction::{dot_p, solve_quartic};
use crate::functionmap::uactor::{get_actor_root_comp, get_attached_actor, get_distance_between, get_replicated_movement};
use crate::offsets::offsets;
use crate::shiptracker::ShipTracker;
use crate::viewtypes::{Actor, BLUE, GOLDENROD, ObjectType, RED, WHITE};

pub fn do_cannon_prediction(ship_actor: u64, game: &mut dyn Service){

    match is_local_on_cannon(game){
        Some(cannon_actor) => {
            let mut projectile_info: ACannon = game.get_mem().read(cannon_actor + offsets::PROJECTILE_SPEED).unwrap();
            let (mut cannon_coords, mut ship_coords) = get_coords(game, cannon_actor, ship_actor);
            let mut ship_movement = get_replicated_movement(game, ship_actor);

            draw_rotation_prediction(game, &projectile_info, ship_coords.clone(), ship_movement.clone());
            draw_quartic_prediction(game, &projectile_info, ship_coords.clone(), ship_movement.clone());
        }
        None => {}
    }

}

// Calculates a time-based function given multiple parameters
fn time_func(t: f32, k: f32, l: f32, m: f32, n: f32, r: f32, w: f32, theta: f32, s2: f32) -> f32 {
    let k2 = k * k;
    let l2 = l * l;
    let m2 = m * m;
    let n2 = n * n;
    let r2 = r * r;
    n2 * t.powi(4) + ((2.0 * m * n) - s2) * t.powi(2) + 2.0 * r * (k * (theta + (w * t)).cos() + l * (theta + (w * t)).sin()) + k2 + l2 + m2 + r2
}



fn is_local_on_cannon(game : &dyn Service) -> Option<u64>{
    let local_player_read = game.get_lplayer().read().unwrap();
    let mut attached_actor = get_interactive_actor(game, local_player_read.player_pawn);

    //Check if attached actor is a cannon
    if attached_actor != 0 {

        let mut raw_name = String::new();
        let actor_id = game.get_mem().read_i32(attached_actor + offsets::ACTOR_ID);

        if !game.get_actor_map().contains_key(&actor_id) && actor_id != 0 {
            match game.get_mem().read_gname(actor_id, game.get_gnames()) {
                Ok(name) => {
                    raw_name = name;
                },
                Err(e) => {
                    eprintln!("Unable to find actor name: {}", e);
                },
            }
        } else if let Some(name) = game.get_actor_map().get(&actor_id) {
            raw_name = name.clone();
        }

        if raw_name.contains("Cannon"){
            return Some(attached_actor);
        }
    }
    return None;
}

fn get_coords(game : &dyn Service, cannon_address : u64, ship_address : u64) -> (Vec3d, Vec3d) {
    let (cannon_location, cannon_rotation) = get_actor_root_comp(game, cannon_address);
    let (ship_location, ship_rotation) = get_actor_root_comp(game, ship_address);

    (cannon_location, ship_location)    
}

fn get_launch_angle(cannon_coords: &Vec3d, target_coords: &Vec3d, gravity: f64, projectile_speed: f64, distance: f64) -> f64 {
    let height = target_coords.z - cannon_coords.z;

    let speed2 = projectile_speed.powi(2);
    let speed4 = speed2.powi(2);
    let gravity_x = gravity * distance;

    let root_part = gravity * (gravity * distance.powi(2) + 2.0 * height * speed2);
    let root = speed4 - root_part;

    if root < 0.0 {
        return 0.0;
    }

    let root_sqrt = root.sqrt();

    let angle_calc1 = (speed2 - root_sqrt) / gravity_x;
    let angle_calc2 = (speed2 + root_sqrt) / gravity_x;

    let angle_calc = angle_calc1.min(angle_calc2);

    let new_angle = angle_calc.atan();

    new_angle.tan()
}

fn launch_angles(cannon_coords: &Vec3d, target_coords: &Vec3d, gravity: f64, projectile_speed: f64, distance: f64) -> f64 {
    let new_angle_tan = get_launch_angle(cannon_coords, target_coords, gravity, projectile_speed, distance);

    // Updating the target's Z coordinate based on the calculated angle and distance.
    let draw_angle = distance * new_angle_tan;

    (cannon_coords.z + draw_angle)
}

fn draw_quartic_prediction(game: &mut dyn Service, projectile_info : &ACannon, mut ship_coords: Vec3d, mut ship_movement: FRepMovement){
    let my_ship_movement = match game.get_ship_tracker() {
        Some(ship_tracker) => { ship_tracker.my_ship_movement }
        None => { FRepMovement::default() }
    };

    let local_p = game.get_lplayer().read().unwrap();
    let gravity = projectile_info.gravity_scale * 981.0;
    let gravity_vec = Vec3d::new(0.0,0.0, (gravity * -1.0)as f64);

    let distance = get_2d_distance(&local_p.camera_pos, &ship_coords);

    if distance < 50000.0 && distance > 5000.0 {

        let net_vel_vec = ship_movement.linear_velocity - my_ship_movement.linear_velocity;
        let net_pos = ship_coords - local_p.camera_pos;

        let net_vel = Vec3d::new(net_vel_vec.x as f64, net_vel_vec.y as f64,net_vel_vec.z as f64);

        // Quartic
        let c4 = dot_p(gravity_vec, gravity_vec) * 0.25;
        let c3 = dot_p(net_vel, gravity_vec);
        let c2 = dot_p(net_pos, gravity_vec) + dot_p(net_vel, net_vel) - (projectile_info.projectile_speed.powi(2) as f64);
        let c1 = 2.0 * dot_p(net_pos, net_vel);
        let c0 = dot_p(net_pos, net_pos);

        let p_in_coeffs = [Complex::new(c0, 0.0), Complex::new(c1, 0.0), Complex::new(c2, 0.0), Complex::new(c3, 0.0), Complex::new(c4, 0.0)];
        let mut p_out_roots = [Complex::new(0.0, 0.0); 4];

        solve_quartic(&p_in_coeffs, &mut p_out_roots);

        let f_best_root = p_out_roots.iter()
            .filter(|&root| root.re > 0.0 && root.im.abs() < 0.0001)
            .map(|root| root.re)
            .fold(f64::MAX, f64::min);

        if f_best_root == f64::MAX {
            return;
        }

        let mut o_aim_at = ship_coords + (net_vel * f_best_root);
        o_aim_at.z += launch_angles(&local_p.camera_pos, &o_aim_at, gravity as f64, projectile_info.projectile_speed as f64, distance);

        //Draw prediction
        let mut screen_pos = Vec2d::default();
        if local_p.world_to_screen(&o_aim_at, &mut screen_pos) {
            let my_actor = Actor::new(screen_pos.x, screen_pos.y, 10.0, 10.0, String::from(""), 8, ObjectType::Circle, GOLDENROD);
            game.get_draw_list().lock().unwrap().push(my_actor);
        }
    }
}

fn get_2d_distance(cannon_coords: &Vec3d, target_coords: &Vec3d) -> f64 {
    let x_power = (cannon_coords.x - target_coords.x).powi(2);
    let y_power = (cannon_coords.y - target_coords.y).powi(2);
    let dist_sum = x_power + y_power;
    let distance_xy : f64 = (dist_sum).sqrt();
    return distance_xy;
}

fn draw_rotation_prediction(game: &mut dyn Service, projectile_info: &ACannon, mut ship_coords: Vec3d, ship_movement: FRepMovement) {
    let mut my_ship_movement = match game.get_ship_tracker() {
        Some(ship_tracker) => { ship_tracker.my_ship_movement }
        None => { FRepMovement::default() }
    };

    let local_player_read = game.get_lplayer().read().unwrap();

    let gravity = (projectile_info.gravity_scale * 981.0) / 100.0;
    let projectile_speed = projectile_info.projectile_speed / 100.0;
    let cannon_coords = Vec3d::new(local_player_read.camera_pos.x / 100.0, local_player_read.camera_pos.y / 100.0, local_player_read.camera_pos.z / 100.0);

    my_ship_movement.linear_velocity.x /= 100.0;
    my_ship_movement.linear_velocity.y /= 100.0;
    my_ship_movement.linear_velocity.z /= 100.0;

    ship_coords.x /= 100.0;
    ship_coords.y /= 100.0;
    ship_coords.z /= 100.0;

    let distance = get_2d_distance(&cannon_coords, &ship_coords);

    if distance < 600.00 {
        let linear_velocity_x = ship_movement.linear_velocity.x / 100.0;
        let linear_velocity_y = ship_movement.linear_velocity.y / 100.0;

        let angular_velocity = ship_movement.angular_velocity.z;
        let angular_velocity_radians = angular_velocity * PI / 180.0; // Correctly converting degrees to radians.
        let speed_magnitude = (linear_velocity_x.powi(2) + linear_velocity_y.powi(2)).sqrt();

        let disk_radius = speed_magnitude / angular_velocity_radians;
        let effective_radius = disk_radius * 0.98;

        let actual_heading = atan2(linear_velocity_y, linear_velocity_x);
        let actual_heading_deg = actual_heading * 180.0 / PI;

        let center = FVector {
            x: effective_radius * (actual_heading + PI / 2.0).cos() + ship_coords.x as f32,
            y: effective_radius * (actual_heading + PI / 2.0).sin() + ship_coords.y as f32,
            z: 0.0, // Assuming z is not used or is always set to 0.
        };

        let angle_theta = actual_heading - PI / 2.0;

        let mut t_time: f32 = 0.0;
        let t_iterator: f32 = 0.1;
        let mut target_pos_t = Vec3d::default();
        while t_time < 50.0 {
            let w_time = angular_velocity_radians * t_time;
            target_pos_t.x = (center.x + (effective_radius * (w_time + angle_theta).cos() - my_ship_movement.linear_velocity.x * t_time)) as f64;
            target_pos_t.y = (center.y + (effective_radius * (w_time + angle_theta).sin() - my_ship_movement.linear_velocity.y * t_time)) as f64;
            target_pos_t.z = ship_coords.z; // Assuming flat trajectory for simplicity.

            let n_dist = get_2d_distance(&cannon_coords, &target_pos_t);

            let s_angle = get_launch_angle(&cannon_coords, &target_pos_t, gravity as f64, projectile_speed as f64, n_dist);
            let s_time = n_dist / (projectile_speed as f64 * s_angle.cos());

            if s_time < t_time as f64 {
                target_pos_t.z = launch_angles(&cannon_coords, &target_pos_t, gravity as f64, projectile_speed as f64, n_dist); // Assuming launch_angles function adjusts z based on angle and distance.
                break;
            }
            t_time += t_iterator;
        }

        target_pos_t.x *= 100.0;
        target_pos_t.y *= 100.0;
        target_pos_t.z *= 100.0;

        // Draw prediction.
        let mut screen_pos = Vec2d::default();
        if local_player_read.world_to_screen(&target_pos_t, &mut screen_pos) {
            let my_actor = Actor::new(screen_pos.x, screen_pos.y, 10.0, 10.0, String::from(""), 8, ObjectType::Circle, BLUE);
            game.get_draw_list().lock().unwrap().push(my_actor);
        }
    }
}

fn aim_at_static_target(o_target_pos: &Vec3d, f_projectile_speed: f64, f_projectile_gravity_scalar: f64, o_source_pos: &Vec3d) -> f64 {
    let gravity = 981.0 * f_projectile_gravity_scalar;
    let diff = o_target_pos - o_source_pos;
    let o_diff_xy = Vec3d::new(diff.x, diff.y, 0.0);
    let f_ground_dist = ((o_diff_xy.x * o_diff_xy.x) + (o_diff_xy.y * o_diff_xy.y)).sqrt();
    let s2 = f_projectile_speed.powi(2);
    let s4 = s2.powi(2);
    let y = diff.z;
    let x = f_ground_dist;
    let gx = gravity * x;
    let root_part = s4 - (gravity * ((gx * x) + (2.0 * y * s2)));

    if root_part < 0.0 {
        return 0.0;
    }

    let root = root_part.sqrt();

    let angle_calc1 = (s2 - root) / gx;
    let angle_calc2 = (s2 + root) / gx;

    let angle_calc = angle_calc1.min(angle_calc2);

    let new_angle = angle_calc.atan();

    let new_angle_tan = new_angle.tan();

    let draw_angle = x * new_angle_tan;

    return o_source_pos.z + draw_angle;
}

// Calculates the derivative of the time-based function
fn time_deriv_func(t: f32, k: f32, l: f32, m: f32, n: f32, r: f32, w: f32, theta: f32, s2: f32) -> f32 {
    let n2 = n * n;
    4.0 * n2 * t.powi(3) + 2.0 * ((2.0 * m * n) - s2) * t + 2.0 * r * w * (l * (theta + (w * t)).cos() - k * (theta + (w * t)).sin())
}

// Implements the Newton-Raphson method for finding roots of equations
fn newton_raphson(mut t: f32, k: f32, l: f32, m: f32, n: f32, r: f32, w: f32, theta: f32, s2: f32) -> f32 {
    let mut h = time_func(t, k, l, m, n, r, w, theta, s2) / time_deriv_func(t, k, l, m, n, r, w, theta, s2);
    let mut counter = 0;
    while h.abs() >= 0.01 {
        if counter > 200 {
            break;
        }
        h = time_func(t, k, l, m, n, r, w, theta, s2) / time_deriv_func(t, k, l, m, n, r, w, theta, s2);
        t -= h;
        counter += 1;
    }
    t
}