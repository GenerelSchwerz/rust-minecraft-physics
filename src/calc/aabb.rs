#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub minX: f32,
    pub minY: f32,
    pub minZ: f32,
    pub maxX: f32,
    pub maxY: f32,
    pub maxZ: f32,
}

impl AABB {

    #[inline(always)]
    pub fn new(minX: f32, minY: f32, minZ: f32, maxX: f32, maxY: f32, maxZ: f32) -> Self {
        Self {
            minX,
            minY,
            minZ,
            maxX,
            maxY,
            maxZ
        }
    }

    pub fn floor(&mut self) {
        self.minX = self.minX.floor();
        self.minY = self.minY.floor();
        self.minZ = self.minZ.floor();
        self.maxX = self.maxX.floor();
        self.maxY = self.maxY.floor();
        self.maxZ = self.maxZ.floor();
    }

    pub fn floored(&self) -> Self {
        Self {
            minX: self.minX.floor(),
            minY: self.minY.floor(),
            minZ: self.minZ.floor(),
            maxX: self.maxX.floor(),
            maxY: self.maxY.floor(),
            maxZ: self.maxZ.floor(),
        }
    }

    pub fn extend(mut self, dx: f32, dy: f32, dz: f32) -> Self {
        if dx < 0.0 {
            self.minX += dx
        } else {
            self.maxX += dx;
        }
        if dy < 0.0 {
            self.minY += dy
        } else {
            self.maxY += dy
        }
        if dz < 0.0 {
            self.minZ += dz
        } else {
            self.maxZ += dz
        }

        return self;
    }

    pub fn contract(mut self, x: f32, y: f32, z: f32) -> Self {
        self.minX += x;
        self.minY += y;
        self.minZ += z;
        self.maxX -= x;
        self.maxY -= y;
        self.maxZ -= z;
        return self;
    }

    pub fn expand(mut self, x: f32, y: f32, z: f32) -> Self {
        self.minX -= x;
        self.minY -= y;
        self.minZ -= z;
        self.maxX += x;
        self.maxY += y;
        self.maxZ += z;
        return self;
    }

    pub fn offset(mut self, x: f32, y: f32, z: f32) -> Self {
        self.minX += x;
        self.minY += y;
        self.minZ += z;
        self.maxX += x;
        self.maxY += y;
        self.maxZ += z;
        return self;
    }

    pub fn compute_offset_x(&self, other: &AABB, mut offset_x: f32) -> f32 {
        if other.maxY > self.minY
            && other.minY < self.maxY
            && other.maxZ > self.minZ
            && other.minZ < self.maxZ
        {
            if offset_x > 0.0 && other.maxX <= self.minX {
                offset_x = (self.minX - other.maxX).min(offset_x)
            } else if offset_x < 0.0 && other.minX >= self.maxX {
                offset_x = (self.maxX - other.minX).min(offset_x)
            }
        }
        return offset_x;
    }

    pub fn compute_offset_y(&self, other: &AABB, mut offset_y: f32) -> f32 {
        if other.maxX > self.minX
            && other.minX < self.maxX
            && other.maxZ > self.minZ
            && other.minZ < self.maxZ
        {
            if offset_y > 0.0 && other.maxY <= self.minY {
                offset_y = (self.minY - other.maxY).min(offset_y)
            } else if offset_y < 0.0 && other.minY >= self.maxY {
                offset_y = (self.maxY - other.minY).min(offset_y)
            }
        }
        return offset_y;
    }

    pub fn compute_offset_z(&self, other: &AABB, mut offset_z: f32) -> f32 {
        if other.maxX > self.minX
            && other.minX < self.maxX
            && other.maxY > self.minY
            && other.minY < self.maxY
        {
            if offset_z > 0.0 && other.maxZ <= self.minZ {
                offset_z = (self.minZ - other.maxZ).min(offset_z)
            } else if offset_z < 0.0 && other.minZ >= self.maxZ {
                offset_z = (self.maxZ - other.minZ).min(offset_z)
            }
        }
        return offset_z;
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        return self.minX < other.maxX
            && self.maxX > other.minX
            && self.minY < other.maxY
            && self.maxY > other.minY
            && self.minZ < other.maxZ
            && self.maxZ > other.minZ;
    }
}
