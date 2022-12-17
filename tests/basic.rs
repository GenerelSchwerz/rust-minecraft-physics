extern crate minecraft_physics;

mod tests {
    use minecraft_physics::{
        settings::{self, ControlStateHandler},
        simulator::{self, World},
        states::{
            self,
            physics_context::{CollisionBehavior, EntityType},
        },
    };

    use super::*;

    pub struct TestWorld {
        stone_height: u32,
    }

    impl World for TestWorld {
        fn get_block(&self, pos: &glam::Vec3A) -> Option<simulator::Block> {
            let b_type = if pos.y <= self.stone_height as f32 {
                1
            } else {
                2
            };
            // None
            // Some(simulator::Block::test_new(*pos, b_type));
            if b_type == 0 {
                Some(simulator::Block::test_new(
                    "empty".to_string(),
                    0,
                    b_type,
                    *pos,
                    vec![],
                ))
            } else {
                Some(simulator::Block::test_new(
                    "block".to_string(),
                    0,
                    b_type,
                    *pos,
                    vec![[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]],
                ))
            }
        }
    }

    #[test]
    fn test_basic() {
        let sim = simulator::Simulator::default();

        let world = TestWorld { stone_height: 60 };

        let entity = states::EntityState {
            position: glam::Vec3A::new(0.0, 80.0, 0.0),
            control_states: ControlStateHandler {
                // forward: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let collision_behavior = CollisionBehavior::new(true, true);
        let entity_type = EntityType::new(
            "player".to_string(),
            "player".to_string(),
            Some(0.6),
            Some(1.8),
        );

        let mut ctx = states::physics_context::EntityPhysicsContext::raw(
            entity,
            collision_behavior,
            entity_type,
            settings::PlayerPoses::Standing,
            true,
            0.6, // player
            0.08,
            0.9800000190734863,
            0.08 / 16.0,
            0.08 / 4.0,
            true,
        );
        for _ in 0..40 {
            ctx = sim.simulate(ctx, &world);
            println!("{} {}", ctx.get_state().position, ctx.get_state().velocity)
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
