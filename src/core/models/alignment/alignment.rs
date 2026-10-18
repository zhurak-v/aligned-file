#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Alignment {
    Byte2 = 2,
    Byte4 = 4,
    Byte8 = 8,
    Byte16 = 16,
    Byte32 = 32,
    Byte64 = 64,
    Byte128 = 128,
    Byte256 = 256,
    Byte512 = 512,
    Byte1024 = 1024,
    Byte2048 = 2048,
    Byte4096 = 4096,
    Byte8192 = 8192,
    Byte16384 = 16384,
    Byte32768 = 32768,
    Byte65536 = 65536,
}

impl Alignment {
    #[inline]
    pub const fn round_up(value: usize, alignment: Alignment) -> usize {
        let align: usize = alignment.as_usize();
        (value + align - 1) & !(align - 1)
    }
    
    #[inline]
    pub const fn round_down(value: usize, alignment: Alignment) -> usize {
        let align: usize = alignment.as_usize();
        value & !(align - 1)
    }
}

impl Alignment {
    #[inline]
    pub const fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self as u64
    }

    #[inline]
    pub const fn from_usize(value: usize) -> Option<Alignment> {
        match value {
            2 => Some(Alignment::Byte2),
            4 => Some(Alignment::Byte4),
            8 => Some(Alignment::Byte8),
            16 => Some(Alignment::Byte16),
            32 => Some(Alignment::Byte32),
            64 => Some(Alignment::Byte64),
            128 => Some(Alignment::Byte128),
            256 => Some(Alignment::Byte256),
            512 => Some(Alignment::Byte512),
            1024 => Some(Alignment::Byte1024),
            2048 => Some(Alignment::Byte2048),
            4096 => Some(Alignment::Byte4096),
            8192 => Some(Alignment::Byte8192),
            16384 => Some(Alignment::Byte16384),
            32768 => Some(Alignment::Byte32768),
            65536 => Some(Alignment::Byte65536),
            _ => None,
        }
    }
}