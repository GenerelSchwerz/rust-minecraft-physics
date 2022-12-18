use std::collections::{HashMap, HashSet};

use crate::{
    calc::aabb::AABB,
    settings::{physics_settings, PlayerAttribute, PlayerAttributeModifier},
    states::physics_context::EntityPhysicsContext,
};

use super::{Block, World};

#[derive(Default)]
pub struct PrismarineSimulator {
    slime_block_id: u32,
    soulsand_id: u32,
    web_id: u32,
    honeyblock_id: u32,
    bubblecolumn_id: u32,
    ladder_id: u32,
    vine_id: u32,
    water_id: u32,
    lava_id: u32,
    water_like: HashSet<u32>,
    movement_speed_attribute: String,
    block_slipperiness: HashMap<u32, f32>,
}

fn glam_offset(org: &glam::Vec3A, x: f32, y: f32, z: f32) -> glam::Vec3A {
    glam::Vec3A::new(org.x + x, org.y + y, org.z + z)
}
fn glam_translate(mut org: glam::Vec3A, x: f32, y: f32, z: f32) -> glam::Vec3A {
    org.x += x;
    org.y += y;
    org.z += z;
    return org;
}

impl PrismarineSimulator {
    fn support_feature(_key: &str) -> bool {
        return false;
    }

    fn get_entity_bb(entity: &EntityPhysicsContext, pos: &glam::Vec3A) -> AABB {
        return entity.get_bb_at_pos_with_pose(&pos);
    }

    fn set_pos_to_bb_center_bottom(entity: &mut EntityPhysicsContext, bb: &AABB) {
        // println!("{} {:?}", entity.state.position, bb);

        let half_width = entity.get_half_width();
        entity.state.position.x = bb.min_x + half_width;
        entity.state.position.y = bb.min_y;
        entity.state.position.z = bb.min_z + half_width;
    }

    fn should_move_entity(entity: &EntityPhysicsContext) -> bool {
        return !((entity.state.is_collided_horizontally || entity.state.is_collided_vertically)
            && !entity.collision_behavior.affected_after_collision);
    }

    pub fn get_underlying_block_bbs(query_bb: &AABB, world: &impl World) -> Vec<AABB> {
        let mut surrounding_bbs = vec![];
        let q_bb_fl = query_bb.floored();
        let mut cursor = glam::Vec3A::new(q_bb_fl.min_x, q_bb_fl.min_y - 0.251, q_bb_fl.min_z);

        while cursor.z <= q_bb_fl.max_z {
            cursor.x = q_bb_fl.min_x;
            while cursor.x <= q_bb_fl.max_x {
                if let Some(block) = world.get_block(&cursor) {
                    let b_pos = block.position;
                    for shape in block.shapes {
                        let bb =
                            AABB::new(shape[0], shape[1], shape[2], shape[3], shape[4], shape[5])
                                .offset(b_pos.x, b_pos.y, b_pos.z);
                        surrounding_bbs.push(bb);
                    }
                }
                cursor.x += 1.0;
            }
            cursor.z += 1.0;
        }

        return surrounding_bbs;
    }

    pub fn get_surrounding_block_bbs(query_bb: &AABB, world: &impl World) -> Vec<AABB> {
        let mut surrounding_bbs = vec![];
        let q_bb_fl = query_bb.floored();
        let mut cursor = glam::Vec3A::new(q_bb_fl.min_x, q_bb_fl.min_y - 1.0, q_bb_fl.min_z);
        while cursor.y <= q_bb_fl.max_y {
            cursor.z = q_bb_fl.min_z;
            while cursor.z <= q_bb_fl.max_z {
                cursor.x = q_bb_fl.min_x;
                while cursor.x <= q_bb_fl.max_x {
                    if let Some(block) = world.get_block(&cursor) {
                        let b_pos = block.position;
                        for shape in block.shapes {
                            let bb = AABB::new(
                                shape[0], shape[1], shape[2], shape[3], shape[4], shape[5],
                            )
                            .offset(b_pos.x, b_pos.y, b_pos.z);
                            surrounding_bbs.push(bb);
                        }
                    }
                    cursor.x += 1.0;
                }
                cursor.z += 1.0;
            }
            cursor.y += 1.0;
        }
        return surrounding_bbs;
    }

