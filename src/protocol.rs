//! Handle switching between register and serial protocol

pub trait FromBufferFallible: Sized {
    fn try_read(buf: &[u8]) -> Option<Self>;
}

pub trait FromBuffer: Sized {
    fn read(buf: &[u8]) -> Self;
}

impl<T: FromBuffer> FromBufferFallible for T {
    fn try_read(buf: &[u8]) -> Option<Self> {
        Some(<Self as FromBuffer>::read(buf))
    }
}



