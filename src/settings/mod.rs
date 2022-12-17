pub mod physics_settings;

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

#[derive(Clone, Default)]
pub struct PlayerAttributeModifier {
    pub(crate) uuid: String,
    pub(crate) operation: u16,
    pub(crate) amount: f32, // dunno, I think limit is ~1000
}

#[derive(Clone, Default)]
pub struct PlayerAttribute {
    value: f32,
    modifiers: Vec<PlayerAttributeModifier>,
}


// note: move this to a mod instead of impl here.
impl PlayerAttribute {
    pub fn createAttributeValue(base: f32) -> Self {
        Self {
            value: base,
            modifiers: vec![],
        }
    }

    /// assumed decimal point wanted.
    pub fn getAttributeValue(prop: PlayerAttribute) -> f32 {
        let mut x = prop.value;
        for m in &prop.modifiers {
            if m.operation != 0 {
                continue;
            }
            x += m.amount;
        }
        let mut y = x;
        for m in &prop.modifiers {
            if m.operation != 1 {
                continue;
            }
            y += x * m.amount;
        }
        for m in &prop.modifiers {
            if m.operation != 2 {
                continue;
            }
            y += y * m.amount;
        }
        return y;
    }

    pub fn addAttributeModifier(mut attributes: PlayerAttribute, modifier: PlayerAttributeModifier) -> PlayerAttribute {
        attributes.modifiers.push(modifier);
        return attributes;
    }

    pub fn checkAttributeModifier(attributes: &PlayerAttribute, uuid: &str) -> bool {
        return attributes
            .modifiers
            .iter()
            .find(|m| m.uuid == uuid)
            .is_some();
    }

    pub fn deleteAttributeModifier(mut attributes: PlayerAttribute, uuid: &str) -> PlayerAttribute {
        attributes.modifiers = attributes
            .modifiers
            .into_iter()
            .filter(|m| m.uuid != uuid)
            .collect();

        return attributes;
    }

    // pretty sure my delete attributes is unfaithful.
    // original code below:

    // export function deleteAttributeModifier(attributes: Attribute, uuid: string) {
    //     for (const modifier of attributes.modifiers) {
    //         if (modifier.uuid === uuid) attributes.modifiers.splice(attributes.modifiers.indexOf(modifier));
    //     }
    //     return attributes;
    // }
}