    fn adjust_pos_height(
        &self,
        entity: &mut EntityPhysicsContext,
        pos: &mut glam::Vec3A,
        world: &impl World, /*prismarine-world*/
    ) {
        // println!("{} pos: {}", entity.state.position, pos);
        let player_bb = entity.get_bb_at_pos_with_pose(pos);
        let query_bb = player_bb.extend(0.0, -1.0, 0.0);
        let surrounding_bbs = Self::get_surrounding_block_bbs(&query_bb, world);

        let mut dy = -1.0;
        for block_bb in surrounding_bbs {
            dy = block_bb.compute_offset_y(&player_bb, dy);
        }
        pos.y += dy;
    }

    fn move_entity(
        &self,
        entity: &mut EntityPhysicsContext,
        mut dx: f32,
        mut dy: f32,
        mut dz: f32,
        world: &impl World, /*prismarine-world*/
    ) {
        if !Self::should_move_entity(entity) {
            entity.state.velocity.x = 0.0;
            entity.state.velocity.y = 0.0;
            entity.state.velocity.z = 0.0;
            // clear
            return;
        }

        if entity.state.is_in_web && !entity.entity_type.name.contains("arrow") {
            dx *= 0.25;
            dy *= 0.05;
            dz *= 0.25;
            entity.state.velocity.x = 0.0;
            entity.state.velocity.y = 0.0;
            entity.state.velocity.z = 0.0;
            entity.state.is_in_web = false;
        }

        let old_old_vel_x = dx; // was const
        let mut old_vel_x = dx;
        let old_vel_y = dy; // was const
        let mut old_vel_z = dz;
        let old_old_vel_z = dz; // was const

        if entity.use_controls && entity.state.control_states.sneak && entity.state.on_ground {
            let step = 0.05;

            // In the 3 loops bellow, y offset should be -1, but that doesnt reproduce vanilla behavior.
            while dx != 0.0
                && Self::get_surrounding_block_bbs(
                    &entity.get_current_bb_with_pose().offset(dx, 0.0, 0.0),
                    world,
                )
                .len()
                    == 0
            {
                if dx < step && dx >= -step {
                    dx = 0.0;
                } else if dx > 0.0 {
                    dx -= step;
                } else {
                    dx += step;
                }
                old_vel_x = dx;
            }

            while dz != 0.0
                && Self::get_surrounding_block_bbs(
                    &entity.get_current_bb_with_pose().offset(0.0, 0.0, dz),
                    world,
                )
                .len()
                    == 0
            {
                if dz < step && dz >= -step {
                    dz = 0.0;
                } else if dx > 0.0 {
                    dz -= step;
                } else {
                    dz += step;
                }
                old_vel_z = dz;
            }

            while dx != 0.0
                && dz != 0.0
                && Self::get_surrounding_block_bbs(
                    &entity.get_current_bb_with_pose().offset(dx, 0.0, dz),
                    world,
                )
                .len()
                    == 0
            {
                if dx < step && dx >= -step {
                    dx = 0.0;
                } else if dx > 0.0 {
                    dx -= step;
                } else {
                    dx += step;
                }

                if dz < step && dz >= -step {
                    dz = 0.0;
                } else if dx > 0.0 {
                    dz -= step;
                } else {
                    dz += step;
                }
                old_vel_x = dx;
                old_vel_z = dz;
            }
        }

        let mut player_bb = entity.get_current_bb_with_pose();
        let query_bb = player_bb.extend(dx, dy, dz);
        let surrounding_bbs = Self::get_surrounding_block_bbs(&query_bb, world);
        let old_bb = player_bb.clone();

        for block_bb in &surrounding_bbs {
            dy = block_bb.compute_offset_y(&player_bb, dy);
        }
        player_bb = player_bb.offset(0.0, dy, 0.0);

        for block_bb in &surrounding_bbs {
            dx = block_bb.compute_offset_x(&player_bb, dx);
        }
        player_bb = player_bb.offset(dx, 0.0, 0.0);

        for block_bb in &surrounding_bbs {
            dz = block_bb.compute_offset_z(&player_bb, dz);
        }
        player_bb = player_bb.offset(0.0, 0.0, dz);

        // Step on block if height < stepHeight
        if entity.step_height > 0.0
            && (entity.state.on_ground || (dy != old_vel_y && old_vel_y < 0.0))
            && (dx != old_vel_x || dz != old_vel_z)
        {
            let old_vel_x_col = dx;
            let old_vel_y_col = dy;
            let old_vel_z_col = dz;
            let old_bb_col = player_bb.clone();

            dy = entity.step_height;
            let query_bb = old_bb.extend(old_vel_x, dy, old_vel_z);
            let surrounding_bbs = Self::get_surrounding_block_bbs(&query_bb, world);

            let mut bb1 = old_bb.clone();
            let mut bb2 = old_bb.clone();
            let bb_xz = bb1.extend(dx, 0.0, dz);

            let mut dy1 = dy;
            let mut dy2 = dy;
            for block_bb in &surrounding_bbs {
                dy1 = block_bb.compute_offset_y(&bb_xz, dy1);
                dy2 = block_bb.compute_offset_y(&bb2, dy2);
            }
            bb1 = bb1.offset(0.0, dy1, 0.0);
            bb2 = bb2.offset(0.0, dy2, 0.0);

            let mut dx1 = old_vel_x;
            let mut dx2 = old_vel_x;
            for block_bb in &surrounding_bbs {
                dx1 = block_bb.compute_offset_x(&bb1, dx1);
                dx2 = block_bb.compute_offset_x(&bb2, dx2);
            }
            bb1 = bb1.offset(dx1, 0.0, 0.0);
            bb2 = bb2.offset(dx2, 0.0, 0.0);

            let mut dz1 = old_vel_z;
            let mut dz2 = old_vel_z;
            for block_bb in &surrounding_bbs {
                dz1 = block_bb.compute_offset_z(&bb1, dz1);
                dz2 = block_bb.compute_offset_z(&bb2, dz2);
            }
            bb1 = bb1.offset(0.0, 0.0, dz1);
            bb2 = bb2.offset(0.0, 0.0, dz2);

            let norm1 = dx1 * dx1 + dz1 * dz1;
            let norm2 = dx2 * dx2 + dz2 * dz2;

            if norm1 > norm2 {
                dx = dx1;
                dy = -dy1;
                dz = dz1;
                player_bb = bb1;
            } else {
                dx = dx2;
                dy = -dy2;
                dz = dz2;
                player_bb = bb2;
            }

            for block_bb in &surrounding_bbs {
                dy = block_bb.compute_offset_y(&player_bb, dy);
            }

            player_bb = player_bb.offset(0.0, dy, 0.0);

            if old_vel_x_col * old_vel_x_col + old_vel_z_col * old_vel_z_col >= dx * dx + dz * dz {
                dx = old_vel_x_col;
                dy = old_vel_y_col;
                dz = old_vel_z_col;
                player_bb = old_bb_col;
            }
        }

        // Update flags
        // up until this point, pos == entity.state.position
        Self::set_pos_to_bb_center_bottom(entity, &player_bb);

        // reassign to entity.state.position (we're deviating here).
        // this should still match though.

        entity.state.sneak_collision = dx != old_old_vel_x || dz != old_old_vel_z;
        entity.state.is_collided_horizontally = dx != old_vel_x || dz != old_vel_z;
        entity.state.is_collided_vertically = dy != old_vel_y;
        entity.state.on_ground = entity.state.is_collided_vertically && old_vel_y < 0.0;

        let block_at_feet = world.get_block(&glam::Vec3A::new(
            entity.state.position.x,
            entity.state.position.y - 0.2,
            entity.state.position.z,
        ));

        if dx != old_vel_x {
            entity.state.velocity.x = 0.0;
        }
        if dz != old_vel_z {
            entity.state.velocity.z = 0.0;
        }
        if dy != old_vel_y {
            if entity.collision_behavior.block_effects
                && block_at_feet.is_some()
                && block_at_feet.unwrap().b_type == self.slime_block_id
                && !entity.state.control_states.sneak
            {
                entity.state.velocity.y = -entity.state.velocity.y;
            } else {
                entity.state.velocity.y = 0.0;
            }
        }

        // Finally, apply block collisions (web, soulsand...)
        player_bb = player_bb.contract(0.001, 0.001, 0.001);
        let p_bb_fl = player_bb.floored();
        let mut cursor = glam::Vec3A::new(p_bb_fl.min_x, p_bb_fl.min_y, p_bb_fl.min_z);

        while cursor.y <= p_bb_fl.max_y {
            cursor.z = p_bb_fl.min_z;
            while cursor.z <= p_bb_fl.max_z {
                cursor.x = p_bb_fl.min_x;
                while cursor.x <= p_bb_fl.max_x {
                    if let Some(block) = world.get_block(&cursor) {
                        if entity.collision_behavior.block_effects
                            && Self::support_feature("velocityBlocksOnCollision")
                        {
                            if block.b_type == self.soulsand_id {
                                entity.state.velocity.x *= physics_settings::SOUL_SAND_SPEED;
                                entity.state.velocity.z *= physics_settings::SOUL_SAND_SPEED;
                            } else if block.b_type == self.honeyblock_id {
                                entity.state.velocity.x *= physics_settings::HONEY_BLOCK_SPEED;
                                entity.state.velocity.z *= physics_settings::HONEY_BLOCK_SPEED;
                            }
                        }
                        if block.b_type == self.web_id {
                            entity.state.is_in_web = true;
                        }
                        // no blockEffects check here, apparently all entities are affected by self.
                        else if block.b_type == self.bubblecolumn_id {
                            let down = block.metadata == 0; // uhhh. actually, this could be !0 which is true.
                            let above_block = world.get_block(&glam::Vec3A::new(
                                cursor.x,
                                cursor.y + 1.0,
                                cursor.z,
                            ));

                            let bubble_drag = if above_block.is_some_and(|b| b.b_type == 0) {
                                physics_settings::BUBBLE_COLUMN_SURFACE_DRAG
                            } else {
                                physics_settings::BUBBLE_COLUMN_DRAG
                            };

                            if down {
                                entity.state.velocity.y = bubble_drag
                                    .max_down
                                    .max(entity.state.velocity.y - bubble_drag.down);
                            } else {
                                entity.state.velocity.y = bubble_drag
                                    .max_up
                                    .max(entity.state.velocity.y + bubble_drag.up);
                            }
                        }
                    }

                    cursor.x += 1.0;
                }
                cursor.z += 1.0;
            }
            cursor.y += 1.0;
        }
        if entity.collision_behavior.block_effects && Self::support_feature("velocityBlocksOnTop") {
            let tmp = entity.state.position.clone().floor();
            if let Some(block_below) = world.get_block(&glam::Vec3A::new(tmp.x, tmp.y - 0.5, tmp.z))
            {
                if block_below.b_type == self.soulsand_id {
                    entity.state.velocity.x *= physics_settings::SOUL_SAND_SPEED;
                    entity.state.velocity.z *= physics_settings::SOUL_SAND_SPEED;
                } else if block_below.b_type == self.honeyblock_id {
                    entity.state.velocity.x *= physics_settings::HONEY_BLOCK_SPEED;
                    entity.state.velocity.z *= physics_settings::HONEY_BLOCK_SPEED;
                }
            }
        }
    }

