#![feature(is_some_and)]
#![feature(core_intrinsics)]

pub mod calc;
pub mod settings;
pub mod simulator;
pub mod states;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::{simulator::World, states::physics_context::{CollisionBehavior, EntityType}, settings::ControlStateHandler};

    use super::*;

    pub struct TestWorld {
        stone_height: u32,
    }

    impl World for TestWorld {
        fn get_block(&self, pos: &glam::Vec3A) -> Option<simulator::Block> {
            let b_type = if pos.y <= self.stone_height as f32 { 1 } else { 2 };
            // None
            // Some(simulator::Block::test_new(*pos, b_type));
            if b_type == 0 {
                Some(simulator::Block {
                    bounding_box: "empty".to_string(),
                    metadata: 0,
                    b_type,
                    position: *pos,
                    shapes: vec![]

                })
            } else {
                Some(simulator::Block {
                    bounding_box: "block".to_string(),
                    metadata: 0,
                    b_type,
                    position: *pos,
                    shapes: vec![[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]]

                })
            }
        }
    }

    #[test]
    fn test_basic() {
        let sim = simulator::Simulator::default();

        let world = TestWorld {
            stone_height: 60
        };

        let entity = states::EntityState {
            position: glam::Vec3A::new(0.0, 80.0, 0.0),
            control_states: ControlStateHandler {
                // forward: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut ctx = states::physics_context::EntityPhysicsContext {
            state: entity,
            collision_behavior: CollisionBehavior {
                block_effects: true,
                affected_after_collision: true,
            },
            gravity_then_drag: true,
            use_controls: true,
            step_height: 0.6, // player
            gravity: 0.08,
            airdrag:  0.9800000190734863,
            water_gravity: 0.08 / 16.0,
            lava_gravity: 0.08 / 4.0,
            pose: settings::PlayerPoses::Standing,
            entity_type: EntityType {
                e_type: "player".to_string(),
                name:"player".to_string(),
                width: Some(0.6),
                height: Some(1.8)
            }
        };

        for _ in 0..40 {
            ctx = sim.simulate(ctx, &world);
            println!("{} {}", ctx.state.position, ctx.state.velocity)
        }

        // let entity = settings::entity_physics_ctx::entity_physics_context::
    }

    #[test]
    fn it_works() {
        let result = glam::Vec3A::splat(0.0);

        let mut res1 = result;

        res1.x = res1.x + 1.0;

        println!("{result} {res1}");

        // assert_eq!(result, res1);
    }
}
