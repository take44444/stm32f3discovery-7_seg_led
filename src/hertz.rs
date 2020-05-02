#[derive(Clone, Copy)]
pub struct Hertz(pub u32);

#[derive(Clone, Copy)]
pub struct MegaHertz(pub u32);

pub trait U32Ext {
    fn hz(self) -> Hertz;
    fn mhz(self) -> MegaHertz;
}

impl U32Ext for u32 {
    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}