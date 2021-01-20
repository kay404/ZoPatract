pub const BELLMAN: &str = "bellman";
#[cfg(feature = "libsnark")]
pub const LIBSNARK: &str = "libsnark";
pub const ARK: &str = "ark";

#[cfg(feature = "libsnark")]
pub const BACKENDS: &[&str] = &[BELLMAN, LIBSNARK, ARK];
#[cfg(not(feature = "libsnark"))]
pub const BACKENDS: &[&str] = &[BELLMAN, ARK];

pub const BN128: &str = "bn128";
pub const BLS12_381: &str = "bls12_381";
pub const BLS12_377: &str = "bls12_377";
pub const BW6_761: &str = "bw6_761";
pub const CURVES: &[&str] = &[BN128, BLS12_381, BLS12_377, BW6_761];

pub const G16: &str = "g16";
#[cfg(feature = "libsnark")]
pub const PGHR13: &str = "pghr13";
pub const GM17: &str = "gm17";

#[cfg(feature = "libsnark")]
pub const SCHEMES: &[&str] = &[G16, PGHR13, GM17];
#[cfg(not(feature = "libsnark"))]
pub const SCHEMES: &[&str] = &[G16, GM17];
