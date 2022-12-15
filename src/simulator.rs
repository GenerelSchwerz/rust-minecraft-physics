use std::collections::{HashMap, HashSet};

use glam::vec3a;

use crate::{
    calc::aabb::AABB,
    settings::{
        entity_physics_ctx::entity_physics_context::EntityPhysicsContext, physics_settings,
        PlayerAttribute, PlayerAttributeModifier,
    },
};

/// Temporary
#[derive(Default)]
pub struct Block {
    // original = type: u32
    boundingBox: String,
    metadata: u32,
    b_type: u32,
    position: glam::Vec3A,
    shapes: Vec<[f32; 6]>,
}

#[derive(Default)]
pub struct BlockProps {
    waterlogged: bool,
}

impl Block {
    pub fn getProperties(&self) -> BlockProps {
        BlockProps::default()
    }
}

/// Temporary
pub struct World {}

impl World {
    pub fn get_block(&self, pos: &glam::Vec3A) -> Option<Block> {
        Some(Block::default())
    }
}

pub struct Simulator {
    slime_block_id: u32,
    soulsand_id: u32,
    web_id: u32,
    honeyblock_id: u32,
    bubblecolumn_id: u32,
    ladder_id: u32,
    vine_id: u32,
    water_id: u32,
    water_like: HashSet<u32>,
    movement_speed_attribute: String,
    block_slipperiness: HashMap<u32, f32>,
}

fn glamOffset(org: &glam::Vec3A, x: f32, y: f32, z: f32) -> glam::Vec3A {
    glam::Vec3A::new(org.x + x, org.y + y, org.z + z)
}
fn glamTranslate(mut org: glam::Vec3A, x: f32, y: f32, z: f32) -> glam::Vec3A {
    org.x += x;
    org.y += y;
    org.z += z;
    return org;
}

impl Simulator {
    pub fn support_feature(key: &str) -> bool {
        return false;
    }

    pub fn get_entity_bb(entity: &EntityPhysicsContext, pos: &glam::Vec3A) -> AABB {
        return entity.get_bb_at_pos_with_pose(&pos);
    }

    pub fn set_pos_to_bb_center_bottom(
        entity: &EntityPhysicsContext,
        bb: &AABB,
        pos: &mut glam::Vec3A,
    ) {
        let half_width = entity.get_half_width();
        pos.x = bb.minX + half_width;
        pos.y = bb.minY;
        pos.z = bb.minZ + half_width;
    }

    pub fn should_move_entity(entity: &EntityPhysicsContext) -> bool {
        return !((entity.state.is_collided_horizontally || entity.state.is_collided_vertically)
            && !entity.collision_behavior.affected_after_collision);
    }

    pub fn get_underlying_block_bbs(query_bb: &AABB, world: &World) -> Vec<AABB> {
        let mut surrounding_bbs = vec![];
        let q_bb_fl = query_bb.floored();
        let mut cursor = glam::Vec3A::new(q_bb_fl.minX, q_bb_fl.minY - 0.251, q_bb_fl.minZ);

        while cursor.z <= q_bb_fl.maxZ {
            while cursor.x <= q_bb_fl.maxX {
                if let Some(block) = world.get_block(&cursor) {
                    let b_pos = block.position;
                    for shape in block.shapes {
                        let bb =
                            AABB::new(shape[0], shape[1], shape[2], shape[3], shape[4], shape[5]);
                        bb.offset(b_pos.x, b_pos.y, b_pos.z);
                        surrounding_bbs.push(bb);
                    }
                }
                cursor.x += 1.0;
            }
            cursor.z += 1.0;
        }

        return surrounding_bbs;
    }

