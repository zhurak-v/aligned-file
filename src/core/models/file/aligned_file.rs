use std::cmp::Ordering;

use crate::core::models::alignment::alignment::Alignment;
use crate::core::models::collection::queue::tree_queue::TreeQueue;
use crate::core::models::comparator::comparator::Comparator;
use crate::core::models::version::version::Version;

pub struct AlignedFile {
    header: Header,
}

impl AlignedFile {
    pub fn new(
        alignment: Alignment,
        offsets_count: u32
    ) -> AlignedFile {
        let header: Header = Header::new(alignment, offsets_count);
        AlignedFile { header }
    }
}

pub struct Header {
    magic: [u8; 8],
    version: Version,
    alignment: Alignment,
    offsets_count: u32,
    offsets: TreeQueue<u64>,
}

impl Header {
    const MAGIC: [u8; 8] = *b"zhurak-v";
    const VERSION: Version = Version::new(1, 0, 0);
}

impl Header {
    pub fn new(
        alignment: Alignment,
        offsets_count: u32,
    ) -> Header {
        let comparator: OffsetComparator = OffsetComparator::new(alignment);
        Header {
            magic: Header::MAGIC,
            version: Header::VERSION,
            alignment,
            offsets_count,
            offsets: TreeQueue::new(comparator),
        }
    }
}

struct OffsetComparator {
    alignment: Alignment,
}

impl OffsetComparator {
    #[inline]
    pub const fn new(alignment: Alignment) -> OffsetComparator {
        OffsetComparator { alignment }
    }
}

impl Comparator<u64> for OffsetComparator {
    #[inline]
    fn compare(&self, a: &u64, b: &u64) -> Ordering {
        let distance_a: u64 = Alignment::round_up(
            *a as usize,
            (*self).alignment
        ) as u64 - a;
        let distance_b: u64 = Alignment::round_up(
            *b as usize,
            (*self).alignment
        ) as u64 - b;

        distance_b.cmp(&distance_a)
    }
}