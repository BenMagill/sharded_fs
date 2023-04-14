// Keeps track of what files exist and if not where they are

// Like s3, this refers to a file
struct Object {
    // TODO: use a fixed size string type (as name will be hashed)
    name: String,
    // TODO: use fixed size type
    mime: String,
    in_fs: bool, // Does it exist on this server?
}

struct Library {
    files: Vec<Object>
}