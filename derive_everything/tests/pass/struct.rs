use derive_everything::{derive_everything, derive_float_everything};

#[derive_everything]
struct Trivial;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ProofThatTheyAreDerived(Trivial);

#[derive_everything]
struct MoreComplicated {
    num: u64,
    string: String,
    vec: Vec<u64>,
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ProofThatTheyAreAlsoDerived(MoreComplicated);

#[derive_float_everything]
struct WithFloat {
    num: f64,
    string: String,
    vec: Vec<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
struct ProofThatTheyAreDerivedForFloat(WithFloat);

fn main() {}
