use crate::Error;
use std::pin::Pin;

const BUF_DATA_SMALL_SIZE: usize = 32;
const BUF_DATA_MEDIUM_SIZE: usize = 128;
const BUF_DATA_LARGE_SIZE: usize = 1024;
const BUF_DATA_MEGA_SIZE: usize = 1024 * 64;

pub enum BufData {
    Small(Pin<Box<[u8; BUF_DATA_SMALL_SIZE]>>),
    Medium(Pin<Box<[u8; BUF_DATA_MEDIUM_SIZE]>>),
    Large(Pin<Box<[u8; BUF_DATA_LARGE_SIZE]>>),
    Mega(Pin<Box<[u8; BUF_DATA_MEGA_SIZE]>>),
}

impl BufData {
    pub fn new_small() -> BufData {
        BufData::Small(Box::pin([0u8; BUF_DATA_SMALL_SIZE]))
    }

    pub fn new_medium() -> BufData {
        BufData::Medium(Box::pin([0u8; BUF_DATA_MEDIUM_SIZE]))
    }

    pub fn new_large() -> BufData {
        BufData::Large(Box::pin([0u8; BUF_DATA_LARGE_SIZE]))
    }

    pub fn new_mega() -> BufData {
        BufData::Mega(Box::pin([0u8; BUF_DATA_MEGA_SIZE]))
    }

    pub fn get(&self) -> Pin<&[u8]> {
        match self {
            BufData::Small(data) => data.as_ref(),
            BufData::Medium(data) => data.as_ref(),
            BufData::Large(data) => data.as_ref(),
            BufData::Mega(data) => data.as_ref(),
        }
    }

    pub fn get_mut(&mut self) -> Pin<&mut [u8]> {
        match self {
            BufData::Small(data) => data.as_mut(),
            BufData::Medium(data) => data.as_mut(),
            BufData::Large(data) => data.as_mut(),
            BufData::Mega(data) => data.as_mut(),
        }
    }
}

pub struct Buf {
    data: BufData,
    size: usize,
}

impl Buf {
    pub fn new(size: usize) -> Result<Buf, Error> {
        if size <= BUF_DATA_SMALL_SIZE {
            Ok(Buf {
                data: BufData::new_small(),
                size,
            })
        } else if size <= BUF_DATA_MEDIUM_SIZE {
            Ok(Buf {
                data: BufData::new_medium(),
                size,
            })
        } else if size < BUF_DATA_LARGE_SIZE {
            Ok(Buf {
                data: BufData::new_large(),
                size,
            })
        } else if size < BUF_DATA_MEGA_SIZE {
            Ok(Buf {
                data: BufData::new_mega(),
                size,
            })
        } else {
            Err(Error::BufTooLarge(size))
        }
    }

    pub fn data(&self) -> Pin<&[u8]> {
        self.data.get()
    }

    pub fn data_mut(&mut self) -> Pin<&mut [u8]> {
        self.data.get_mut()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
