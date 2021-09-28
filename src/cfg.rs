pub struct Config {
    //
    // Readonly properties. Available through getters
    //
    WIDTH: usize,
    HEIGHT: usize,
    //
    // Read-Write properties. Available through direct access
    // from every module
    //
    pub frame_delay: u32
}

impl Config {
    pub fn new() -> Config {
        Config {
            WIDTH: 1024,
            HEIGHT: 1024,
            frame_delay : 0
        }
    }
    //
    // All readonly properties should have their getters
    //
    pub fn width(&self) -> usize { self.WIDTH }
    pub fn height(&self) -> usize { self.HEIGHT }
}