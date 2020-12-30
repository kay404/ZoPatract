use bellman_ce::pairing::bls12_381::{Bls12, Fq2};
use ark_bls12_381::Bls12_381;
prime_field!(
    b"52435875175126190479447740508185965837690552500527637822603658699938581184513",
    "bls12_381"
);
bellman_extensions!(Bls12, Fq2);
ark_extensions!(Bls12_381);
