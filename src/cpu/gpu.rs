pub struct GPU {

    vram: [u8; 0x2000],
    oam: [u8; 0x00A0],
    scanline_counter: u16,

    // FF40 - LCDC: LCD control
    lcd_control: u8,
    // FF41 - STAT: LCD status
    lcd_status: u8,
    // FF42,FF43 - SCY, SCX: Background viewport y,x position
    scy: u8,
    scx: u8,
    // FF44 - LY: LCD Y coordinate
    ly: u8,
    // FF45 - LYC: LY compare
    lyc: u8,
    // FF46 - DMA: OAM DMA source address & start
    dma: u8,
    // FF47 - BGP: BG palette data
    bgp: u8,
    // FF48, FF49 - OBP0, OBP1: OBJ palette 0, 1 data
    obp0: u8,
    obp1: u8,
    // FF4A, FF4B - WY, WX: Window y,x position plus 7
    wy: u8,
    wx: u8
}

impl GPU {
    pub fn new() -> GPU {

        GPU { vram: [0; 0x2000], oam: [0; 0x00A0], scanline_counter: 0, lcd_control: 0, lcd_status: 0, scy: 0, scx: 0, ly: 0, lyc: 0, dma: 0, bgp: 0, obp0: 0, obp1: 0, wy: 0, wx: 0 }
    }

    pub fn read_lcd_control(&self) -> u8 {
        self.lcd_control
    }

    pub fn write_lcd_control(&mut self, data: u8) {
        self.lcd_control = data;
    }

    pub fn write_scy(&mut self, data: u8) {
        self.scy = data;
    }

    pub fn write_scx(&mut self, data: u8) {
        self.scx = data;
    }

    pub fn write_obp0(&mut self, data: u8) {
        self.obp0 = data;
    }

    pub fn write_obp1(&mut self, data: u8) {
        self.obp1 = data;
    }

    pub fn write_bgp(&mut self, data: u8) {
        self.bgp = data;
    }

    pub fn read_ly(&self) -> u8 {
        self.ly
    }

    pub fn write_lcd_status(&mut self, data: u8) {
        self.lcd_status = (data & !0b111) | (self.lcd_status & 0b111);
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        self.vram[address as usize]
    }

    pub fn write_vram(&mut self, address: u16, data: u8) {
        self.vram[address as usize] = data;
    }

    pub fn write_oam(&mut self, address: u16, data: u8) {
        self.oam[address as usize] = data;
    }

    pub fn write_wy(&mut self, data: u8) {
        self.wy = data;
    }

    pub fn write_wx(&mut self, data: u8) {
        self.wx = data;
    }

    pub fn oam_dma(&mut self, data: &[u8]) {
        for i in 0..0xA0 {
            self.oam[i] = data[i];
        }
    }

    pub fn update(&mut self, cycles: u8) -> (bool, bool) {
        let mut request_vblank = false;
        let mut request_lcd = false;

        if self.lcd_control >> 7 == 0 {

            self.scanline_counter = 0;
            self.ly = 0;
            self.lcd_status = (self.lcd_status & !0b11) | 0b01;
            return (false, false)
        }


        self.scanline_counter += cycles as u16;
        // if self.scanline_counter >= 456 {
        //     self.scanline_counter %= 456;
        //     self.ly += 1;
        //     if self.ly == 144 {
        //         request_vblank = true;
        //     } else if self.ly > 153 {
        //         self.ly = 0;
        //     } else if self.ly < 144 {
        //         // draw
        //     }
        // }

        match self.lcd_status & 0b11 {
            0b00 => {
                // In HBLANK
                if self.scanline_counter >= 204 {
                    self.scanline_counter %= 204;
                    self.ly += 1;

                    if self.ly >= 144 {
                        self.lcd_status = (self.lcd_status & !0b11) | 0b01;
                        request_vblank = true;
                        if (self.lcd_status >> 4) & 1 == 1 {
                            request_lcd = true;
                        }
                    } else {
                        self.lcd_status = (self.lcd_status & !0b11) | 0b10;
                        if (self.lcd_status >> 5) & 1 == 1 {
                            request_lcd = true;
                        }
                    }
                }
            },
            0b01 => {
                // In VBLANK
                if self.scanline_counter >= 456 {
                    self.scanline_counter %= 456;
                    self.ly += 1;
                    if self.ly == 154 {
                        self.lcd_status = (self.lcd_status & !0b11) | 0b10;
                        self.ly = 0;
                    }
                }
            },
            0b10 => {
                // In OAM Scan
                if self.scanline_counter >= 80 {
                    self.scanline_counter %= 80;
                    self.lcd_status = (self.lcd_status & !0b11) | 0b11;
                    if (self.lcd_status >> 5) & 1 == 1 {
                        request_lcd = true;
                    }
                }
            },
            0b11 => {
                // Drawing pixels
                if self.scanline_counter >= 172 {
                    self.scanline_counter %= 172;
                    self.lcd_status &= !0b11;
                    if (self.lcd_status >> 3) & 1 == 1 {
                        request_lcd = true;
                    }
                }
            }
            _ => panic!("Unexpected LCD status: {}", self.lcd_status)
        }

        if self.ly == self.lyc {
            self.lcd_status |= 0b100;
            if (self.lcd_status >> 6) & 1 == 1 {
                request_lcd = true;
            }
        } else {
            self.lcd_status &= !0b100;
        }
        
        (request_vblank, request_lcd)
    } 


    fn scan_line(&mut self) {

        // Draw Background
        if self.lcd_control & 1 == 1 {
            
        }

        // Draw sprites
        if (self.lcd_control >> 1) & 1 == 1 {
            
        }
    }
}
