use crate::states::physics_context::EntityPhysicsContext;

use super::World;

pub trait Symulator {

    fn simulate(ctx: EntityPhysicsContext, world: &impl World) -> EntityPhysicsContext;


}



pub struct GenSimulator {
    
}