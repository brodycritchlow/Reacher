use super::SearchBuilder;
use ignore::DirEntry;
use std::{cmp::Ordering, time::SystemTime};
pub type FilterFn = fn(&DirEntry) -> bool; // Initialize type that represents function that returns bool

pub enum FilterType {
    Created(Ordering, SystemTime),
    Modified(Ordering, SystemTime),
    FileSize(Ordering, u64),
    Custom(FilterFn)
}

#[derive(Debug, Clone)]
pub enum FileSize {
    Byte(u64),
    Kilobyte(f64),
    Megabyte(f64),
    Gigabyte(f64),
    Terabyte(f64),
}

pub trait FilterExt {
    fn created_before(self, t: SystemTime) -> Self;
    fn created_at(self, t: SystemTime) -> Self;
    fn created_after(self, t: SystemTime) -> Self;
    fn modified_before(self, t: SystemTime) -> Self;
    fn modified_at(self, t: SystemTime) -> Self;
    fn modified_after(self, t: SystemTime) -> Self;
    fn file_size_smaller(self, size: FileSize) -> Self;
    fn file_size_equal(self, size: FileSize) -> Self;
    fn file_size_greater(self, size: FileSize) -> Self;
    fn custom_filter(self, f: FilterFn) -> Self;
}

fn convert(b: f64, pow: u32) -> u64 {
    (b * 1024_u64.pow(pow) as f64) as u64
}
#[allow(clippy::from_over_into)] // Removes warning suggesting conversion to From<>
impl Into<u64> for FileSize {
    fn into(self) -> u64 {
        use self::FileSize::{Byte, Gigabyte, Kilobyte, Megabyte, Terabyte};

        match self {
            Byte(b) => b,
            Kilobyte(b) => convert(b, 1),
            Megabyte(b) => convert(b, 2),
            Gigabyte(b) => convert(b, 3),
            Terabyte(b) => convert(b, 4),
        }
    }
}

use FilterType::{Created, Custom, FileSize as FilterFileSize, Modified};
use Ordering::{Equal, Greater, Less};

impl FilterExt for SearchBuilder {
    fn created_before(self, t: SystemTime) -> Self {
        self.filter(Created(Less, t))
    }

    fn created_at(self, t: SystemTime) -> Self {
        self.filter(Created(Equal, t))
    }

    fn created_after(self, t: SystemTime) -> Self {
        self.filter(Created(Greater, t))
    }

    fn modified_before(self, t: SystemTime) -> Self {
        self.filter(Modified(Less, t))
    }

    fn modified_at(self, t: SystemTime) -> Self {
        self.filter(Modified(Equal, t))
    }

    fn modified_after(self, t: SystemTime) -> Self {
        self.filter(Modified(Greater, t))
    }

    fn file_size_smaller(self, size: FileSize) -> Self {
        self.filter(FilterFileSize(Less, size.into()))
    }

    fn file_size_equal(self, size: FileSize) -> Self {
        self.filter(FilterFileSize(Equal, size.into()))
    }

    fn file_size_greater(self, size: FileSize) -> Self {
        self.filter(FilterFileSize(Greater, size.into()))
    }
    fn custom_filter(self, f: FilterFn) -> Self {
        self.filter(Custom(f))
    }
}