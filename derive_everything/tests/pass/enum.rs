use derive_everything::{derive_enum_everything, derive_everything, derive_float_enum_everything};

#[derive_everything]
enum Trivial {
    #[default]
    A,
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ProofThatTheyAreDerived(Trivial);

#[derive_enum_everything]
enum MoreComplicated {
    Num(u64),
    Str { string: String },
    Vector(Vec<u64>),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ProofThatTheyAreAlsoDerived(MoreComplicated);

#[derive_float_enum_everything]
enum WithFloat {
    Num(f64),
    Str { string: String },
    Vector(Vec<u64>),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct ProofThatTheyAreDerivedForFloat(WithFloat);

fn main() {}