    pub fn get_surrounding_block_bbs(query_bb: &AABB, world: &World) -> Vec<AABB> {
        let mut surrounding_bbs = vec![];
        let q_bb_fl = query_bb.floored();
        let mut cursor = glam::Vec3A::new(q_bb_fl.minX, q_bb_fl.minY - 1.0, q_bb_fl.minZ);
        while cursor.y <= q_bb_fl.maxY {
            while cursor.z <= q_bb_fl.maxZ {
                while cursor.x <= q_bb_fl.maxX {
                    if let Some(block) = world.get_block(&cursor) {
                        let b_pos = block.position;
                        for shape in block.shapes {
                            let bb = AABB::new(
                                shape[0], shape[1], shape[2], shape[3], shape[4], shape[5],
                            );
                            bb.offset(b_pos.x, b_pos.y, b_pos.z);
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

    pub fn adjust_pos_height(
        &self,
        entity: &mut EntityPhysicsContext,
        pos: &mut glam::Vec3A,
        world: &World, /*prismarine-world*/
    ) {
        let player_bb = Self::get_entity_bb(entity, &pos);
        let query_bb = player_bb.extended(0.0, -1.0, 0.0);
        let surrounding_bbs = Self::get_surrounding_block_bbs(&query_bb, world);

        let mut dy = -1.0;
        for block_bb in surrounding_bbs {
            dy = block_bb.compute_offset_y(&player_bb, dy);
        }
        pos.y += dy;
    }

    pub fn moveEntity(
        &self,
        entity: &mut EntityPhysicsContext,
        mut dx: f32,
        mut dy: f32,
        mut dz: f32,
        world: &World, /*prismarine-world*/
    ) {
        if !Self::should_move_entity(entity) {
            entity.velocity = glam::Vec3A::splat(0.0); // clear
            return;
        }

        let mut pos = entity.position;

        if entity.state.is_in_web && !entity.entity_type.name.contains("arrow") {
            dx *= 0.25;
            dy *= 0.05;
            dz *= 0.25;
            entity.velocity.x = 0.0;
            entity.velocity.y = 0.0;
            entity.velocity.z = 0.0;
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
                    &Self::get_entity_bb(entity, &pos).offset(dx, 0.0, 0.0),
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
                    &Self::get_entity_bb(entity, &pos).offset(0.0, 0.0, dz),
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
                    &Self::get_entity_bb(entity, &pos).offset(dx, 0.0, dz),
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

        let mut player_bb = Self::get_entity_bb(entity, &pos);
        let query_bb = player_bb.extended(dx, dy, dz);
        let surrounding_bbs = Self::get_surrounding_block_bbs(&query_bb, world);
        let old_bb = player_bb.clone();

        for block_bb in &surrounding_bbs {
            dy = block_bb.compute_offset_y(&player_bb, dy);
        }
        player_bb.offset(0.0, dy, 0.0);

        for block_bb in &surrounding_bbs {
            dx = block_bb.compute_offset_x(&player_bb, dx);
        }
        player_bb.offset(dx, 0.0, 0.0);

        for block_bb in &surrounding_bbs {
            dz = block_bb.compute_offset_z(&player_bb, dz);
        }
        player_bb.offset(0.0, 0.0, dz);

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
            let query_bb = old_bb.extended(old_vel_x, dy, old_vel_z);
            let surrounding_bbs = Self::get_surrounding_block_bbs(&query_bb, world);

            let bb1 = old_bb.clone();
            let bb2 = old_bb.clone();
            let bb_xz = bb1.extended(dx, 0.0, dz);

            let mut dy1 = dy;
            let mut dy2 = dy;
            for block_bb in &surrounding_bbs {
                dy1 = block_bb.compute_offset_y(&bb_xz, dy1);
                dy2 = block_bb.compute_offset_y(&bb2, dy2);
            }
            bb1.offset(0.0, dy1, 0.0);
            bb2.offset(0.0, dy2, 0.0);

            let mut dx1 = old_vel_x;
            let mut dx2 = old_vel_x;
            for block_bb in &surrounding_bbs {
                dx1 = block_bb.compute_offset_x(&bb1, dx1);
                dx2 = block_bb.compute_offset_x(&bb2, dx2);
            }
            bb1.offset(dx1, 0.0, 0.0);
            bb2.offset(dx2, 0.0, 0.0);

            let mut dz1 = old_vel_z;
            let mut dz2 = old_vel_z;
            for block_bb in &surrounding_bbs {
                dz1 = block_bb.compute_offset_z(&bb1, dz1);
                dz2 = block_bb.compute_offset_z(&bb2, dz2);
            }
            bb1.offset(0.0, 0.0, dz1);
            bb2.offset(0.0, 0.0, dz2);

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
            player_bb.offset(0.0, dy, 0.0);

            if old_vel_x_col * old_vel_x_col + old_vel_z_col * old_vel_z_col >= dx * dx + dz * dz {
                dx = old_vel_x_col;
                dy = old_vel_y_col;
                dz = old_vel_z_col;
                player_bb = old_bb_col;
            }
        }

        // Update flags
        // up until this point, pos == entity.position
        Self::set_pos_to_bb_center_bottom(entity, &player_bb, &mut pos);

        // reassign to entity.position (we're deviating here).
        // this should still match though.

        entity.position = pos;

        entity.state.sneak_collision = dx != old_old_vel_x || dz != old_old_vel_z;
        entity.state.is_collided_horizontally = dx != old_vel_x || dz != old_vel_z;
        entity.state.is_collided_vertically = dy != old_vel_y;
        entity.state.on_ground = entity.state.is_collided_vertically && old_vel_y < 0.0;

        let block_at_feet = world.get_block(&glam::Vec3A::new(pos.x, pos.y - 0.2, pos.z));

        if dx != old_vel_x {
            entity.velocity.x = 0.0;
        }
        if dz != old_vel_z {
            entity.velocity.z = 0.0;
        }
        if dy != old_vel_y {
            if entity.collision_behavior.block_effects
                && block_at_feet.is_some()
                && block_at_feet.unwrap().b_type == self.slime_block_id
                && !entity.state.control_states.sneak
            {
                entity.velocity.y = -entity.velocity.y;
            } else {
                entity.velocity.y = 0.0;
            }
        }

        // Finally, apply block collisions (web, soulsand...)
        player_bb.contract(0.001, 0.001, 0.001);
        let p_bb_fl = player_bb.floored();
        let mut cursor = glam::Vec3A::new(p_bb_fl.minX, p_bb_fl.minY, p_bb_fl.minZ);

        while cursor.y <= p_bb_fl.maxY {
            while cursor.z <= p_bb_fl.maxZ {
                while cursor.x <= p_bb_fl.maxX {
                    if let Some(block) = world.get_block(&cursor) {
                        if entity.collision_behavior.block_effects
                            && Self::support_feature("velocityBlocksOnCollision")
                        {
                            if block.b_type == self.soulsand_id {
                                entity.velocity.x *= physics_settings::soulsandSpeed;
                                entity.velocity.z *= physics_settings::soulsandSpeed;
                            } else if block.b_type == self.honeyblock_id {
                                entity.velocity.x *= physics_settings::honeyblockSpeed;
                                entity.velocity.z *= physics_settings::honeyblockSpeed;
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
                                physics_settings::bubbleColumnSurfaceDrag
                            } else {
                                physics_settings::bubbleColumnDrag
                            };

                            if down {
                                entity.velocity.y = bubble_drag
                                    .max_down
                                    .max(entity.velocity.y - bubble_drag.down);
                            } else {
                                entity.velocity.y =
                                    bubble_drag.max_up.max(entity.velocity.y + bubble_drag.up);
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
            let tmp = entity.position.clone().floor();
            if let Some(block_below) = world.get_block(&glam::Vec3A::new(tmp.x, tmp.y - 0.5, tmp.z))
            {
                if block_below.b_type == self.soulsand_id {
                    entity.velocity.x *= physics_settings::soulsandSpeed;
                    entity.velocity.z *= physics_settings::soulsandSpeed;
                } else if block_below.b_type == self.honeyblock_id {
                    entity.velocity.x *= physics_settings::honeyblockSpeed;
                    entity.velocity.z *= physics_settings::honeyblockSpeed;
                }
            }
        }
    }

    pub fn apply_heading(
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

        entity.velocity.x += strafe * cos - forward * sin;
        entity.velocity.z += forward * cos + strafe * sin;
    }

    pub fn isOnLadder(&self, pos: &glam::Vec3A, world: &World /*prismarine-world*/) -> bool {
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
        world: &World, /*prismarine-world*/
    ) -> bool {
        let p_bb = Self::get_entity_bb(entity, pos);
        return !Self::get_surrounding_block_bbs(&p_bb, world)
            .iter()
            .any(|x| p_bb.intersects(x))
            && self.get_water_in_bb(&p_bb, world).len() == 0;
    }

    pub fn get_water_in_bb(
        &self,
        query_bb: &AABB,
        world: &World, /*prismarine-world*/
    ) -> Vec<Block> {
        let mut water_blocks = vec![];
        let q_bb_fl = query_bb.floored();
        let mut cursor = glam::Vec3A::new(q_bb_fl.minX, q_bb_fl.minY - 1.0, q_bb_fl.minZ);
        while cursor.y <= q_bb_fl.maxY {
            while cursor.z <= q_bb_fl.maxZ {
                while cursor.x <= q_bb_fl.maxX {
                    if let Some(block) = world.get_block(&cursor) {
                        if block.b_type == self.water_id
                            || self.water_like.contains(&block.b_type)
                            || block.getProperties().waterlogged
                        {
                            let water_level =
                                cursor.y + 1.0 - self.get_liquid_height_percent(&block);
                            if query_bb.maxY.ceil() >= water_level {
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

    pub fn get_liquid_height_percent(&self, block: &Block) -> f32 {
        return (self.get_rendered_depth(block) + 1.0) / 9.0;
    }

    pub fn get_rendered_depth(&self, block: &Block) -> f32 {
        if self.water_like.contains(&block.b_type) {
            return 0.0;
        }
        if block.getProperties().waterlogged {
            return 0.0;
        }
        if block.b_type != self.water_id {
            return -1.0;
        }
        let meta = block.metadata;
        return if meta >= 8 { 0.0 } else { meta as f32 };
    }

    pub fn get_flow(&self, block: &Block, world: &World /*prismarine-world*/) -> glam::Vec3A {
        let curlevel = self.get_rendered_depth(block);
        let mut flow = glam::Vec3A::splat(0.0);
        for [dx, dz] in [[0.0, 1.0], [-1.0, 0.0], [0.0, -1.0], [1.0, 0.0]] {
            if let Some(adj_block) = world.get_block(&glamOffset(&block.position, dx, 0.0, dz)) {
                let adj_level: f32 = self.get_rendered_depth(&adj_block);

                // if block is not water.
                if adj_level < 0.0 {
                    if adj_block.boundingBox != "empty" {
                        if let Some(adj_block) =
                            world.get_block(&glamOffset(&block.position, dx, -1.0, dz))
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
                    let adj_block = world.get_block(&glamOffset(&block.position, dx, 0.0, dz));
                    let adj_up_block = world.get_block(&glamOffset(&block.position, dx, 1.0, dz));
                    if adj_block.is_some_and(|b| b.boundingBox != "empty")
                        || adj_up_block.is_some_and(|b| b.boundingBox != "empty")
                    {
                        flow = glamTranslate(flow.normalize(), 0.0, -6.0, 0.0);
                    }
                }
            }
        }
        return flow.normalize();
    }

    pub fn is_in_water_apply_current(
        &self,
        bb: &AABB,
        vel: &mut glam::Vec3,
        world: &World, /*prismarine-world*/
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

    pub fn move_entity_with_heading(
        &self,
        entity: &mut EntityPhysicsContext,
        strafe: f32,
        forward: f32,
        world: &World, /*prismarine-world*/
    ) {
        if !Self::should_move_entity(entity) {
            entity.velocity = glam::Vec3A::splat(0.0);
            return;
        }

        let gravity_multiplier = if entity.velocity.y <= 0.0 && entity.state.slow_falling > 0 {
            physics_settings::slowFalling
        } else {
            1.0
        };

        // Unsure how to handle this w/ other entities.
        if !entity.state.is_in_water && !entity.state.is_in_lava {
            let mut acceleration = physics_settings::airborneAcceleration;
            let mut inertia = physics_settings::airborneInertia;
            if let Some(block_under) = world.get_block(&vec3a(
                entity.position.x,
                entity.position.y - 1.0,
                entity.position.z,
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
                            PlayerAttribute::createAttributeValue(physics_settings::playerSpeed);
                    }
                    // Client-side sprinting (don't rely on server-side sprinting)
                    // setSprinting in LivingEntity.java
                    //TODO: Generalize to all entities.
                    player_speed_attribute = PlayerAttribute::deleteAttributeModifier(
                        player_speed_attribute,
                        &physics_settings::sprintingUUID,
                    ); // always delete sprinting (if it exists)
                    if entity.state.control_states.sprint {
                        if !PlayerAttribute::checkAttributeModifier(
                            &player_speed_attribute,
                            &physics_settings::sprintingUUID,
                        ) {
                            player_speed_attribute = PlayerAttribute::addAttributeModifier(
                                player_speed_attribute,
                                PlayerAttributeModifier {
                                    uuid: physics_settings::sprintingUUID.to_string(),
                                    amount: physics_settings::sprintSpeed,
                                    operation: 2,
                                },
                            );
                        }
                    }
                    // Calculate what the speed is (0.1 if no modification)
                    let attribute_speed =
                        PlayerAttribute::getAttributeValue(player_speed_attribute);

                    inertia = self
                        .block_slipperiness
                        .get(&block_under.b_type)
                        .unwrap_or(&physics_settings::defaultSlipperiness)
                        * 0.91;
                    acceleration = attribute_speed * (0.1627714 / (inertia * inertia * inertia));
                    if acceleration < 0.0 {
                        acceleration = 0.0; // acceleration should not be negative
                    }
                }
            }

            Self::apply_heading(entity, strafe, forward, acceleration);

            if (entity.collision_behavior.block_effects && self.isOnLadder(&entity.position, world))
            {
                entity.velocity.x = (-physics_settings::ladderMaxSpeed)
                    .min(entity.velocity.x)
                    .max(physics_settings::ladderMaxSpeed);
                entity.velocity.z = (-physics_settings::ladderMaxSpeed)
                    .min(entity.velocity.x)
                    .max(physics_settings::ladderMaxSpeed);
                entity.velocity.y = entity.velocity.y.max(if entity.state.control_states.sneak {
                    0.0
                } else {
                    -physics_settings::ladderMaxSpeed
                });
            }

            self.moveEntity(
                entity,
                entity.velocity.x,
                entity.velocity.y,
                entity.velocity.z,
                world,
            );

            if entity.collision_behavior.block_effects
                && self.isOnLadder(&entity.position, world)
                && (entity.state.is_collided_horizontally
                    || (Self::support_feature("climbUsingJump")
                        && entity.state.control_states.jump))
            {
                entity.velocity.y = physics_settings::ladderClimbSpeed; // climb ladder
            }

            // Not adding an additional function call. No point.
            if entity.gravity_then_drag {
                // Apply gravity, then air drag.
                if entity.state.levitation > 0 {
                    entity.velocity.y +=
                        (0.05 * entity.state.levitation as f32 - entity.velocity.y) * 0.2;
                } else {
                    entity.velocity.y -= entity.gravity * gravity_multiplier;
                }
                entity.velocity.y *= entity.airdrag;
            } else {
                // Apply airdrag, then gravity.
                entity.velocity.y *= entity.airdrag;
                if entity.state.levitation > 0 {
                    entity.velocity.y +=
                        (0.05 * entity.state.levitation as f32 - entity.velocity.y) * 0.2;
                } else {
                    entity.velocity.y -= entity.gravity * gravity_multiplier;
                }
            }

            entity.velocity.x *= inertia;
            entity.velocity.z *= inertia;
        } else {
            // Water / Lava movement
            let last_y = entity.position.y;
            let mut acceleration = physics_settings::liquidAcceleration;
            let inertia = if entity.state.is_in_water {
                physics_settings::waterInertia
            } else {
                physics_settings::lavaInertia
            };
            let mut horizontal_inertia = inertia;

            if entity.state.is_in_water {
                let mut strider = entity.state.depth_strider.min(3);
                if !entity.state.on_ground {
                    strider /= 2; // originally * 0.5, not... too sure what this does. Swimming?
                }
                if strider > 0 {
                    horizontal_inertia += ((0.546 - horizontal_inertia) * strider as f32) / 3.0;
                    acceleration += ((0.7 - acceleration) * strider as f32) / 3.0;
                }

                if entity.state.dolphins_grace > 0 {
                    horizontal_inertia = 0.96;
                }
            }

            Self::apply_heading(entity, strafe, forward, acceleration);
            self.moveEntity(
                entity,
                entity.velocity.x,
                entity.velocity.y,
                entity.velocity.z,
                world,
            );
            if entity.gravity_then_drag {
                entity.velocity.y -= if entity.state.is_in_water {
                    entity.water_gravity
                } else {
                    entity.lava_gravity
                } * gravity_multiplier;
                entity.velocity.y *= inertia;
            } else {
                entity.velocity.y *= inertia;
                entity.velocity.y -= if entity.state.is_in_water {
                    entity.water_gravity
                } else {
                    entity.lava_gravity
                } * gravity_multiplier;
            }
            entity.velocity.x *= horizontal_inertia;
            entity.velocity.z *= horizontal_inertia;

            if entity.state.is_collided_horizontally
                && self.does_not_collide(
                    entity,
                    &glam::Vec3A::new(
                        entity.position.x + entity.velocity.x,
                        entity.velocity.y + 0.6 + last_y,
                        entity.position.z + entity.velocity.z,
                    ),
                    world,
                )
            {
                entity.velocity.y = physics_settings::outOfLiquidImpulse; // jump out of liquid
            }
        }
    }
}