    fn apply_heading(
        entity: &mut EntityPhysicsContext,
        mut strafe: f32,
        mut forward: f32,
        multiplier: f32,
    ) {
        if !Self::should_move_entity(entity) {
            return;
        }
        let mut speed = (strafe * strafe + forward * forward).sqrt();
        if speed < 0.01 {
            return;
        }

        speed = multiplier / speed.max(1.0);

        strafe *= speed;
        forward *= speed;

        let yaw = std::f32::consts::PI - entity.state.yaw;
        let (sin, cos) = yaw.sin_cos();

        if sin >= f32::EPSILON {
            entity.state.velocity.x += strafe * cos - forward * sin;
        }

        if cos >= f32::EPSILON {
            entity.state.velocity.z += forward * cos + strafe * sin;
        }
    }

    fn is_on_ladder(
        &self,
        pos: &glam::Vec3A,
        world: &impl World, /*prismarine-world*/
    ) -> bool {
        if let Some(block) = world.get_block(pos) {
            return block.b_type == self.ladder_id || block.b_type == self.vine_id;
        }

        return false;
    }

    // pub fn get_water_in_bb(

    pub fn does_not_collide(
        &self,
        entity: &EntityPhysicsContext,
        pos: &glam::Vec3A,
        world: &impl World, /*prismarine-world*/
    ) -> bool {
        let p_bb = Self::get_entity_bb(entity, pos);
        return !Self::get_surrounding_block_bbs(&p_bb, world)
            .iter()
            .any(|x| p_bb.intersects(x))
            && self.get_water_in_bb(&p_bb, world).len() == 0;
    }

