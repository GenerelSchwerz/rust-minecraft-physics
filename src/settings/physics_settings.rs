
pub struct BubbleColumnInfo {
    pub(crate) down: f32,
    pub(crate) max_down: f32,
    pub(crate) up: f32,
    pub(crate) max_up: f32,
}

pub const yawSpeed: f32 = 3.0;
pub const pitchSpeed: f32 = 3.0;
pub const playerSpeed: f32 = 0.1;
pub const negligeableVelocity: f32 = 0.003; // actually 0.005 for 1.8; but seems fine
pub const soulsandSpeed: f32 = 0.4;
pub const honeyblockSpeed: f32 = 0.4;
pub const honeyblockJumpSpeed: f32 = 0.4;
pub const ladderMaxSpeed: f32 = 0.15;
pub const ladderClimbSpeed: f32 = 0.2;
pub const waterInertia: f32 = 0.8;
pub const lavaInertia: f32 = 0.5;
pub const liquidAcceleration: f32 = 0.02;
pub const defaultSlipperiness: f32 = 0.6;
pub const outOfLiquidImpulse: f32 = 0.3;
pub const autojumpCooldown: u8 = 10; // ticks (0.5s)
pub const bubbleColumnSurfaceDrag: BubbleColumnInfo = BubbleColumnInfo {
    down: 0.03,
    max_down: -0.9,
    up: 0.1,
    max_up: 1.8,
};
pub const bubbleColumnDrag: BubbleColumnInfo = BubbleColumnInfo {
    down: 0.03,
    max_down: -0.3,
    up: 0.06,
    max_up: 0.7,
};
pub const slowFalling: f32 = 0.125;
pub const sprintingUUID: &str =  "662a6b8d-da3e-4c1c-8813-96ea6097278d"; // SPEED_MODIFIER_SPRINTING_UUID is from LivingEntity.java
pub const jumpHeight: f32 = 0.41999998688697815; // 0.41999998688697815 | used to be Math.fround(0.42)

pub const airborneInertia: f32 = 0.91;
pub const airborneAcceleration: f32 = 0.02;

pub const sprintSpeed: f32 = 0.30000001192092896; // 0.30000001192092896 | used to be Math.fround(0.3)
pub const sneakSpeed: f32 = 0.3;
pub const usingItemSpeed: f32 = 0.2;
