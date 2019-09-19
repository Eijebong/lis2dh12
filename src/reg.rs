#![allow(non_upper_case_globals)]

#[cfg(feature = "out_f32")]
use cast::f32;

/// I2C slave address
pub const I2C_SAD: u8 = 0b001_1000;

pub const I2C_SUB_MULTI: u8 = 0b1000_0000;

/// Operating mode
pub enum Mode {
    /// High-resolution mode (12-bit data output)
    HighResolution,
    /// Normal mode (10-bit data output)
    Normal,
    /// Low-power mode (8-bit data output)
    LowPower,
}

/// Register mapping
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Register {
    STATUS_REG_AUX = 0x07,
    OUT_TEMP_L = 0x0C,
    OUT_TEMP_H = 0x0D,
    WHO_AM_I = 0x0F,
    CTRL_REG0 = 0x1E,
    TEMP_CFG_REG = 0x1F,
    CTRL_REG1 = 0x20,
    CTRL_REG2 = 0x21,
    CTRL_REG3 = 0x22,
    CTRL_REG4 = 0x23,
    CTRL_REG5 = 0x24,
    CTRL_REG6 = 0x25,
    REFERENCE = 0x26,
    STATUS_REG = 0x27,
    OUT_X_L = 0x28,
    OUT_X_H = 0x29,
    OUT_Y_L = 0x2A,
    OUT_Y_H = 0x2B,
    OUT_Z_L = 0x2C,
    OUT_Z_H = 0x2D,
    FIFO_CTRL_REG = 0x2E,
    FIFO_SRC_REG = 0x2F,
    INT1_CFG = 0x30,
    INT1_SRC = 0x31,
    INT1_THS = 0x32,
    INT1_DURATION = 0x33,
    INT2_CFG = 0x34,
    INT2_SRC = 0x35,
    INT2_THS = 0x36,
    INT2_DURATION = 0x37,
    CLICK_CFG = 0x38,
    CLICK_SRC = 0x39,
    CLICK_THS = 0x3A,
    TIME_LIMIT = 0x3B,
    TIME_LATENCY = 0x3C,
    TIME_WINDOW = 0x3D,
    ACT_THS = 0x3E,
    ACT_DUR = 0x3F,
}

impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }
}

// === STATUS_REG_AUX (07h) ===

pub const TOR: u8 = 0b0100_0000;
pub const TDA: u8 = 0b0000_0100;

// === WHO_AM_I (0Fh) ===

/// WHO_AM_I device identification register
pub const DEVICE_ID: u8 = 0b0011_0011;

// === TEMP_CFG_REG (1Fh) ===

pub const TEMP_EN: u8 = 0b1100_0000;

// === CTRL_REG1 (20h) ===

pub const ODR_MASK: u8 = 0b1111_0000;

/// Output Data Rate
#[derive(Copy, Clone)]
pub enum Odr {
    /// Power-down mode
    PowerDown = 0b0000,
    /// 1 Hz
    Hz1 = 0b0001,
    /// 10 Hz
    Hz10 = 0b0010,
    /// 25 Hz
    Hz25 = 0b0011,
    /// 50 Hz
    Hz50 = 0b0100,
    /// 100 Hz
    Hz100 = 0b0101,
    /// 200 Hz
    Hz200 = 0b0110,
    /// 400 Hz
    Hz400 = 0b0111,
    /// Low-power mode (1.620 kHz)
    HighRate0 = 0b1000,
    /// High-resolution / Normal (1.344 kHz),
    /// Low-power (5.376 kHz)
    HighRate1 = 0b1001,
}

pub const LPen: u8 = 0b0000_1000;
pub const Zen: u8 = 0b0000_0100;
pub const Yen: u8 = 0b0000_0010;
pub const Xen: u8 = 0b0000_0001;

// === CTRL_REG4 (23h) ===

pub const BDU: u8 = 0b1000_0000;

pub const FS_MASK: u8 = 0b0011_0000;

/// Full-scale selection
#[derive(Copy, Clone)]
pub enum FullScale {
    /// ±2 g
    G2 = 0b00,
    /// ±4 g
    G4 = 0b01,
    /// ±8 g
    G8 = 0b10,
    /// ±16 g
    G16 = 0b11,
}

impl FullScale {
    #[cfg(feature = "out_f32")]
    pub(crate) fn convert_i16tof32(self, val: i16) -> f32 {
        // mg/digit for high-resolution mode (12-bit)
        let sens: f32 = match self {
            Self::G2 => 0.001,
            Self::G4 => 0.002,
            Self::G8 => 0.004,
            Self::G16 => 0.012,
        };
        // up to 12-bit data, left-justified
        f32(val >> 4) * sens
    }
}

pub const HR: u8 = 0b0000_1000;

// === CTRL_REG5 (24h) ===

pub const FIFO_EN: u8 = 0b0100_0000;

// === STATUS_REG (27h) ===

pub const ZYXOR: u8 = 0b1000_0000;
pub const ZOR: u8 = 0b0100_0000;
pub const YOR: u8 = 0b0010_0000;
pub const XOR: u8 = 0b0001_0000;
pub const ZYXDA: u8 = 0b0000_1000;
pub const ZDA: u8 = 0b0000_0100;
pub const YDA: u8 = 0b0000_0010;
pub const XDA: u8 = 0b0000_0001;

// === FIFO_CTRL_REG (2Eh) ===

pub const FM_MASK: u8 = 0b1100_0000;

/// Full-scale selection
#[derive(Copy, Clone)]
pub enum FifoMode {
    /// Bypass mode
    Bypass = 0b00,
    /// FIFO mode
    Fifo = 0b01,
    /// Stream mode
    Stream = 0b10,
    /// Stream-to-FIFO mode
    StreamToFifo = 0b11,
}

pub const FTH_MASK: u8 = 0b0001_1111;