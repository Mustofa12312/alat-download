#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub index: u32,
    pub start_byte: u64,
    pub end_byte: u64,
}

pub struct SegmentManager;

impl SegmentManager {
    /// Divides a total size into `num_connections` segments.
    pub fn split_into_segments(total_size: u64, num_connections: u32) -> Vec<Segment> {
        if total_size == 0 || num_connections == 0 {
            return vec![Segment {
                index: 0,
                start_byte: 0,
                end_byte: 0,
            }];
        }

        // If file is too small, use fewer connections
        let min_segment_size = 1024 * 1024; // 1 MB min segment
        let actual_connections = if total_size / (num_connections as u64) < min_segment_size {
            let max_conn = (total_size / min_segment_size) as u32;
            if max_conn > 0 { max_conn } else { 1 }
        } else {
            num_connections
        };

        let mut segments = Vec::new();
        let segment_size = total_size / (actual_connections as u64);
        
        for i in 0..actual_connections {
            let start = (i as u64) * segment_size;
            let end = if i == actual_connections - 1 {
                total_size - 1
            } else {
                start + segment_size - 1
            };

            segments.push(Segment {
                index: i,
                start_byte: start,
                end_byte: end,
            });
        }

        segments
    }
}