    pub fn is_material_in_bb(
        query_bb: &AABB,
        b_type: u32,
        world: &impl World, /*prismarine-world*/
    ) -> bool {
        let q_bb_fl = query_bb.floored();
        let mut cursor = glam::Vec3A::new(q_bb_fl.min_x, q_bb_fl.min_y, q_bb_fl.min_z);

        while cursor.y <= q_bb_fl.max_y {
            cursor.z = q_bb_fl.min_z;
            while cursor.z <= q_bb_fl.max_z {
                cursor.x = q_bb_fl.min_x;
                while cursor.x <= q_bb_fl.max_x {
                    let block = world.get_block(&cursor);
                    if block.is_some_and(|b| b.b_type == b_type) {
                        return true;
                    }

                    cursor.x += 1.0;
                }
                cursor.z += 1.0;
            }
            cursor.y += 1.0;
        }

        return false;
    }

    fn get_water_in_bb(
        &self,
        query_bb: &AABB,
        world: &impl World, /*prismarine-world*/
    ) -> Vec<Block> {
        let mut water_blocks = vec![];
        let q_bb_fl = query_bb.floored();
        let mut cursor = glam::Vec3A::new(q_bb_fl.min_x, q_bb_fl.min_y - 1.0, q_bb_fl.min_z);
        while cursor.y <= q_bb_fl.max_y {
            cursor.z = q_bb_fl.min_z;
            while cursor.z <= q_bb_fl.max_z {
                cursor.x = q_bb_fl.min_x;
                while cursor.x <= q_bb_fl.max_x {
                    if let Some(block) = world.get_block(&cursor) {
                        if block.b_type == self.water_id
                            || self.water_like.contains(&block.b_type)
                            || block.get_properties().waterlogged
                        {
                            let water_level =
                                cursor.y + 1.0 - self.get_liquid_height_percent(&block);
                            if query_bb.max_y.ceil() >= water_level {
                                water_blocks.push(block);
                            }
                        }
                    }
                    cursor.x += 1.0;
                }
                cursor.z += 1.0;
            }
            cursor.y += 1.0;
        }

        return water_blocks;
    }

