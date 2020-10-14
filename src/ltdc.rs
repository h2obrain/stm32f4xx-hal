//! LTDC interface (lcd/tft-display controller 8/16/32-bit parallel display interface)

use crate::pin_defs::*;
use crate::stm32::LTDC; //{LTDC,ltdc};


/// LTDC specific types

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LtdcPolarity {
    pub pcpol: LtdcSignalPolarity,
    pub depol: LtdcSignalPolarity,
    pub vspol: LtdcSignalPolarity,
    pub hspol: LtdcSignalPolarity,
}
impl Default for LtdcPolarity {
    fn default() -> Self {
        Self {
            pcpol: LtdcSignalPolarity::ACTIVE_LOW,
            depol: LtdcSignalPolarity::ACTIVE_LOW,
            vspol: LtdcSignalPolarity::ACTIVE_LOW,
            hspol: LtdcSignalPolarity::ACTIVE_LOW,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LtdcTiming {
    pub h_sync: u16, /* HSA, horizontal sync time */
    pub h_back_porch: u16, /* HBP, 'blind' pixels left */
    pub h_active: u16, /* HACT, display width, visible resolution */
    pub h_front_porch: u16, /* HFP, 'blind' pixels right */
    
    pub v_sync: u16, /* VSA, vertical sync time */
    pub v_back_porch: u16, /* VBP, 'blind' pixels up */
    pub v_active: u16, /* VACT, display height, visible resolution */
    pub v_front_porch: u16, /* VFP, 'blind' pixels down */
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LtdcSignalPolarity {
    ACTIVE_LOW,
    ACTIVE_HIGH,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum LtdcColorCoding {
    Argb8888 = 0,
    Rgb888 = 1,
    Rgb565 = 2,
    Argb1555 = 3,
    Argb4444 = 4,
    L8 = 5,
    Al44 = 6,
    Al88 = 7,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum LtdcLayer {
    Layer1 = 1,
    Layer2 = 2,
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum LtdcReload {
    Immediate = 0,
    VerticalBlanking = 1,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LtdcBlendingFactor {
    ConstAlpha,
    PixelAlphaTimesConstAlpha,
}


/// helper-functions
fn get_rgb888_from_rgb565(rgb565:u16) -> u32 {
    let rgb565_32 = rgb565 as u32;
      ((((rgb565_32) & 0xF800) >> (11-8))/31)<<16
    | ((((rgb565_32) & 0x07E0) <<  (8-5))/63)<<8
    | ((((rgb565_32) & 0x001F) <<  (8-0))/31)<<0
}
fn align_up_to(alignment:u16, alignee:u16) -> u16 {
    let off = alignee % alignment;
    if off == 0 {
        alignee
    } else {
        alignee + alignment - off
    }
}


// implementation..

pub trait LtdcLowLevelExt {
    /* Needed for the dsi driver some day.. */
    fn config_access_possible(&self) -> bool;
    fn config_access_begin(&self);
    fn config_access_end(&self);
    
    /* update settings (this is sometimes needed..) */
    fn reload(&self, mode:LtdcReload);
    /* Returns true if while NOT reloading */
    fn ready(&self) -> bool;

    /* Configure R,G,B component values for LCD background color */
    fn set_background_color(&self, rgb888:u32);
    /* Set signal polarities (inverted from DSI! DEP is inverted twice!!) */
    fn set_polarity(&self, polarity: LtdcPolarity);
    /* Timing configuration */
    fn set_timing(&self, timing: LtdcTiming);
//    /* Execute on layer */
//    fn layer<F: FnOnce(&mut ltdc::LAYER)>(&self, layer: LtdcLayer, f: F);
    /* Set pixel format */
    fn set_pixel_format(&self, layer: LtdcLayer, color_coding: LtdcColorCoding);
    /* Default Color configuration (configure A,R,G,B component values) */
    fn set_default_colors(&self, layer: LtdcLayer, argb8888:u32);
    /* Set layer constant alpha multiplier */
    fn set_constant_alpha(&self, layer: LtdcLayer, alpha:u8);
    /* Configure blending factors */
    fn set_blending_factors(
                &self, layer: LtdcLayer,
                bf1: LtdcBlendingFactor,
                bf2: LtdcBlendingFactor,
            );
    /* Color keying */
    fn set_color_key(&self, layer: LtdcLayer, rgb888: u32);
    /* Windowing */
    fn windowing_config(&self, layer: LtdcLayer, h_back_porch:u16,v_back_porch:u16,h_active:u16,v_active:u16);
    fn windowing_config_xywh(&self, layer: LtdcLayer, x:u16,y:u16,w:u16,h:u16);
    /* Line length and pitch */
    fn set_fb_line_length(&self, layer: LtdcLayer, length:u16, pitch:u16);
    /* Configure the number of lines */
    fn set_fb_line_count(&self, layer: LtdcLayer, v_active:u16);
    /* Pixel buffers */
    fn set_fbuffer_address(&self, layer: LtdcLayer, fbuffer:u32);
    /* Enable foreground & background Layers */
    fn enable_layer(&self, layer: LtdcLayer, enable:bool);
    /* Enable color lookup table */
    fn enable_color_lookup_table(&self, layer: LtdcLayer, enable:bool);
    /* Enable color keying */
    fn enable_color_keying(&self, layer: LtdcLayer, enable:bool);
    /* enable dithering to add artsy graphical artifacts */
    fn enable_dithering(&self, enable:bool);
}
impl LtdcLowLevelExt for LTDC {
    /* Needed for the dsi driver some day.. */
    fn config_access_possible(&self) -> bool { true }
    fn config_access_begin(&self) {}
    fn config_access_end(&self) {}
    
    
    /* update settings (this is sometimes needed..) */
    fn reload(&self, mode:LtdcReload) {
        match mode {
            LtdcReload::Immediate => self.srcr.write(|w| w.imr().set_bit()),
            LtdcReload::VerticalBlanking => self.srcr.write(|w| w.vbr().set_bit()),
        }
    }
    /* Returns true if while NOT reloading */
    fn ready(&self) -> bool {
        let r = self.srcr.read();
        !(r.vbr().bit() || r.imr().bit())
    }

    /* Configure R,G,B component values for LCD background color */
    fn set_background_color(&self, rgb888:u32) {
        self.bccr.write(|w| unsafe { w.bits(rgb888) });
    }
    /* Set signal polarities (inverted from DSI! DEP is inverted twice!!) */
    fn set_polarity(&self, polarity: LtdcPolarity) {
        self.gcr.write(|w|
            w
                .pcpol().bit(polarity.pcpol == LtdcSignalPolarity::ACTIVE_HIGH)
                .depol().bit(polarity.depol == LtdcSignalPolarity::ACTIVE_HIGH)
                .vspol().bit(polarity.vspol == LtdcSignalPolarity::ACTIVE_HIGH)
                .hspol().bit(polarity.hspol == LtdcSignalPolarity::ACTIVE_HIGH)
        );
    }
    /* Timing configuration */
    fn set_timing(&self, timing: LtdcTiming) {
            let mut hor;
            let mut ver;
            /*assert!((timing.h_sync > 0) && (timing.h_sync > 0) && (timing.h_sync <= 0x400) && (timing.h_sync <= 0x300));*/
            hor = timing.h_sync - 1;
            ver = timing.v_sync  - 1;
            /*assert!((hor&0xfff == hor) && (h&0x7ff == h));*/
            self.sscr.write(|w| unsafe { w.hsw().bits(hor).vsh().bits(ver) });
        
            hor += timing.h_back_porch;
            ver += timing.v_back_porch;
            /*assert((hor&0xfff == hor) && (ver&0x7ff == ver));*/
            self.bpcr.write(|w| unsafe { w.ahbp().bits(hor).avbp().bits(ver) });
        
            hor += timing.h_active;
            ver += timing.v_active;
            /*assert((hor&0xfff == hor) && (ver&0x7ff == ver));*/
            self.awcr.write(|w| unsafe { w.aaw().bits(hor).aah().bits(ver) });
        
            hor += timing.h_front_porch;
            ver += timing.v_front_porch;
            /*assert((hor&0xfff == hor) && (ver&0x7ff == ver));*/
            self.twcr.write(|w| unsafe { w.totalw().bits(hor).totalh().bits(ver) });
    }
    /* Set pixel format */
    fn set_pixel_format(&self, layer: LtdcLayer, color_coding: LtdcColorCoding) {
        match layer {
            LtdcLayer::Layer1 => {
                self.layer1.pfcr.write(|w| unsafe { w.bits(color_coding as u32) });
            },
            LtdcLayer::Layer2 => {
                self.layer2.pfcr.write(|w| unsafe { w.bits(color_coding as u32) });
            },
        };
    }
    /* Default Color configuration (configure A,R,G,B component values) */
    fn set_default_colors(&self, layer: LtdcLayer, argb8888:u32) {
        match layer {
            LtdcLayer::Layer1 => {
                self.layer1.dccr.write(|w| unsafe { w.bits(argb8888) });
            },
            LtdcLayer::Layer2 => {
                self.layer2.dccr.write(|w| unsafe { w.bits(argb8888) });
            },
        };
    }
    /* Set layer constant alpha multiplier */
    fn set_constant_alpha(&self, layer: LtdcLayer, alpha:u8) {
        match layer {
            LtdcLayer::Layer1 => {
                self.layer1.cacr.write(|w| unsafe { w.consta().bits(alpha) });
            },
            LtdcLayer::Layer2 => {
                self.layer2.cacr.write(|w| unsafe { w.consta().bits(alpha) });
            },
        };
    }
    /* Configure blending factors */
    fn set_blending_factors(
        &self,
        layer: LtdcLayer,
        bf1: LtdcBlendingFactor,
        bf2: LtdcBlendingFactor,
    ) {
        let bf1 = match bf1 {
            LtdcBlendingFactor::ConstAlpha => 0b100,
            LtdcBlendingFactor::PixelAlphaTimesConstAlpha => 0b110,
        };
        let bf2 = match bf2 {
            LtdcBlendingFactor::ConstAlpha => 0b101,
            LtdcBlendingFactor::PixelAlphaTimesConstAlpha => 0b111,
        };
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.bfcr.write(|w| unsafe { w.bf1().bits(bf1).bf2().bits(bf2) });
    }
    /* Color keying */
    fn set_color_key(&self, layer: LtdcLayer, rgb888: u32) {
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.ckcr.write(|w| unsafe { w.bits(rgb888) });
    }
    /* Windowing */
    fn windowing_config(&self, layer: LtdcLayer, h_back_porch:u16,v_back_porch:u16,h_active:u16,v_active:u16) {
        let h_active_end = h_active + h_back_porch - 1;
        let v_active_end = v_active + v_back_porch - 1;
        /*assert!((h_back_porch & 0xfff == h_back_porch) &&
             (v_back_porch  & 0xfff == v_back_porch) &&
             (active_width & 0xfff == active_width) &&
             (active_height & 0xfff == active_height));*/
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.whpcr.write(|w| unsafe { w.whstpos().bits(h_back_porch).whsppos().bits(h_active_end) });
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.wvpcr.write(|w| unsafe { w.wvstpos().bits(v_back_porch).wvsppos().bits(v_active_end) });   
    }
    fn windowing_config_xywh(&self, layer: LtdcLayer, x:u16,y:u16,w:u16,h:u16) {
        let local_config = !self.config_access_possible();
        if local_config { self.config_access_begin(); }
        let bpcr = self.bpcr.read();
        let xoff = 1 + bpcr.ahbp().bits(); // hsync+h_back_porch
        let yoff = 1 + bpcr.avbp().bits(); // vsync+v_back_porch
        self.windowing_config(layer, xoff+x, yoff+y, w,h);
        if local_config { self.config_access_end(); }
    }
    /* Line length and pitch */
    fn set_fb_line_length(&self, layer: LtdcLayer, length:u16, pitch:u16) {
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.cfblr.write(|w| unsafe { w.cfbll().bits(length).cfbp().bits(pitch) });
    }
    /* Configure the number of lines */
    fn set_fb_line_count(&self, layer: LtdcLayer, v_active:u16) {
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.cfblnr.write(|w| unsafe { w.cfblnbr().bits(v_active) });
    }
    /* Pixel buffers */
    fn set_fbuffer_address(&self, layer: LtdcLayer, fbuffer:u32) {
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.cfbar.write(|w| unsafe { w.bits(fbuffer) });
    }
    /* Enable layer */
    fn enable_layer(&self, layer: LtdcLayer, enable:bool) {
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.cr.modify(|_,w| w.len().bit(enable));
    }
    /* Enable color lookup table */
    fn enable_color_lookup_table(&self, layer: LtdcLayer, enable:bool) {
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.cr.modify(|_,w| w.cluten().bit(enable));
    }
    /* Enable color keying */
    fn enable_color_keying(&self, layer: LtdcLayer, enable:bool) {
        match layer {
            LtdcLayer::Layer1 => &self.layer1,
            LtdcLayer::Layer2 => &self.layer2,
        }.cr.modify(|_,w| w.colken().bit(enable));
    }
    /* Enable dithering to add artsy graphical artifacts */
    fn enable_dithering(&self, enable:bool) {
        self.gcr.modify(|_,w| w.den().bit(enable));
    }
}

pub trait LtdcExt<PINS> {
    fn setup(
        self,
        pins: PINS,
        config: LtdcConfig,
    ) -> Ltdc<PINS>
    where
        PINS: Pins<LTDC>;
}
impl<PINS> LtdcExt<PINS> for LTDC {
    fn setup(
        self,
        pins: PINS,
        config: LtdcConfig,
    ) -> Ltdc<PINS>
    where
        PINS: Pins<LTDC>,
    {
        Ltdc::new(self, pins, config)
    }
}


#[allow(dead_code)]
pub struct Ltdc<PINS> {
    ltdc: LTDC,
    pins: PINS,
}

impl<PINS> Ltdc<PINS> {
    fn new(
        ltdc: LTDC,
        pins: PINS,
        config: LtdcConfig,
    ) -> Self
    where
        PINS: Pins<LTDC>,
    {
        let ltdc = Ltdc { pins, ltdc };
        ltdc.config(config);
        ltdc
    }
}
pub trait LtdcController {
    fn config(&self, config: LtdcConfig) -> u16;
}
impl<PINS> LtdcController for Ltdc<PINS> {
    fn config(&self, config: LtdcConfig) -> u16 {
        // TODO reset everything
        
//        let ltdc = Ltdc { ltdc, pins };
        
        let local_config = !self.ltdc.config_access_possible();
        if local_config {
            self.ltdc.config_access_begin();
        }

        /* Configure R,G,B component values for LCD background color */
        self.ltdc.set_background_color(0x000000);

        /* Set signal polarities (inverted from DSI! DEP is inverted twice!!) */
        self.ltdc.set_polarity(config.polarity);

        /* Timing configuration */
        self.ltdc.set_timing(config.timing);

        /* Pixel format */
        
        
//    uint32_t ltdc_color_coding, ltdc_color_coding_byte_size;
//    switch (color_coding) {
//        case DISPLAY_COLOR_CODING_RGB565 :
//            ltdc_color_coding = LTDC_LxPFCR_RGB565;
//            ltdc_color_coding_byte_size = 2;
//            break;
//        case DISPLAY_COLOR_CODING_RGB888 :
//            ltdc_color_coding = LTDC_LxPFCR_RGB888;
//            ltdc_color_coding_byte_size = 3;
//            break;
//        case DISPLAY_COLOR_CODING_ARGB8888 :
//        default :
//            ltdc_color_coding = LTDC_LxPFCR_ARGB8888;
//            ltdc_color_coding_byte_size = 4;
//            break;
//    }
        self.ltdc.set_pixel_format(LtdcLayer::Layer1, config.color_coding);
        self.ltdc.set_pixel_format(LtdcLayer::Layer2, config.color_coding);

        /* Default Color configuration (configure A,R,G,B component values) */
        self.ltdc.set_default_colors(LtdcLayer::Layer1, 0x00000000);
        self.ltdc.set_default_colors(LtdcLayer::Layer2, 0x00000000);

        /* Set layer constant alpha multiplier */
        self.ltdc.set_constant_alpha(LtdcLayer::Layer1, 255);
        self.ltdc.set_constant_alpha(LtdcLayer::Layer2, 255);
    
        /* Configure blending factors */
        self.ltdc.set_blending_factors(
                LtdcLayer::Layer1,
                LtdcBlendingFactor::PixelAlphaTimesConstAlpha,
                LtdcBlendingFactor::PixelAlphaTimesConstAlpha,
            );
        self.ltdc.set_blending_factors(
                LtdcLayer::Layer2,
                LtdcBlendingFactor::PixelAlphaTimesConstAlpha,
                LtdcBlendingFactor::PixelAlphaTimesConstAlpha,
            );

        /* Color keying */
        if let Some(color_key) = config.color_key {
            self.ltdc.set_color_key(
                    LtdcLayer::Layer2,
                    match config.color_coding {
                        LtdcColorCoding::Argb8888 | LtdcColorCoding::Rgb888 => color_key,
                        LtdcColorCoding::Rgb565 => get_rgb888_from_rgb565(color_key as u16),
                        LtdcColorCoding::Argb1555 => color_key,
                        LtdcColorCoding::Argb4444 => color_key,
                        LtdcColorCoding::L8 => color_key,
                        LtdcColorCoding::Al44 => color_key,
                        LtdcColorCoding::Al88 => color_key,
                    }
                );
            self.ltdc.enable_color_keying(LtdcLayer::Layer2, true);
        }

        /* Windowing */
        self.ltdc.windowing_config_xywh(LtdcLayer::Layer1, 0,0, config.timing.h_active, config.timing.v_active);
        self.ltdc.windowing_config_xywh(LtdcLayer::Layer2, 0,0, config.timing.h_active, config.timing.v_active);
    
        /* Line length and pitch */
        let color_coding_byte_size = match config.color_coding {
                    LtdcColorCoding::Argb8888 => 4,
                    LtdcColorCoding::Rgb888 => 3,
                    LtdcColorCoding::Rgb565 => 2,
                    LtdcColorCoding::Argb1555 => 2,
                    LtdcColorCoding::Argb4444 => 2,
                    LtdcColorCoding::L8 => 1,
                    LtdcColorCoding::Al44 => 1,
                    LtdcColorCoding::Al88 => 2,
                };
        let line_byte_size = config.timing.h_active * color_coding_byte_size;
        let line_byte_pitch = align_up_to(64,line_byte_size);
    //  assert("Invalid display width"&&(line_byte_size==line_byte_pitch));
        self.ltdc.set_fb_line_length(LtdcLayer::Layer1, line_byte_size+3,line_byte_pitch);
        self.ltdc.set_fb_line_length(LtdcLayer::Layer2, line_byte_size+3,line_byte_pitch);
    
        /* Configure the number of lines */
        self.ltdc.set_fb_line_count(LtdcLayer::Layer1, config.timing.v_active);
        self.ltdc.set_fb_line_count(LtdcLayer::Layer2, config.timing.v_active);
    
        /* Pixel buffers */
        if let Some(buffer) = config.layer1_buffer {
            self.ltdc.set_fbuffer_address(LtdcLayer::Layer1, buffer);
        }
        if let Some(buffer) = config.layer2_buffer {
            self.ltdc.set_fbuffer_address(LtdcLayer::Layer2, buffer);
        }
    
        /* update settings (otherwise fbuffer_address reads the old value!) */
        self.ltdc.reload(LtdcReload::Immediate);
        while !self.ltdc.ready() {}
    
        /* Enable foreground & background Layers */
        self.ltdc.enable_layer(LtdcLayer::Layer1, config.layer1_buffer.is_some());
        self.ltdc.enable_layer(LtdcLayer::Layer2, config.layer1_buffer.is_some());
    
        /* enable dithering to add artsy graphical artifacts */
        self.ltdc.enable_dithering(true);
    
        /* update settings (this is sometimes needed..) */
        self.ltdc.reload(LtdcReload::Immediate);
        while !self.ltdc.ready() {}
    
        if local_config { self.ltdc.config_access_end(); }

        line_byte_pitch / color_coding_byte_size
    }
}




pub struct LtdcConfig {
    pub polarity: LtdcPolarity,
    pub timing: LtdcTiming,
    pub color_coding: LtdcColorCoding,
    pub color_key: Option<u32>,
    pub layer1_buffer: Option<u32>,
    pub layer2_buffer: Option<u32>,
}



/// Ltdc Pins (for rgb565)
pub trait Pins<Ltdc> {}
impl<Ltdc, 
    CLK, DE,
    HSYNC, VSYNC,
//    R0, R1, 
    R2, R3, R4, R5, R6, R7,
//    G0, G1, 
    G2, G3, G4, G5, G6, G7,
//    B0, B1, 
    B2, B3, B4, B5, B6, B7,
> Pins<Ltdc>
for (
    CLK, DE,
    HSYNC, VSYNC,
//    R0, R1, 
    R2, R3, R4, R5, R6, R7,
//    G0, G1, 
    G2, G3, G4, G5, G6, G7,
//    B0, B1, 
    B2, B3, B4, B5, B6, B7,
)
where
    CLK: PinClk<Ltdc>,
    DE: PinDe<Ltdc>,
    HSYNC: PinHsync<Ltdc>,
    VSYNC: PinVsync<Ltdc>,
//    R0: PinR0<Ltdc>,
//    R1: PinR1<Ltdc>,
    R2: PinR2<Ltdc>,
    R3: PinR3<Ltdc>,
    R4: PinR4<Ltdc>,
    R5: PinR5<Ltdc>,
    R6: PinR6<Ltdc>,
    R7: PinR7<Ltdc>,
//    G0: PinG0<Ltdc>,
//    G1: PinG1<Ltdc>,
    G2: PinG2<Ltdc>,
    G3: PinG3<Ltdc>,
    G4: PinG4<Ltdc>,
    G5: PinG5<Ltdc>,
    G6: PinG6<Ltdc>,
    G7: PinG7<Ltdc>,
//    B0: PinB0<Ltdc>,
//    B1: PinB1<Ltdc>,
    B2: PinB2<Ltdc>,
    B3: PinB3<Ltdc>,
    B4: PinB4<Ltdc>,
    B5: PinB5<Ltdc>,
    B6: PinB6<Ltdc>,
    B7: PinB7<Ltdc>,
{}
