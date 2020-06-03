use super::*;
use crate::*;
use paired::bls12_381::{Bls12, Fr};

fn main() {
    let constants = PoseidonConstants::<Bls12, U2>::new();
    println!("mds_matrices: {:?}, {:?}", constants.mds_matrices.m, constants.mds_matrices.m_hat);
    println!("len round_constants: {:?}", constants.round_constants.len());
    println!("len round_constants: {:?}", constants.round_constants.len());
    println!("len compressed_round_constants: {:?}", constants.compressed_round_constants.len());
    println!("pre_sparse_matrix: {:?}", constants.pre_sparse_matrix);
    println!("len sparse_matrixes: {:?}", constants.sparse_matrixes.len());
    println!("arity_tag: {:?}", constants.arity_tag);
}