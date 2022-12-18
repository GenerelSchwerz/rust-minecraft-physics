use crate::states::physics_context::EntityPhysicsContext;

pub mod prismarine_simulator;
pub mod gen_simulator;

/// Temporary
#[derive(Default)]
pub struct Block {
    // original = type: u32
    pub(crate) bounding_box: String,
    pub(crate) metadata: u32,
    pub(crate) b_type: u32,
    pub(crate) position: glam::Vec3A,
    pub(crate) shapes: Vec<[f32; 6]>,
}

impl Block {
    pub fn test_new(
        bounding_box: String,
        metadata: u32,
        b_type: u32,
        position: glam::Vec3A,
        shapes: Vec<[f32; 6]>,
    ) -> Self {
        Self {
            bounding_box,
            metadata,
            b_type,
            position,
            shapes,
        }
    }
}

#[derive(Default)]
pub struct BlockProps {
    waterlogged: bool,
}

impl Block {
    pub fn get_properties(&self) -> BlockProps {
        BlockProps::default()
    }
}

pub trait World {
    fn get_block(&self, pos: &glam::Vec3A) -> Option<Block>;
}


pub trait Simulator {

    fn simulate(&self, ctx: EntityPhysicsContext, world: &impl World) -> EntityPhysicsContext;
}