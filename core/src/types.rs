// Memento - A Whisper implementation in Rust
//
// Copyright 2017-2018 TSH Labs
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Structures that define the Whisper file format on disk

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MementoDatabase {
    header: Header,
    data: Data,
}

impl MementoDatabase {
    pub fn new(header: Header, data: Data) -> MementoDatabase {
        MementoDatabase {
            header: header,
            data: data,
        }
    }

    #[inline]
    pub fn header(&self) -> &Header {
        &self.header
    }

    #[inline]
    pub fn data(&self) -> &Data {
        &self.data
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Header {
    metadata: Metadata,
    archive_info: Vec<ArchiveInfo>,
}

impl Header {
    pub fn new(metadata: Metadata, archive_info: Vec<ArchiveInfo>) -> Header {
        Header {
            metadata: metadata,
            archive_info: archive_info,
        }
    }

    #[inline]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    #[inline]
    pub fn archive_info(&self) -> &[ArchiveInfo] {
        &self.archive_info
    }

    /// Get the amount of space required for the file header in bytes
    #[inline]
    pub fn size(&self) -> u64 {
        Metadata::storage() + (ArchiveInfo::storage() * self.metadata.archive_count() as u64)
    }

    /// Get the amount of space required for the entire file in bytes
    #[inline]
    pub fn file_size(&self) -> u64 {
        self.archive_info()
            .iter()
            .fold(self.size(), |acc, info| acc + info.archive_size())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AggregationType {
    Average = 1,
    Sum = 2,
    Last = 3,
    Max = 4,
    Min = 5,
    AvgZero = 6,
    AbsMax = 7,
    AbsMin = 8,
}

impl Default for AggregationType {
    fn default() -> AggregationType {
        AggregationType::Average
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Metadata {
    aggregation: AggregationType,
    max_retention: u32,
    x_files_factor: f32,
    archive_count: u32,
}

impl Metadata {
    pub fn new(
        aggregation: AggregationType,
        max_retention: u32,
        x_files_factor: f32,
        archive_count: u32,
    ) -> Metadata {
        Metadata {
            aggregation: aggregation,
            max_retention: max_retention,
            x_files_factor: x_files_factor,
            archive_count: archive_count,
        }
    }

    #[inline]
    pub fn storage() -> u64 {
        16 /* bytes required for an instance */
    }

    #[inline]
    pub fn aggregation(&self) -> AggregationType {
        self.aggregation
    }

    #[inline]
    pub fn max_retention(&self) -> u32 {
        self.max_retention
    }

    #[inline]
    pub fn x_files_factor(&self) -> f32 {
        self.x_files_factor
    }

    #[inline]
    pub fn archive_count(&self) -> u32 {
        self.archive_count
    }

    #[inline]
    pub fn archive_info_size(&self) -> u64 {
        self.archive_count as u64 * ArchiveInfo::storage()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArchiveInfo {
    offset: u32,
    seconds_per_point: u32,
    num_points: u32,
}

impl ArchiveInfo {
    pub fn new(offset: u32, seconds_per_point: u32, num_points: u32) -> ArchiveInfo {
        ArchiveInfo {
            offset: offset,
            seconds_per_point: seconds_per_point,
            num_points: num_points,
        }
    }

    #[inline]
    pub fn storage() -> u64 {
        12 /* bytes required for an instance */
    }

    #[inline]
    pub fn archive_size(&self) -> u64 {
        Point::storage() * self.num_points as u64
    }

    #[inline]
    pub fn offset(&self) -> u32 {
        self.offset
    }

    #[inline]
    pub fn seconds_per_point(&self) -> u32 {
        self.seconds_per_point
    }

    #[inline]
    pub fn num_points(&self) -> u32 {
        self.num_points
    }

    #[inline]
    pub fn retention(&self) -> u32 {
        self.num_points * self.seconds_per_point
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Data {
    archives: Vec<Archive>,
}

impl Data {
    pub fn new(archives: Vec<Archive>) -> Data {
        Data { archives: archives }
    }

    #[inline]
    pub fn archives(&self) -> &[Archive] {
        &self.archives
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Archive {
    points: Vec<Point>,
}

impl Archive {
    pub fn new(points: Vec<Point>) -> Archive {
        Archive { points: points }
    }

    #[inline]
    pub fn points(&self) -> &[Point] {
        &self.points
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Point {
    timestamp: u32,
    value: f64,
}

impl Point {
    pub fn new(timestamp: u32, value: f64) -> Point {
        Point {
            timestamp: timestamp,
            value: value,
        }
    }

    #[inline]
    pub fn storage() -> u64 {
        12 /* bytes required for an instance */
    }

    #[inline]
    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }

    #[inline]
    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::{AggregationType, ArchiveInfo, Header, Metadata};

    #[test]
    fn test_header_size() {
        let metadata = Metadata::new(AggregationType::Average, 31536000, 0.5, 5);
        let info1 = ArchiveInfo::new(76, 10, 8640);
        let info2 = ArchiveInfo::new(103756, 60, 10080);
        let info3 = ArchiveInfo::new(224716, 300, 8640);
        let info4 = ArchiveInfo::new(328396, 600, 25920);
        let info5 = ArchiveInfo::new(639436, 3600, 8760);

        let header = Header::new(metadata, vec![info1, info2, info3, info4, info5]);

        assert_eq!(76, header.size());
        assert_eq!(744556, header.file_size());
    }
}
