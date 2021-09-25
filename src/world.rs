/**
 * Structure of the world of dots/atoms/pixels
 */
pub struct World {
    dots: Vec<u16> // linear array of dots
}

impl World {
    pub fn new(len: usize) -> World {
        let mut v = Vec::with_capacity(len);
        unsafe { v.set_len(len) }
        
        World {
            dots: v
        }
    }

    pub fn get_dot(&self, offs: usize) -> u16 {
        self.dots[offs]
    }

    pub fn set_dot(&mut self, offs: usize, dot: u16) {
        self.dots[offs] = dot;
    }

    pub fn len(&self) -> usize {
        self.dots.len()
    }
}