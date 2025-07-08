#[repr(C)]
pub union Pixel {
    float: f32,
    number: u32,
    color: (u8, u8, u8, u8),  // ARGB
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel { number: 0u32 }
    }

    pub fn f32(&self) -> f32 {
        let result = unsafe{ self.float };
        return result;
    }

    pub fn f32_set(&mut self, value: f32) {
        self.float = value;
    }

    pub fn u32(&self) -> u32 {
        let result = unsafe{ self.number };
        return result;
    }

    pub fn u32_set(&mut self, value: u32) {
        self.number = value;
    }

    pub fn a(&self) -> u8 {
        let result = unsafe{ self.color.0 };
        return result;
    }

    pub fn a_set(&mut self, value: u8) {
        self.color.0 = value;
    }

    pub fn r(&self) -> u8 {
        let result = unsafe{ self.color.1 };
        return result;
    }

    pub fn r_set(&mut self, value: u8) {
        self.color.1 = value;
    }

    pub fn g(&self) -> u8 {
        let result = unsafe{ self.color.2 };
        return result;
    }

    pub fn g_set(&mut self, value: u8) {
        self.color.2 = value;
    }

    pub fn b(&self) -> u8 {
        let result = unsafe{ self.color.3 };
        return result;
    }

    pub fn b_set(&mut self, value: u8) {
        self.color.3 = value;
    }
}

// TODO: tests
