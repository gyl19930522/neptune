use paired::bls12_381::Bls12;
use generic_array::typenum::{U2, U11};
use neptune::poseidon::{PoseidonConstants, Arity};

fn template1() {
    let constants = PoseidonConstants::<Bls12, U2>::new();
    println!("mds_matrices: {:?}", constants.mds_matrices.m);
    print!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_inv);
    print!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_hat);
    print!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_hat_inv);
    print!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_prime);
    print!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_double_prime);
    print!("*********************************************************");
    println!("len round_constants: {:?}", constants.round_constants.len());
    println!("len compressed_round_constants: {:?}", constants.compressed_round_constants.len());
    println!("pre_sparse_matrix: {:?}", constants.pre_sparse_matrix);
    println!("len sparse_matrixes: {:?}", constants.sparse_matrixes.len());
    println!("arity_tag: {:?}", constants.arity_tag);
    println!("full_rounds: {:?}", constants.full_rounds);
    println!("half_full_rounds: {:?}", constants.half_full_rounds);
    println!("partial_rounds: {:?}", constants.partial_rounds);
}

fn template2() {
    let constants = PoseidonConstants::<Bls12, U11>::new();
    println!("mds_matrices: {:?}", constants.mds_matrices.m);
    println!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_inv);
    println!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_hat);
    println!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_hat_inv);
    println!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_prime);
    println!("*********************************************************");
    println!("mds_matrices: {:?}", constants.mds_matrices.m_double_prime);
    println!("*********************************************************");
    println!("len round_constants: {:?}", constants.round_constants.len());
    println!("len compressed_round_constants: {:?}", constants.compressed_round_constants.len());
    println!("pre_sparse_matrix: {:?}", constants.pre_sparse_matrix);
    println!("len sparse_matrixes: {:?}", constants.sparse_matrixes.len());
    println!("arity_tag: {:?}", constants.arity_tag);
    println!("full_rounds: {:?}", constants.full_rounds);
    println!("half_full_rounds: {:?}", constants.half_full_rounds);
    println!("partial_rounds: {:?}", constants.partial_rounds);
}

fn main() {
    template1();
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    template2();
}