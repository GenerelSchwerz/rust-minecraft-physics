extern crate minecraft_physics;

mod tests {

    use minecraft_physics::{
        simulators::{self, World},
        states::{
            self,
            physics_context::{CollisionBehavior, EntityType},
            player_context::ControlStateHandler,
        },
    };

    use super::*;

    pub struct TestWorld {
        stone_height: u32,
    }

    impl World for TestWorld {
        fn get_block(&self, pos: &glam::Vec3A) -> Option<simulators::Block> {
            // None
            if pos.y as u32 > self.stone_height {
                Some(simulators::Block::test_new(
                    "empty".to_string(),
                    0,
                    1,
                    *pos,
                    vec![],
                ))
            } else {
                Some(simulators::Block::test_new(
                    "block".to_string(),
                    0,
                    2,
                    *pos,
                    vec![[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]],
                ))
            }
        }
    }

    #[test]
    fn test_basic() {
        let sim = simulators::prismarine_simulator::PrismarineSimulator::default();

        let world = TestWorld { stone_height: 60 };

        let entity = states::EntityState {
            position: glam::Vec3A::new(0.0, 80.0, 0.0),
            control_states: ControlStateHandler {
                forward: true,
                sprint: true,
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
            states::player_context::PlayerPoses::Standing,
            true,
            0.6, // player
            0.08,
            0.08 / 16.0,
            0.8,
            0.08 / 4.0,
            0.5,
            0.9800000190734863,
            true,
        );
        for _ in 0..40 {
            ctx = sim.simulate(ctx, &world);
            println!("{} {}", ctx.get_state().position, ctx.get_state().velocity)
        }

        // let entity = settings::entity_physics_ctx::entity_physics_context::
    }
}
