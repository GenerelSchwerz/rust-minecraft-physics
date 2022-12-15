

pub mod entity_physics_context {
    use crate::{calc::aabb::AABB, settings::{PlayerPoses}, states::EntityState};


    pub struct CollisionBehavior {
        pub(crate) block_effects: bool,
        pub(crate) affected_after_collision: bool,
    }

    /// placeholder.
    pub struct EntityType {
        /// original: "type"
        pub(crate) e_type: String,
        pub(crate) name: String,
        width: Option<f32>,
        height: Option<f32>,
    }

    pub struct EntityPhysicsContext {
        pub(crate) state: EntityState,

        pub(crate) collision_behavior: CollisionBehavior,
        pub(crate) entity_type: EntityType,
        pose: PlayerPoses,
  
        pub(crate) use_controls: bool,
   
        pub(crate) position: glam::Vec3A,
        pub(crate) velocity: glam::Vec3A,


        // pub(crate) attributes: 

        pub(crate) step_height: f32,
        pub(crate) gravity: f32,
        pub(crate) water_gravity: f32,
        pub(crate) lava_gravity: f32,
        pub(crate) airdrag: f32,
        pub(crate) gravity_then_drag: bool,
   
    }

    impl EntityPhysicsContext {
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
                minX: self.position.x - half_width,
                minY: self.position.y,
                minZ: self.position.z - half_width,
                maxX: self.position.x + half_width,
                maxY: self.position.y + self.get_height(),
                maxZ: self.position.z + half_width,
            };
        }
    }

}

