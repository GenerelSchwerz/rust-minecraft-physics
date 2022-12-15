

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

// export class EPhysicsCtx {
//     public static loadData: (data: md.IndexedData) => void = load;
//     public static entityConstructor: new (id: number) => Entity;
//     public static mcData: md.IndexedData;
//     public static entityData: md.IndexedData["entitiesByName"];
//     public static mobData: md.IndexedData["mobs"];

//     /**
//      * From minecraft's Player.java file.
//      */
//     public static readonly playerPoseContext: PlayerPoseContext = {
//         0: { width: 0.6, height: 1.8 },
//         1: { width: 0.2, height: 0.2 },
//         2: { width: 0.6, height: 0.6 },
//         3: { width: 0.6, height: 0.6 },
//         4: { width: 0.6, height: 0.6 },
//         5: { width: 0.6, height: 0.6 },
//         6: { width: 0.6, height: 1.5 },
//         7: { width: 0.2, height: 0.2 },
//     };

//     public readonly position: Vec3;
//     public readonly velocity: Vec3;

//     public readonly stepHeight: number = 0;

//     public readonly gravity: number = 0.0;
//     public readonly waterGravity: number;
//     public readonly lavaGravity: number;

//     public readonly airdrag: number = Math.fround(1 - 0.0);
//     public readonly gravityThenDrag: boolean = false;
//     public readonly useControls: boolean = false;

//     public readonly collisionBehavior: CollisionContext = {
//         blockEffects: false,
//         affectedAfterCollision: true,
//     };

//     constructor(public ctx: IPhysics, public pose: PlayerPoses, public readonly state: EntityState, public readonly entityType: md.Entity) {

//         self.position = state.position;
//         self.velocity = state.velocity;

//         if (entityType.type === "player" || !!EPhysicsCtx.mobData[entityType.id]) {
//             self.gravity = 0.08;
//             self.airdrag = Math.fround(1 - 0.02);
//             self.gravityThenDrag = true;
//             self.useControls = true;
//             self.stepHeight = entityType.type === "player" ? 0.6 : 1.0;
//             self.collisionBehavior = {
//                 blockEffects: true,
//                 affectedAfterCollision: true,
//             };
//         }

//         if (entityType.name?.includes("experience_bottle")) {
//             self.gravity = 0.06;
//             self.airdrag = Math.fround(1 - 0.01);
//         }
//         if (entityType.name?.includes("spit")) {
//             self.gravity = 0.06;
//             self.airdrag = Math.fround(1 - 0.01);
//         }
//         switch (entityType.type) {
//             case "water_creature":
//             case "animal":
//             case "hostile":
//             case "mob":
//                 self.gravity = 0.08;
//                 self.airdrag = Math.fround(1 - 0.02);
//                 self.gravityThenDrag = true;
//                 self.useControls = true;
//                 self.stepHeight = 1.0;
//                 self.collisionBehavior = {
//                     blockEffects: true,
//                     affectedAfterCollision: true,
//                 };
//             case "projectile":
//                 self.gravity = 0.03;
//                 self.airdrag = Math.fround(1 - 0.01);
//                 self.collisionBehavior = {
//                     blockEffects: false,
//                     affectedAfterCollision: false,
//                 };
//             case "orb":
//                 self.gravity = 0.03;
//                 self.airdrag = Math.fround(1 - 0.02);
//                 self.collisionBehavior = {
//                     blockEffects: false,
//                     affectedAfterCollision: true,
//                 };
//             case "other":
//                 if (entityType.name?.includes("minecart")) {
//                     self.gravity = 0.04;
//                     self.airdrag = Math.fround(1 - 0.05);
//                     self.collisionBehavior = {
//                         blockEffects: false,
//                         affectedAfterCollision: true,
//                     };
//                 } else if (entityType.name?.includes("block") || entityType.name?.includes("tnt")) {
//                     self.gravity = 0.04;
//                     self.airdrag = Math.fround(1 - 0.02);
//                     self.collisionBehavior = {
//                         blockEffects: false,
//                         affectedAfterCollision: true,
//                     };
//                 } else if (entityType.name?.includes("boat")) {
//                     self.gravity = 0.04;
//                     self.airdrag = Math.fround(1 - 0);
//                     self.collisionBehavior = {
//                         blockEffects: false,
//                         affectedAfterCollision: true,
//                     };
//                 } else if (
//                     entityType.name?.includes("egg") ||
//                     entityType.name?.includes("snowball") ||
//                     entityType.name?.includes("potion") ||
//                     entityType.name?.includes("pearl")
//                 ) {
//                     self.gravity = 0.03;
//                     self.airdrag = Math.fround(1 - 0.01);
//                     self.collisionBehavior = {
//                         blockEffects: false,
//                         affectedAfterCollision: false,
//                     };
//                 } else if (entityType.name?.includes("orb")) {
//                     self.gravity = 0.03;
//                     self.airdrag = Math.fround(1 - 0.02);
//                     self.collisionBehavior = {
//                         blockEffects: false,
//                         affectedAfterCollision: true,
//                     };
//                 } else if (entityType.name?.includes("bobber")) {
//                     self.gravity = 0.03;
//                     self.airdrag = Math.fround(1 - 0.08);
//                     self.collisionBehavior = {
//                         blockEffects: false,
//                         affectedAfterCollision: true,
//                     };
//                 } else if (entityType.name?.includes("spit")) {
//                     self.gravity = 0.06;
//                     self.airdrag = Math.fround(1 - 0.01);
//                     self.collisionBehavior = {
//                         blockEffects: false,
//                         affectedAfterCollision: true,
//                     };
//                 } else if (entityType.name?.includes("arrow") || entityType.name?.includes("trident")) {
//                     self.gravity = 0.05;
//                     self.airdrag = Math.fround(1 - 0.01);
//                     self.collisionBehavior = {
//                         blockEffects: false,
//                         affectedAfterCollision: false,
//                     };
//                 } else if (entityType.name?.includes("fireball") || entityType.name?.includes("skull")) {
//                     self.gravity = 0.0;
//                     self.airdrag = Math.fround(1 - 0.05);
//                     self.gravityThenDrag = true;
//                     self.collisionBehavior = {
//                         blockEffects: false,
//                         affectedAfterCollision: false,
//                     };
//                 }
//         }