    fn get_liquid_height_percent(&self, block: &Block) -> f32 {
        return (self.get_rendered_depth(block) + 1.0) / 9.0;
    }

    fn get_rendered_depth(&self, block: &Block) -> f32 {
        if self.water_like.contains(&block.b_type) {
            return 0.0;
        }
        if block.get_properties().waterlogged {
            return 0.0;
        }
        if block.b_type != self.water_id {
            return -1.0;
        }
        let meta = block.metadata;
        return if meta >= 8 { 0.0 } else { meta as f32 };
    }

    fn get_flow(&self, block: &Block, world: &impl World /*prismarine-world*/) -> glam::Vec3A {
        let curlevel = self.get_rendered_depth(block);
        let mut flow = glam::Vec3A::splat(0.0);
        for [dx, dz] in [[0.0, 1.0], [-1.0, 0.0], [0.0, -1.0], [1.0, 0.0]] {
            if let Some(adj_block) = world.get_block(&glam_offset(&block.position, dx, 0.0, dz)) {
                let adj_level: f32 = self.get_rendered_depth(&adj_block);

                // if block is not water.
                if adj_level < 0.0 {
                    if adj_block.bounding_box != "empty" {
                        if let Some(adj_block) =
                            world.get_block(&glam_offset(&block.position, dx, -1.0, dz))
                        {
                            let adj_level = self.get_rendered_depth(&adj_block);
                            if adj_level >= 0.0 {
                                let f = adj_level - (curlevel - 8.0);
                                flow.x += dx * f;
                                flow.z += dz * f;
                            }
                        }
                    }

                // if block is some form of water.
                } else {
                    let f = adj_level - curlevel;
                    flow.x += dx * f;
                    flow.z += dz * f;
                }
            }
        }

        if block.metadata >= 8 {
            for [dx, dz] in [[0.0, 1.0], [-1.0, 0.0], [0.0, -1.0], [1.0, 0.0]] {
                {
                    let adj_block = world.get_block(&glam_offset(&block.position, dx, 0.0, dz));
                    let adj_up_block = world.get_block(&glam_offset(&block.position, dx, 1.0, dz));
                    if adj_block.is_some_and(|b| b.bounding_box != "empty")
                        || adj_up_block.is_some_and(|b| b.bounding_box != "empty")
                    {
                        flow = glam_translate(flow.normalize(), 0.0, -6.0, 0.0);
                    }
                }
            }
        }
        return flow.normalize();
    }

