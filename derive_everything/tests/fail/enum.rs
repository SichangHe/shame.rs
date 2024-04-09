use derive_everything::derive_everything;

#[derive_everything]
enum MoreComplicated {
    Num(u64),
    Str { string: String },
    Vector(Vec<u64>),
}

fn main() {}
