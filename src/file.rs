// read and write to file on disk


#[derive(Debug, Serialize, Deserialize)]
pub struct WhisperFile {
    header: Header,
    data: Data,
}

impl WhisperFile {
    pub fn new(header: Header, data: Data) -> WhisperFile {
        WhisperFile {
            header: header,
            data: data,
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}


// 16 + (12 * num)
#[derive(Debug, Serialize, Deserialize)]
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

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn archive_info(&self) -> &[ArchiveInfo] {
        &self.archive_info
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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


// 4 + 4 + 4 + 4 = 16
#[derive(Debug, Serialize, Deserialize)]
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

    pub fn aggregation(&self) -> AggregationType {
        self.aggregation
    }

    pub fn max_retention(&self) -> u32 {
        self.max_retention
    }

    pub fn x_files_factor(&self) -> f32 {
        self.x_files_factor
    }

    pub fn archive_count(&self) -> u32 {
        self.archive_count
    }
}


// 4 + 4 + 4 = 12
#[derive(Debug, Serialize, Deserialize)]
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

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn seconds_per_point(&self) -> u32 {
        self.seconds_per_point
    }

    pub fn num_points(&self) -> u32 {
        self.num_points
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    archives: Vec<Archive>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Archive {
    points: Vec<Point>,
}

// 4 + 8 = 12
#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    timestamp: u32,
    value: f64,
}


#[cfg(test)]
mod tests {
    use super::{WhisperFile, Header, AggregationType, Metadata,
                ArchiveInfo, Data, Archive, Point};

    #[test]
    fn test_serialize_to_json() {
        let p1 = Point { timestamp: 1501355048, value: 45.9 };
        let p2 = Point { timestamp: 1501355058, value: 30.8 };
        let p3 = Point { timestamp: 1501355068, value: 46.0 };
        let p4 = Point { timestamp: 1501355078, value: 35.3 };

        let a1 = Archive { points: vec![p1, p2, p3, p4] };
        let d1 = Data { archives: vec![a1] };

        let m1 = Metadata {
            aggregation: AggregationType::Max,
            max_retention: 300,
            x_files_factor: 0.3,
            archive_count: 1,
        };

        let ai1 = ArchiveInfo {
            offset: 28,
            seconds_per_point: 10,
            num_points: 30,
        };

        let h1 = Header {
            metadata: m1,
            archive_info: vec![ai1],
        };

        let wf = WhisperFile {
            header: h1,
            data: d1,
        };

        use serde_json;
        println!("File JSON: {}", serde_json::to_string(&wf).unwrap());
    }
}
