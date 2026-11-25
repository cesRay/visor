/// Files larger than this are not previewed
pub const MAX_PREVIEW_BYTES: u64 = 1_000_000;

/// A NUL byte in this many leading bytes marks a file as binary
pub const BINARY_SNIFF_BYTES: usize = 8192;
