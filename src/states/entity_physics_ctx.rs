pub mod physics_context {
    use crate::{calc::aabb::AABB, settings::PlayerPoses, states::EntityState};

    pub struct CollisionBehavior {
        pub(crate) block_effects: bool,
        pub(crate) affected_after_collision: bool,
    }

    impl CollisionBehavior {
        pub fn new(block_effects: bool, affected_after_collision: bool) -> Self {
            Self {
                block_effects,
                affected_after_collision,
            }
        }
    }

    impl Default for CollisionBehavior {
        /// correct for mobs and players.
        fn default() -> Self {
            Self {
                block_effects: true,
                affected_after_collision: true,
            }
        }
    }

    /// placeholder.
    #[derive(Default)]
    pub struct EntityType {
        /// original: "type"
        pub(crate) e_type: String,
        pub(crate) name: String,
        pub(crate) width: Option<f32>,
        pub(crate) height: Option<f32>,
    }

    impl EntityType {
        pub fn new(e_type: String, name: String, width: Option<f32>, height: Option<f32>) -> Self {
            Self {
                e_type,
                name,
                width,
                height,
            }
        }
    }

    #[derive(Default)]
    pub struct EntityPhysicsContext {
        pub(crate) state: EntityState,

        pub(crate) collision_behavior: CollisionBehavior,
        pub(crate) entity_type: EntityType,
        pub(crate) pose: PlayerPoses,

        pub(crate) use_controls: bool,

        pub(crate) step_height: f32,
        pub(crate) gravity: f32,
        pub(crate) water_gravity: f32,
        pub(crate) lava_gravity: f32,
        pub(crate) airdrag: f32,
        pub(crate) gravity_then_drag: bool,
    }

    impl EntityPhysicsContext {
        pub fn raw(
            state: EntityState,
            collision_behavior: CollisionBehavior,
            entity_type: EntityType,
            pose: PlayerPoses,
            use_controls: bool,
            step_height: f32,
            gravity: f32,
            water_gravity: f32,
            lava_gravity: f32,
            airdrag: f32,
            gravity_then_drag: bool,
        ) -> Self {
            Self {
                state,
                collision_behavior,
                entity_type,
                pose,
                use_controls,
                step_height,
                gravity,
                water_gravity,
                lava_gravity,
                airdrag,
                gravity_then_drag,
            }
        }

        pub fn from_state(state: EntityState) -> Self {
            Self {
                state,
                ..Default::default()
            }
        }

        pub fn get_state(&self) -> &EntityState {
            &self.state
        }

        pub fn get_width(&self) -> f32 {
            if self.entity_type.e_type == "player" {
                // potential performance penalty due to self.pose needing copy (not zero-cost).
                return self.pose.get_info().width;
            }
            return self.entity_type.width.unwrap_or(0.0);
        }

        pub fn get_height(&self) -> f32 {
            if self.entity_type.e_type == "player" {
                // potential performance penalty due to self.pose needing copy (not zero-cost).
                return self.pose.get_info().height;
            }
            return self.entity_type.height.unwrap_or(0.0);
        }

        pub fn get_half_width(&self) -> f32 {
            return self.get_width() / 2.0;
        }

        /// Not sure whether or not this is used.
        pub fn get_bb_at_pos(&self, position: &glam::Vec3A) -> AABB {
            let half_width = self.entity_type.width.unwrap_or(0.0) / 2.0;
            return AABB {
                minX: position.x - half_width,
                minY: position.y,
                minZ: position.z - half_width,
                maxX: position.x + half_width,
                maxY: position.y + self.entity_type.height.unwrap_or(0.0),
                maxZ: position.z + half_width,
            };
        }

        /// Not sure whether or not this is used.
        pub fn get_bb_at_pos_with_pose(&self, position: &glam::Vec3A) -> AABB {
            let half_width = self.get_half_width();
            return AABB {
                minX: position.x - half_width,
                minY: position.y,
                minZ: position.z - half_width,
                maxX: position.x + half_width,
                maxY: position.y + self.get_height(),
                maxZ: position.z + half_width,
            };
        }

        /// Not sure whether or not this is used.
        pub fn get_current_bb_with_pose(&self) -> AABB {
            let half_width = self.get_half_width();
            return AABB {
                minX: self.state.position.x - half_width,
                minY: self.state.position.y,
                minZ: self.state.position.z - half_width,
                maxX: self.state.position.x + half_width,
                maxY: self.state.position.y + self.get_height(),
                maxZ: self.state.position.z + half_width,
            };
        }
    }
}
