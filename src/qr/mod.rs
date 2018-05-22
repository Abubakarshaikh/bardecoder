use std::error::Error;
use std::fmt;
use std::ops::Index;

use point::Point;

use self::format::ECLevel;

pub mod blocks;
pub mod correct;
pub mod data;
pub mod format;

#[derive(Debug)]
pub struct QRData {
    pub data: Vec<u8>,
    pub version: u32,
    pub side: u32,
}

impl QRData {
    pub fn new(data: Vec<u8>, version: u32) -> QRData {
        QRData {
            data,
            version,
            side: 4 * version + 17,
        }
    }
}

impl Index<[u32; 2]> for QRData {
    type Output = u8;

    fn index(&self, index: [u32; 2]) -> &u8 {
        let pixel = self.data[index[1] as usize * self.side as usize + index[0] as usize];
        if pixel == 0 {
            &1
        } else {
            &0
        }
    }
}

#[derive(Debug)]
pub struct QRLocation {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub module_size: f64,
    pub version: u32,
}

#[derive(Debug)]
pub struct QRFinderPosition {
    pub location: Point,
    pub module_size: f64,
}

#[derive(Debug)]
pub struct BlockInfo {
    pub block_count: u8,
    pub total_per: u8,
    pub data_per: u8,
    pub ec_cap: u8,
}

impl BlockInfo {
    pub fn new(block_count: u8, total_per: u8, data_per: u8, ec_cap: u8) -> BlockInfo {
        BlockInfo {
            block_count,
            total_per,
            data_per,
            ec_cap,
        }
    }
}

pub fn block_info(version: u32, level: ECLevel) -> Option<Vec<BlockInfo>> {
    match (version, level) {
        (1, ECLevel::MEDIUM) => Some(vec![BlockInfo::new(1, 26, 16, 4)]),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct QRError {
    msg: String,
}

impl Error for QRError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl fmt::Display for QRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "QRError: {}", self.msg)
    }
}