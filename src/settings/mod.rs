pub mod physics_settings;


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
    pub fn create_attribute_value(base: f32) -> Self {
        Self {
            value: base,
            modifiers: vec![],
        }
    }

    /// assumed decimal point wanted.
    pub fn get_attribute_value(prop: PlayerAttribute) -> f32 {
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

    pub fn add_attribute_modifier(mut attributes: PlayerAttribute, modifier: PlayerAttributeModifier) -> PlayerAttribute {
        attributes.modifiers.push(modifier);
        return attributes;
    }

    pub fn check_attribute_modifier(attributes: &PlayerAttribute, uuid: &str) -> bool {
        return attributes
            .modifiers
            .iter()
            .find(|m| m.uuid == uuid)
            .is_some();
    }

    pub fn delete_attribute_modifier(mut attributes: PlayerAttribute, uuid: &str) -> PlayerAttribute {
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