//         if (ctx.supportFeature("independentLiquidGravity")) {
//             self.waterGravity = 0.02;
//             self.lavaGravity = 0.02;
//         } else if (ctx.supportFeature("proportionalLiquidGravity")) {
//             self.waterGravity = self.gravity / 16;
//             self.lavaGravity = self.gravity / 4;
//         } else {
//             self.waterGravity = 0.005;
//             self.lavaGravity = 0.02;
//         }
//     }

//     public static FROM_ENTITY(ctx: IPhysics, entity: Entity) {
//         return new EPhysicsCtx(ctx, getPose(entity), EntityState.CREATE_FROM_ENTITY(ctx, entity), EPhysicsCtx.entityData[entity.name!]);
//     }

//     public static FROM_ENTITY_TYPE(ctx: IPhysics, entityType: md.Entity, options: Partial<Entity> = {}) {
//         const newE = applyMdToNewEntity(EPhysicsCtx, entityType, options);
//         return new EPhysicsCtx(ctx, PlayerPoses.STANDING, EntityState.CREATE_FROM_ENTITY(ctx, newE), entityType);
//     }

//     public static FROM_ENTITY_STATE(ctx: IPhysics, entityState: EntityState, entityType: md.Entity) {
//         return new EPhysicsCtx(ctx, entityState.pose, entityState, entityType);
//     }

//     public clone() {
//         return new EPhysicsCtx(self.ctx, self.state.pose, self.state.clone(), self.entityType);
//     }

//     public get height(): number {
//         if (self.entityType.type === "player") {
//             return EPhysicsCtx.playerPoseContext[self.pose].height;
//         }
//         return self.entityType.height ?? 0;
//     }

//     public get width(): number {
//         if (self.entityType.type === "player") {
//             return EPhysicsCtx.playerPoseContext[self.pose].width;
//         }
//         return self.entityType.width ?? 0;
//     }

//     public getHalfWidth(): number {
//         return self.width / 2;
//     }

//     public getCurrentBBWithPose(): AABB {
//         const halfWidth = self.getHalfWidth();
//         return new AABB(
//             self.position.x - halfWidth,
//             self.position.y,
//             self.position.z - halfWidth,
//             self.position.x + halfWidth,
//             self.position.y + self.height,
//             self.position.z + halfWidth
//         );
//     }

//     public getBBWithPose(position: { x: number; y: number; z: number }): AABB {
//         const halfWidth = self.getHalfWidth();
//         return new AABB(
//             position.x - halfWidth,
//             position.y,
//             position.z - halfWidth,
//             position.x + halfWidth,
//             position.y + self.height,
//             position.z + halfWidth
//         );
//     }

//     public getBB(position: { x: number; y: number; z: number }): AABB {
//         const halfWidth = self.entityType.width ? self.entityType.width / 2 : 0;
//         return new AABB(
//             position.x - halfWidth,
//             position.y,
//             position.z - halfWidth,
//             position.x + halfWidth,
//             position.y + (self.entityType.height ?? 0),
//             position.z + halfWidth
//         );
//     }
// }
