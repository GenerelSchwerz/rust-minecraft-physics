#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

impl AABB {

    #[inline(always)]
    pub fn new(min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> Self {
        Self {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z
        }
    }

    pub fn floor(&mut self) {
        self.min_x = self.min_x.floor();
        self.min_y = self.min_y.floor();
        self.min_z = self.min_z.floor();
        self.max_x = self.max_x.floor();
        self.max_y = self.max_y.floor();
        self.max_z = self.max_z.floor();
    }

    pub fn floored(&self) -> Self {
        Self {
            min_x: self.min_x.floor(),
            min_y: self.min_y.floor(),
            min_z: self.min_z.floor(),
            max_x: self.max_x.floor(),
            max_y: self.max_y.floor(),
            max_z: self.max_z.floor(),
        }
    }

    pub fn extend(mut self, dx: f32, dy: f32, dz: f32) -> Self {
        if dx < 0.0 {
            self.min_x += dx
        } else {
            self.max_x += dx;
        }
        if dy < 0.0 {
            self.min_y += dy
        } else {
            self.max_y += dy
        }
        if dz < 0.0 {
            self.min_z += dz
        } else {
            self.max_z += dz
        }

        return self;
    }

    pub fn contract(mut self, x: f32, y: f32, z: f32) -> Self {
        self.min_x += x;
        self.min_y += y;
        self.min_z += z;
        self.max_x -= x;
        self.max_y -= y;
        self.max_z -= z;
        return self;
    }

    pub fn expand(mut self, x: f32, y: f32, z: f32) -> Self {
        self.min_x -= x;
        self.min_y -= y;
        self.min_z -= z;
        self.max_x += x;
        self.max_y += y;
        self.max_z += z;
        return self;
    }

    pub fn offset(mut self, x: f32, y: f32, z: f32) -> Self {
  
        self.min_x += x;
        self.min_y += y;
        self.min_z += z;
        self.max_x += x;
        self.max_y += y;
        self.max_z += z;

   
        return self;
    }

    pub fn compute_offset_x(&self, other: &AABB, mut offset_x: f32) -> f32 {
        if other.max_y > self.min_y
            && other.min_y < self.max_y
            && other.max_z > self.min_z
            && other.min_z < self.max_z
        {
            if offset_x > 0.0 && other.max_x <= self.min_x {
                offset_x = (self.min_x - other.max_x).min(offset_x)
            } else if offset_x < 0.0 && other.min_x >= self.max_x {
                offset_x = (self.max_x - other.min_x).max(offset_x)
            }
        }
        return offset_x;
    }

    pub fn compute_offset_y(&self, other: &AABB, mut offset_y: f32) -> f32 {
        if other.max_x > self.min_x
            && other.min_x < self.max_x
            && other.max_z > self.min_z
            && other.min_z < self.max_z
        {
            if offset_y > 0.0 && other.max_y <= self.min_y { 
                offset_y = (self.min_y - other.max_y).min(offset_y)
            } else if offset_y < 0.0 && other.min_y >= self.max_y {
                // println!("{} {}", self.max_y - other.min_y, offset_y);
                offset_y = (self.max_y - other.min_y).max(offset_y)
            }
        } else {
            
        }
        return offset_y;
    }

    pub fn compute_offset_z(&self, other: &AABB, mut offset_z: f32) -> f32 {
        if other.max_x > self.min_x
            && other.min_x < self.max_x
            && other.max_y > self.min_y
            && other.min_y < self.max_y
        {
            if offset_z > 0.0 && other.max_z <= self.min_z {
                offset_z = (self.min_z - other.max_z).min(offset_z)
            } else if offset_z < 0.0 && other.min_z >= self.max_z {
                offset_z = (self.max_z - other.min_z).max(offset_z)
            }
        }
        return offset_z;
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        return self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
            && self.min_z < other.max_z
            && self.max_z > other.min_z;
    }
}
