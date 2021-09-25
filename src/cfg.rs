pub struct Config {
    width: usize,
    height: usize
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 32768,
            height: 32768
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }
}