use core::panic;
use std::ffi::FromBytesUntilNulError;

pub struct GPU {

    pub framebuffer: [u8; 160*144*4],
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

        GPU { framebuffer: [0xFF; 160*144*4], vram: [0; 0x2000], oam: [0; 0x00A0], scanline_counter: 0, lcd_control: 0, lcd_status: 0, scy: 0, scx: 0, ly: 0, lyc: 0, dma: 0, bgp: 0, obp0: 0, obp1: 0, wy: 0, wx: 0 }
    }

    pub fn read_lcd_control(&self) -> u8 {
        self.lcd_control
    }

    pub fn write_lcd_control(&mut self, data: u8) {
        self.lcd_control = data;
    }

    pub fn read_scy(&self) -> u8 {
        self.scy
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

    pub fn read_lcd_status(&self) -> u8 {
        self.lcd_status
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

    // fn check_ly(&mut self) -> bool {
    //     if self.ly == self.lyc {
    //         self.lcd_status |= 0b100;
    //         if self.lcd_status >> 6 & 1 == 1 {
    //             return true;
    //         }
    //     }
    //     false
    // }

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
                    // request_lcd = self.check_ly();
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
                        if (self.lcd_status >> 5) & 1 == 1 {
                            request_lcd = true;
                        }
                    }
                    // request_lcd = self.check_ly();
                }
            },
            0b10 => {
                // In OAM Scan
                if self.scanline_counter >= 80 {
                    self.scanline_counter %= 80;
                    self.lcd_status = (self.lcd_status & !0b11) | 0b11;
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
                    self.scan_line()
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

        let mut window = false;
        let mut base_tile_address: u16 = 0x8000;
        let mut tile_map_address: u16 = 0x9800;
        
        // Draw Background
        if self.lcd_control & 1 == 1 {

            // Window is enabled
            if (self.lcd_control >> 5) & 1 == 1 {
                if self.wy <= self.ly {
                    window = true;
                }
            }

            if (self.lcd_control >> 4) & 1 == 0 {
                base_tile_address = 0x8800;
            }

            if window {
                if (self.lcd_control >> 6) & 1 == 1 {
                    tile_map_address = 0x9C00;
                }
            } else {
                if (self.lcd_control >> 3) & 1 == 1 {
                    tile_map_address = 0x9C00;
                }
            }

            let y_tilemap = if window {
                self.ly.overflowing_sub(self.wy).0
            } else {
                self.scy.overflowing_add(self.ly).0
            };

            let y_tile: u16 = (y_tilemap as u16 / 8).overflowing_mul(32).0;

            for p in 0u8..160 {
                let mut x_tilemap = p.overflowing_add(self.scx).0;

                if window {
                    if p >= self.wx - 7 {
                        x_tilemap = p - self.wx - 7;
                    }
                }

                let x_tile = x_tilemap / 8;

                let tile_id = self.vram[(tile_map_address + y_tile as u16 + x_tile as u16) as usize - 0x8000];

                let tile_address = if base_tile_address == 0x8000 {
                    base_tile_address + (tile_id as u16 * 16)
                } else {
                    let tile_id = tile_id as i8 as i16;
                    base_tile_address + ((tile_id + 128) as u16 * 16)
                };

                let line = (y_tilemap % 8) * 2;
                let data_1 = self.vram[(tile_address + (line as u16)) as usize - 0x8000];
                let data_2 = self.vram[(tile_address + (line as u16) + 1) as usize - 0x8000];

                let mask = (((x_tilemap % 8) as i8 - 7) * -1) as u8;

                let color_id = ((data_2 >> mask) & 1) << 1 | ((data_1 >> mask) & 1);

                let color = (self.bgp >> (color_id * 2)) & 0b11;

                let (r,g,b) = match color {
                    0 => (255,255,255),
                    1 => (0xCC,0xCC,0xCC),
                    2 => (0x77,0x77,0x77),
                    3 => (0,0,0),
                    _ => panic!("Unexpected color: {}", color)
                };

                self.framebuffer[(self.ly as usize * 160 * 4) + (p as usize * 4)] = r;
                self.framebuffer[(self.ly as usize * 160 * 4) + (p as usize * 4) + 1] = g;
                self.framebuffer[(self.ly as usize * 160 * 4) + (p as usize * 4) + 2] = b;
                self.framebuffer[(self.ly as usize * 160 * 4) + (p as usize * 4) + 3] = 0xFF;
            }
        }

        // Draw sprites
        // if (self.lcd_control >> 1) & 1 == 1 {
        //     let tile_height = if (self.lcd_control >> 2) == 1 {
        //         16
        //     } else {
        //         8
        //     };


        //     for obj in 0..40 {
        //         let index = obj * 4;

        //         let y_pos = self.oam[index].overflowing_sub(16).0;
        //         let x_pos = self.oam[index + 1].overflowing_sub(8).0;
        //         let tile_index = self.oam[index + 2];
        //         let attributes = self.oam[index + 3];
                
        //         if (self.ly >= y_pos) && (self.ly < y_pos + tile_height) {
        //             let mut line = self.ly as i32 - y_pos as i32;
        //             if attributes >> 6 & 1 == 1 {
        //                 line = (line - tile_height as i32) * -2;
        //             }

        //             let data_1 = self.vram[(tile_index as usize) * 16 + line as usize];
        //             let data_2 = self.vram[(tile_index as usize) * 16 + line as usize + 1];

        //             for pixel in (0..8).rev() {
        //                 let mask = if attributes >> 5 & 1 == 1 {
        //                     (((pixel % 8) as i8 - 7) * -1) as u8
        //                 } else {
        //                     pixel
        //                 };

        //                 let color_id = ((data_2 >> mask) & 1) << 1 | ((data_1 >> mask) & 1);
        //                 let color = if attributes >> 4 & 1 == 1 {
        //                     (self.obp1 >> (color_id * 2)) & 0b11
        //                 } else {
        //                     (self.obp0 >> (color_id * 2)) & 0b11
        //                 };

        //                 let (r,g,b) = match color {
        //                     0 => continue,
        //                     1 => (0xCC,0xCC,0xCC),
        //                     2 => (0x77,0x77,0x77),
        //                     3 => (0,0,0),
        //                     _ => panic!("Unexpected color: {}", color)
        //                 };

        //                 let p = -(pixel as i32) + 7 + x_pos as i32;

        //                 self.framebuffer[(self.ly as usize * 160 * 4) + (p as usize * 4)] = r;
        //                 self.framebuffer[(self.ly as usize * 160 * 4) + (p as usize * 4) + 1] = g;
        //                 self.framebuffer[(self.ly as usize * 160 * 4) + (p as usize * 4) + 2] = b;
        //                 self.framebuffer[(self.ly as usize * 160 * 4) + (p as usize * 4) + 3] = 0xFF;
        //             }
        //         }
        //     }
        // }
    }
}
