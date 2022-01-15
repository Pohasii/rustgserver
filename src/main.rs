use crate::messages_from as from_client; // {finish_message_buffer, root_as_message, Message, MessageArgs, Vec2}
use crate::messages_to as to_client; // {finish_message_buffer, root_as_message, Message, MessageArgs, Vec2}
use core::time;
use flatbuffers;
use flatbuffers::FlatBufferBuilder;
use rapier2d::na::Vector2;
use rapier2d::prelude::*;
use std::f32::consts::FRAC_PI_2;
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::{Duration, SystemTime};
//use tokio::net::UdpSocket;

fn main() {
    let mut server = new_server();
    /* Run the game loop, stepping the simulation once per frame. */

    // let mut builder = FlatBufferBuilder::new();

    server.add_player(1);
    server.add_player(1);
    server.add_player(1);
    //println!("{:#?}", fb_ser(&mut builder, &server.players));

    server.add_map();
    server.start();
}

// struct NetworkServer {
//     socket: UdpSocket,
//     socket_addr: SocketAddr,
// }

// impl NetworkServer {
//     fn send(self) {
//         // self.socket.send_to()
//     }
// }

struct GameServer {
    server_step_speed: Duration, //f64,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
    //physics_hooks: dyn PhysicsHooks<RigidBodySet, ColliderSet>,
    //event_handler: dyn EventHandler,
    step: u32,
    player_id: u8,
    players: Vec<Player>,
    time_step: f64,
}

fn new_server() -> GameServer {
    GameServer {
        server_step_speed: time::Duration::from_secs_f64(1.0 / 2.0), //server_step_speed: 33.33,
        rigid_body_set: RigidBodySet::new(),
        collider_set: ColliderSet::new(),
        gravity: vector![0.0, 0.0],
        integration_parameters: IntegrationParameters::default(),
        physics_pipeline: PhysicsPipeline::new(),
        island_manager: IslandManager::new(),
        broad_phase: BroadPhase::new(),
        narrow_phase: NarrowPhase::new(),
        joint_set: JointSet::new(),
        ccd_solver: CCDSolver::new(),
        //physics_hooks: (),
        //event_handler: (),
        step: 0,
        player_id: 0,
        players: Vec::new(),
        time_step: 0.0,
    }
}

impl GameServer {
    fn start(&mut self) {
        let mut builder = FlatBufferBuilder::new();
        loop {
            let start = SystemTime::now();
            self.step();

            self.update_player_stat();

            let calc = SystemTime::now().duration_since(start).unwrap();
            if calc < self.server_step_speed {
                thread::sleep(self.server_step_speed - calc);
                // thread::sleep(time::Duration::from_millis(
                //     (self.server_step_speed - calc) as u64,
                // ));
            }
            // self.time_step = self.physics_pipeline.counters.step_time();
            // if self.time_step.le(&self.server_step_speed) {
            //     thread::sleep(time::Duration::from_millis(
            //         (self.server_step_speed - self.time_step) as u64,
            //     ));
            // }
            for p in self.players.iter() {
                println!(
                    "Player id: {} altitude: y:{}, x:{}",
                    p.id,
                    self.rigid_body_set
                        .get(p.rigid_body_handle)
                        .unwrap()
                        .translation()
                        .y,
                    self.rigid_body_set
                        .get(p.rigid_body_handle)
                        .unwrap()
                        .translation()
                        .x,
                );
            }

            println!("step time {}, current step {}", calc.as_millis(), self.step);

            println!("{:#?}", fb_ser(&mut builder, &self.players));
        }
    }

    fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            &mut self.ccd_solver,
            &(),
            &(),
            //&self.physics_hooks,
            //&self.event_handler,
        );

        self.step = self.step + 1;
    }

    fn add_player(&mut self, number_team: u8) -> u8 {
        // self.rigid_body_set
        //     .get(self.players.first().unwrap().rigid_body_handle)
        //     .unwrap()
        //     .translation()
        //     .y,

        // for p in self.players.iter() {
        //     self.rigid_body_set[p.rigid_body_handle].translation().y,
        //     self.rigid_body_set[p.rigid_body_handle].translation().x,
        //
        // }

        /* Create the bouncing player. */
        let rigid_body: RigidBody = RigidBodyBuilder::new_dynamic()
            .translation(vector![3.0, 3.0]) // start position for player
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.0).build();
        let body_handle: RigidBodyHandle = self.rigid_body_set.insert(rigid_body);
        let call_hand: ColliderHandle =
            self.collider_set
                .insert_with_parent(collider, body_handle, &mut self.rigid_body_set);

        let p_id = self.get_player_id();

        let mut p: Player = Player {
            move_speed: 1 as f32,
            id: p_id,
            collider_handle: call_hand,
            rigid_body_handle: body_handle,
            number_team,
            x: 0.0,
            y: 0.0,
        };
        self.players.push(p);

        p_id
    }

    fn add_map(&mut self) {
        /* Create the ground. */

        // down
        self.collider_set.insert(
            ColliderBuilder::cuboid(100.0, 0.1)
                .position(Isometry::new(Vector2::new(50.0, 0.0), 0.0))
                .build(),
        );

        // top
        self.collider_set.insert(
            ColliderBuilder::cuboid(100.0, 0.1)
                .position(Isometry::new(Vector2::new(50.0, 100.0), 0.0))
                .build(),
        );

        // left
        self.collider_set.insert(
            ColliderBuilder::cuboid(100.0, 0.1)
                .position(Isometry::new(Vector2::new(0.0, 50.0), FRAC_PI_2))
                .build(),
        );

        // right
        self.collider_set.insert(
            ColliderBuilder::cuboid(100.0, 0.1)
                .position(Isometry::new(Vector2::new(100.0, 50.0), FRAC_PI_2))
                .build(),
        );
    }

    fn get_player_id(&mut self) -> u8 {
        self.player_id = self.player_id + 1;
        self.player_id
    }

    //         self.rigid_body_set
    //             .get(p.rigid_body_handle)
    //             .unwrap()
    //             .translation()
    //             .x,

    fn update_player_stat(&mut self) {
        for player in self.players.iter_mut() {
            player.x = self.rigid_body_set[player.rigid_body_handle]
                .translation()
                .x;
            player.y = self.rigid_body_set[player.rigid_body_handle]
                .translation()
                .y;
        }
    }
}

struct Player {
    move_speed: f32,
    id: u8,
    collider_handle: ColliderHandle,
    rigid_body_handle: RigidBodyHandle,
    number_team: u8,
    x: f32,
    y: f32,
}

impl Player {}

// fn deser_imcoming_message(buf: &[u8]) -> Vec2 {
//     roo
// }

fn fb_ser(builder: &mut flatbuffers::FlatBufferBuilder, players: &Vec<Player>) -> Vec<u8> {
    builder.reset();

    let mut p: to_client::Game_Object = Default::default();
    let mut pls = Vec::new();
    //builder.start_vector(players.len());
    for pl in players.iter() {
        p.set_x(pl.x);
        p.set_y(pl.y);
        p.set_id(pl.id);
        p.set_obj_type(1);
        pls.push(p)
        //builder.push(Object::create(builder, &p))
    }

    let res = &to_client::MessageArgs {
        players: Some(builder.create_vector(&pls)),
    };

    let offset = to_client::Message::create(builder, res);
    to_client::finish_message_buffer(builder, offset);

    builder.finished_data().to_vec()
}

fn deser(buf: &[u8]) -> from_client::Vec2 {
    from_client::root_as_vec_2(buf).unwrap()
}

// Automatically generated by the Flatbuffers compiler. Do not modify.
pub mod messages_from {
    use super::*;
    mod vec_2_generated;
    pub use self::vec_2_generated::*;
} // messages_from
  // Automatically generated by the Flatbuffers compiler. Do not modify.
pub mod messages_to {
    use super::*;
    mod message_generated;
    pub use self::message_generated::*;
    mod game_object_generated;
    pub use self::game_object_generated::*;
} // messages_to