    fn is_in_water_apply_current(
        &self,
        bb: &AABB,
        vel: &mut glam::Vec3A,
        world: &impl World, /*prismarine-world*/
    ) -> bool {
        let mut acceleration = glam::Vec3A::splat(0.0);
        let water_blocks = self.get_water_in_bb(bb, world);
        let is_in_water = water_blocks.len() > 0;
        for block in water_blocks {
            let flow = self.get_flow(&block, world);
            acceleration += flow;
        }

        // note: this is .norm() or Math.sqrt(x*x + y*y +z*z);
        // not actually sure what length is but it should be that.
        let len = acceleration.length();
        if len > 0.0 {
            vel.x += (acceleration.x / len) * 0.014;
            vel.y += (acceleration.y / len) * 0.014;
            vel.z += (acceleration.z / len) * 0.014;
        }
        return is_in_water;
    }

    fn move_entity_with_heading(
        &self,
        entity: &mut EntityPhysicsContext,
        strafe: f32,
        forward: f32,
        world: &impl World, /*prismarine-world*/
    ) {
        if !Self::should_move_entity(entity) {
            entity.state.velocity.x = 0.0;
            entity.state.velocity.y = 0.0;
            entity.state.velocity.z = 0.0;
            return;
        }

        let _vel = entity.state.velocity;
        let pos = entity.state.position;

        let gravity_multiplier = if entity.state.velocity.y <= 0.0 && entity.state.slow_falling > 0
        {
            physics_settings::SLOW_FALLING
        } else {
            1.0
        };

        // Unsure how to handle this w/ other entities.
        if !entity.state.is_in_water && !entity.state.is_in_lava {
            let mut acceleration = physics_settings::AIRBORNE_ACCELERATION;
            let mut inertia = physics_settings::AIRBORNE_INERTIA;
            if let Some(block_under) = world.get_block(&glam::Vec3A::new(
                entity.state.velocity.x,
                entity.state.velocity.y - 1.0,
                entity.state.velocity.z,
            )) {
                if entity.state.on_ground {
                    let mut player_speed_attribute: PlayerAttribute;
                    if entity
                        .state
                        .attributes
                        .get(&self.movement_speed_attribute)
                        .is_some()
                    {
                        // Use server-side player attributes
                        player_speed_attribute = entity
                            .state
                            .attributes
                            .get(&self.movement_speed_attribute)
                            .unwrap()
                            .clone();
                    } else {
                        // Create an attribute if the player does not have it
                        //TODO: Generalize to all entities.
                        player_speed_attribute =
                            PlayerAttribute::create_attribute_value(physics_settings::PLAYER_SPEED);
                    }
                    // Client-side sprinting (don't rely on server-side sprinting)
                    // setSprinting in LivingEntity.java
                    //TODO: Generalize to all entities.
                    player_speed_attribute = PlayerAttribute::delete_attribute_modifier(
                        player_speed_attribute,
                        &physics_settings::SPRINTING_UUID,
                    ); // always delete sprinting (if it exists)
                    if entity.state.control_states.sprint {
                        if !PlayerAttribute::check_attribute_modifier(
                            &player_speed_attribute,
                            &physics_settings::SPRINTING_UUID,
                        ) {
                            player_speed_attribute = PlayerAttribute::add_attribute_modifier(
                                player_speed_attribute,
                                PlayerAttributeModifier {
                                    uuid: physics_settings::SPRINTING_UUID.to_string(),
                                    amount: physics_settings::SPRINT_SPEED,
                                    operation: 2,
                                },
                            );
                        }
                    }
                    // Calculate what the speed is (0.1 if no modification)
                    let attribute_speed =
                        PlayerAttribute::get_attribute_value(player_speed_attribute);

                    inertia = self
                        .block_slipperiness
                        .get(&block_under.b_type)
                        .unwrap_or(&physics_settings::DEFAULT_SLIPPERINESS)
                        * 0.91;
                    acceleration = attribute_speed * (0.1627714 / (inertia * inertia * inertia));
                    if acceleration < 0.0 {
                        acceleration = 0.0; // acceleration should not be negative
                    }
                }
            }

            Self::apply_heading(entity, strafe, forward, acceleration);

            if entity.collision_behavior.block_effects
                && self.is_on_ladder(&entity.state.position, world)
            {
                entity.state.velocity.x = (-physics_settings::LADDER_MAX_SPEED)
                    .min(entity.state.velocity.x)
                    .max(physics_settings::LADDER_MAX_SPEED);
                entity.state.velocity.z = (-physics_settings::LADDER_MAX_SPEED)
                    .min(entity.state.velocity.z)
                    .max(physics_settings::LADDER_MAX_SPEED);
                entity.state.velocity.y =
                    entity
                        .state
                        .velocity
                        .y
                        .max(if entity.state.control_states.sneak {
                            0.0
                        } else {
                            -physics_settings::LADDER_MAX_SPEED
                        });
            }

            self.move_entity(
                entity,
                entity.state.velocity.x,
                entity.state.velocity.y,
                entity.state.velocity.z,
                world,
            );

            if entity.collision_behavior.block_effects
                && self.is_on_ladder(&entity.state.position, world)
                && (entity.state.is_collided_horizontally
                    || (Self::support_feature("climbUsingJump")
                        && entity.state.control_states.jump))
            {
                entity.state.velocity.y = physics_settings::LADDER_CLIMB_SPEED; // climb ladder
            }

            // Not adding an additional function call. No point.
            if entity.gravity_then_drag {
                // Apply gravity, then air drag.
                if entity.state.levitation > 0 {
                    entity.state.velocity.y +=
                        (0.05 * entity.state.levitation as f32 - entity.state.velocity.y) * 0.2;
                } else {
                    entity.state.velocity.y -= entity.gravity * gravity_multiplier;
                }
                entity.state.velocity.y *= entity.airdrag;
            } else {
                // Apply airdrag, then gravity.
                entity.state.velocity.y *= entity.airdrag;
                if entity.state.levitation > 0 {
                    entity.state.velocity.y +=
                        (0.05 * entity.state.levitation as f32 - entity.state.velocity.y) * 0.2;
                } else {
                    entity.state.velocity.y -= entity.gravity * gravity_multiplier;
                }
            }

            entity.state.velocity.x *= inertia;
            entity.state.velocity.z *= inertia;
        } else {
            // Water / Lava movement
            let last_y = pos.y;
            let mut acceleration = physics_settings::LIQUID_ACCELERATION;
            let inertia = if entity.state.is_in_water {
                physics_settings::WATER_INERTIA
            } else {
                physics_settings::LAVA_INERTIA
            };
            let mut horizontal_inertia = inertia;

            if entity.state.is_in_water {
                let mut strider = entity.state.depth_strider.min(3) as f32;

                if !entity.state.on_ground {
                    strider *= 0.5; // originally * 0.5, not... too sure what this does. Swimming?
                }
                if strider > f32::EPSILON {
                    horizontal_inertia += ((0.546 - horizontal_inertia) * strider as f32) / 3.0;
                    acceleration += ((0.7 - acceleration) * strider as f32) / 3.0;
                }

                if entity.state.dolphins_grace > 0 {
                    horizontal_inertia = 0.96;
                }
            }

            Self::apply_heading(entity, strafe, forward, acceleration);
            self.move_entity(
                entity,
                entity.state.velocity.x,
                entity.state.velocity.y,
                entity.state.velocity.z,
                world,
            );
            if entity.gravity_then_drag {
                entity.state.velocity.y -= if entity.state.is_in_water {
                    entity.water_gravity
                } else {
                    entity.lava_gravity
                } * gravity_multiplier;
                entity.state.velocity.y *= inertia;
            } else {
                entity.state.velocity.y *= inertia;
                entity.state.velocity.y -= if entity.state.is_in_water {
                    entity.water_gravity
                } else {
                    entity.lava_gravity
                } * gravity_multiplier;
            }
            entity.state.velocity.x *= horizontal_inertia;
            entity.state.velocity.z *= horizontal_inertia;

            if entity.state.is_collided_horizontally
                && self.does_not_collide(
                    entity,
                    &glam::Vec3A::new(
                        pos.x + entity.state.velocity.x,
                        pos.y + entity.state.velocity.y + 0.6 - pos.y + last_y,
                        pos.z + entity.state.velocity.z,
                    ),
                    world,
                )
            {
                entity.state.velocity.y = physics_settings::OUT_OF_LIQUID_IMPULSE;
                // jump out of liquid
            }
        }
    }

