#[derive(PartialEq)]
pub enum LcdInterrupt {
    Off,
    EventTrigger,
    On,
}

pub struct Interrupts {
    pub is_lcd_enabled: bool,
    pub window: bool,
    pub sprites: bool,
    pub background: bool,
    pub vblank: bool,
    pub lyc: bool,
    pub oam_access: bool,
    pub priority_vblank: bool,
    pub hblank: bool,
    pub lcd: LcdInterrupt,
}

impl Interrupts {
    pub fn new() -> Interrupts {
        Interrupts {
            is_lcd_enabled: false,
            window: false,
            sprites: false,
            background: false,
            vblank: false,
            lyc: false,
            oam_access: false,
            priority_vblank: false,
            hblank: false,
            lcd: LcdInterrupt::Off,
        }
    }
}