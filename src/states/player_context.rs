
#[derive(Clone, Copy)]
pub struct WidthAndHeight {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

pub const PLAYER_POSE_CONTEXT: [WidthAndHeight; 8] = [
    WidthAndHeight {
        width: 0.6,
        height: 1.8,
    },
    WidthAndHeight {
        width: 0.2,
        height: 0.2,
    },
    WidthAndHeight {
        width: 0.6,
        height: 0.6,
    },
    WidthAndHeight {
        width: 0.6,
        height: 1.8,
    },
    WidthAndHeight {
        width: 0.6,
        height: 1.8,
    },
    WidthAndHeight {
        width: 0.6,
        height: 1.8,
    },
    WidthAndHeight {
        width: 0.6,
        height: 1.5,
    },
    WidthAndHeight {
        width: 0.2,
        height: 0.2,
    },
];

/// I'm not sure whether or not self is a performance issue.
/// I'll look into it later.
#[repr(usize)]
#[derive(Clone, Copy)]
pub enum PlayerPoses {
    Standing,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack, // dunno
    Sneaking,
    LongJumping,
    Dying,
}

impl Default for PlayerPoses {
    fn default() -> Self {
        Self::Standing
    }
}

impl PlayerPoses {
    #[inline]
    pub fn get_info(self) -> WidthAndHeight {
        return PLAYER_POSE_CONTEXT[self as usize];
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ControlStateHandler {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sprint: bool,
    pub sneak: bool,
}

impl ControlStateHandler {}