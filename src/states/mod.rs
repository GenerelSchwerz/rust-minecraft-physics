mod entity_physics_ctx;
pub use entity_physics_ctx::physics_context;

use std::collections::HashMap;

use inter_struct::prelude::*;

use crate::{calc::aabb::AABB, settings::{PlayerPoses, ControlStateHandler, PlayerAttribute}};



/// Don't want to do merge rn. Lazy.
#[derive(Clone, Default, StructMergeRef)]
#[struct_merge_ref("crate::states::EntityState")]
pub struct EntityState {
    pub height: f32,
    pub half_width: f32,
    pub position: glam::Vec3A,
    pub velocity: glam::Vec3A,

    pub pitch: f32,
    pub yaw: f32,

    /// perhaps merge with is_collided_vertically.
    pub on_ground: bool,

    pub age: u64,
    pub is_in_water: bool,
    pub is_in_lava: bool,
    pub is_in_web: bool,

    pub is_collided_horizontally: bool,
    pub is_collided_vertically: bool,

    pub sneak_collision: bool,

    /// note: it doesn't make sense for self to be above 255.
    pub jump_ticks: u8,
    pub jump_queued: bool,

    /// might just use self.
    pub is_using_item: bool,
    /// potentially useless.
    pub is_using_offhand: bool,
    /// potentially useless.
    pub is_using_mainhand: bool,

    // abstraction of effects.
    // These are the only ones that affect movement.
    pub jump_boost: u16,
    pub speed: u16,
    pub slowness: u16,
    pub dolphins_grace: u16,
    pub slow_falling: u16,
    pub levitation: u16,
    pub depth_strider: u16,

    // pub attributes: any,
    // pub effects: Effect[],
    pub pose: PlayerPoses,

    pub control_states: ControlStateHandler,

    // assuming we always have this.
    // nodejs did not.
    // faithful behavor: Optional<PlayerAttributes>
    pub attributes: HashMap<String,  PlayerAttribute>


}

impl EntityState {

    /// original new function.
    pub fn new(height: f32, half_width: f32, position: glam::Vec3A, velocity: glam::Vec3A, on_ground: bool, yaw: f32, pitch: f32) -> Self {
        Self {
            height,
            half_width,
            position,
            velocity,
            on_ground,
            yaw,
            pitch,
            ..Default::default()
        }
    }

    /// new pose, use PlayerPoses for height and width.
    pub fn new_by_pose(pose: PlayerPoses, position: glam::Vec3A, velocity: glam::Vec3A, on_ground: bool, yaw: f32, pitch: f32) -> Self {
        let tmp = pose.get_info();
        Self {
            pose,
            height: tmp.height,
            half_width: tmp.width / 2.0,
            position,
            velocity,
            on_ground,
            yaw,
            pitch,
            ..Default::default()
        }
    }

    /// original bb func.
    pub fn get_bb(&self) -> AABB {
        let w = self.half_width;
        return AABB {
            minX: self.position.x - w,
            minY: self.position.y,
            minZ: self.position.z - w,
            maxX: self.position.x + w,
            maxY: self.position.y + self.height,
            maxZ: self.position.z + w,
        };
    }

    /// new bb func., use pose bb info.
    pub fn get_pose_bb(&self) -> AABB {
        let tmp = self.pose.get_info();
        let w = tmp.width / 2.0;
        return AABB {
            minX: self.position.x - w,
            minY: self.position.y,
            minZ: self.position.z - w,
            maxX: self.position.x + w,
            maxY: self.position.y + tmp.height,
            maxZ: self.position.z + w,
        };
    }
}