    pub fn simulate(
        &self,
        mut entity: EntityPhysicsContext,
        world: &impl World, /*prismarine-world*/
    ) -> EntityPhysicsContext {
        if !Self::should_move_entity(&entity) {
            entity.state.velocity.x = 0.0;
            entity.state.velocity.y = 0.0;
            entity.state.velocity.z = 0.0;
            return entity;
        }

        let mut vel = entity.state.velocity;
        let pos = &entity.state.position;

        let water_bb = entity
            .get_bb_at_pos_with_pose(pos)
            .contract(0.001, 0.401, 0.001);
        let lava_bb = entity.get_bb_at_pos_with_pose(pos).contract(0.1, 0.4, 0.1);

        // assume that if we shouldn't move entity, isInWater and isInLava are already properly set.

        entity.state.is_in_water = self.is_in_water_apply_current(&water_bb, &mut vel, world);
        entity.state.is_in_lava = Self::is_material_in_bb(&lava_bb, self.lava_id, world);

        // Reset velocity component if it falls under the threshold
        if entity.state.velocity.x.abs() < physics_settings::NEGLIGEABLE_VELOCITY {
            entity.state.velocity.x = 0.0;
        }
        if entity.state.velocity.y.abs() < physics_settings::NEGLIGEABLE_VELOCITY {
            entity.state.velocity.y = 0.0;
        }
        if entity.state.velocity.z.abs() < physics_settings::NEGLIGEABLE_VELOCITY {
            entity.state.velocity.z = 0.0;
        }

        // Handle inputs
        if entity.use_controls {
            if entity.state.control_states.jump || entity.state.jump_queued {
                if entity.state.jump_ticks > 0 {
                    entity.state.jump_ticks -= 1;
                }
                if entity.state.is_in_water || entity.state.is_in_lava {
                    // originally Math.fround(0.4);
                    entity.state.velocity.y += 0.4000000059604645;
                } else if entity.state.on_ground && entity.state.jump_ticks == 0 {
                    let block_below = world.get_block(&glam_offset(
                        &entity.state.position.clone().floor(),
                        0.0,
                        -0.5,
                        0.0,
                    ));
                    // 0.41999998688697815 originally Math.fround(0.42)
                    entity.state.velocity.y = 0.41999998688697815
                        * (if block_below.is_some_and(|b| b.b_type == self.honeyblock_id) {
                            physics_settings::HONEY_BLOCK_JUMP_SPEED
                        } else {
                            1.0
                        });
                    if entity.state.jump_boost > 0 {
                        entity.state.velocity.y += 0.1 * entity.state.jump_boost as f32;
                    }
                    if entity.state.control_states.sprint {
                        let yaw = std::f32::consts::PI - entity.state.yaw;

                        entity.state.velocity.x -= yaw.sin() * 0.2;
                        entity.state.velocity.z += yaw.cos() * 0.2;
                    }
                    entity.state.jump_ticks = physics_settings::AUTO_JUMP_COOLDOWN;
                }
            } else {
                entity.state.jump_ticks = 0; // reset autojump cooldown
            }
            entity.state.jump_queued = false;

            let mut strafe = ((entity.state.control_states.right as u8 as f32)
                - (entity.state.control_states.left as u8 as f32))
                * 0.98;
            let mut forward = ((entity.state.control_states.forward as u8 as f32)
                - (entity.state.control_states.back as u8 as f32))
                * 0.98;

            if entity.state.control_states.sneak {
                strafe *= physics_settings::SNEAK_SPEED;
                forward *= physics_settings::SNEAK_SPEED;
                entity.state.control_states.sprint = false;
            }

            // this is not good enough. Different items slow you down differently.
            // patch this later.
            if entity.state.is_using_item {
                strafe *= physics_settings::USING_ITEM_SPEED;
                forward *= physics_settings::USING_ITEM_SPEED;
                entity.state.control_states.sprint = false;
            }

            // entity.state.velocity = vel;
            self.move_entity_with_heading(&mut entity, strafe, forward, world);
        } else {
            self.move_entity_with_heading(&mut entity, 0.0, 0.0, world);
        }

        return entity;
    }
}
