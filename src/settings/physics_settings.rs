
pub struct BubbleColumnInfo {
    pub(crate) down: f32,
    pub(crate) max_down: f32,
    pub(crate) up: f32,
    pub(crate) max_up: f32,
}

pub const YAW_SPEED: f32 = 3.0;
pub const PITCH_SPEED: f32 = 3.0;

pub const PLAYER_SPEED: f32 = 0.1;
pub const NEGLIGEABLE_VELOCITY: f32 = 0.003; // actually 0.005 for 1.8; but seems fine

pub const SOUL_SAND_SPEED: f32 = 0.4;
pub const HONEY_BLOCK_SPEED: f32 = 0.4;
pub const HONEY_BLOCK_JUMP_SPEED: f32 = 0.4;
pub const LADDER_MAX_SPEED: f32 = 0.15;
pub const LADDER_CLIMB_SPEED: f32 = 0.2;

/// moved to physics context now.
pub const WATER_INERTIA: f32 = 0.8;

/// moved to physics context now.
pub const LAVA_INERTIA: f32 = 0.5;

/// deprecated, moved to physics context now.
pub const LIQUID_ACCELERATION: f32 = 0.02;

pub const DEFAULT_SLIPPERINESS: f32 = 0.6;

pub const OUT_OF_LIQUID_IMPULSE: f32 = 0.3;

pub const AUTO_JUMP_COOLDOWN: u8 = 10; // ticks (0.5s)

pub const BUBBLE_COLUMN_SURFACE_DRAG: BubbleColumnInfo = BubbleColumnInfo {
    down: 0.03,
    max_down: -0.9,
    up: 0.1,
    max_up: 1.8,
};
pub const BUBBLE_COLUMN_DRAG: BubbleColumnInfo = BubbleColumnInfo {
    down: 0.03,
    max_down: -0.3,
    up: 0.06,
    max_up: 0.7,
};
pub const SLOW_FALLING: f32 = 0.125;
pub const SPRINTING_UUID: &str =  "662a6b8d-da3e-4c1c-8813-96ea6097278d"; // SPEED_MODIFIER_SPRINTING_UUID is from LivingEntity.java
pub const JUMP_HEIGHT: f32 = 0.41999998688697815; // 0.41999998688697815 | used to be Math.fround(0.42)

pub const AIRBORNE_INERTIA: f32 = 0.91;
pub const AIRBORNE_ACCELERATION: f32 = 0.02;

pub const SPRINT_SPEED: f32 = 0.30000001192092896; // 0.30000001192092896 | used to be Math.fround(0.3)
pub const SNEAK_SPEED: f32 = 0.3;

/// deprecated, different items slow you down differently.
pub const USING_ITEM_SPEED: f32 = 0.2;
