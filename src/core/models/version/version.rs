pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
}

impl Version {
    #[inline]
    pub const fn new(
        major: u16, 
        minor: u16, 
        patch: u16
    ) -> Version {
        Version {
            major, 
            minor, 
            patch,
        }
    }
}

impl Version {
    #[inline]
    pub fn major(&self) -> u16 {
        (*self).major
    }

    #[inline]
    pub fn minor(&self) -> u16 {
        (*self).minor
    }

    #[inline]
    pub fn patch(&self) -> u16 {
        (*self).patch
    }
}