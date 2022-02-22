// Eventually change to some hash of the resource
type Id = i64;

struct Settings {
    // could maintain separate counters for all blocktypes
    // but hashing largely eliminates the question
}

struct DataGraph {
    settings: Settings,
    authors: Vec<Author>,
}

struct Author {
    author_id: Id,
    name: String,
}

struct Metadata {
    block_id: Id,
    title: String,
    author: Author,
    uri: Uri,
}

enum BlockType {
    Markdown,
    Webpage, // Bookmark
    Json,
    Csv,
}

struct Block {
    block_id: Id,
    block_type: BlockType,
    metadata: Metadata,
}

#[non_exhaustive]
enum ExportTypes {
    /// Compute and return the transitive closure from a Block
    SubGraph {depth: i16},
    StaticSite,
    // https://lib.rs/crates/qrcode
    QRCode,
    // https://crates.io/crates/qair
    Qair,
}