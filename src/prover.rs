use crate::commitments::{Commitment, Randomness, get_column, get_row, get_subgrid};
use crate::types::{ProofError, Challenge, Statement, Witness};
use sudoku::Sudoku;
use rand::prelude::SliceRandom;

pub struct Prover {
    witness: Witness,
    pub statement: Statement
}

pub struct PermutedCommittedSudoku {
    pub commitments: Vec<Commitment>,
    pub randomness: Vec<Randomness>,
    pub permuted_sudoku: Vec<u8>
}

impl Prover {
    pub fn sudoku_instance(sudoku: Sudoku, solution: Sudoku) -> Prover {
        Prover {
            statement: Statement{sudoku},
            witness: Witness{solution}
        }
    }

    pub fn permute_and_commit(&self) -> PermutedCommittedSudoku {
        let mut rng = rand::thread_rng();
        let mut permutation: Vec<u8> = (1..=9).collect();
        permutation.shuffle(&mut rng);

        let mut commitments: Vec<Commitment> = Vec::new();
        let mut randomness: Vec<Randomness> = Vec::new();
        let mut permuted_sudoku: Vec<u8> = Vec::new();

        for &value in self.witness.solution.to_bytes().iter() {
            let permuted_val = permutation[(value-1) as usize];
            let (commitment, random) = Commitment::new(&[permuted_val]);
            commitments.push(commitment);
            randomness.push(random);
            permuted_sudoku.push(permuted_val);
        }

        PermutedCommittedSudoku {
            commitments,
            randomness,
            permuted_sudoku
        }
    }

    pub fn reveal(&self, permuted_sudoku: &PermutedCommittedSudoku, challenge: &Challenge) -> Result<(Vec<u8>, Vec<Randomness>), ProofError> {
        match challenge.0 {
            0..=8 => {
                let row_index = (challenge.0 as usize) + 1;
                let row = get_row(row_index, &permuted_sudoku.permuted_sudoku);
                let random_values = get_row(row_index, &permuted_sudoku.randomness);
                Ok((row, random_values))
            },
            9..=17 => {
                let col_index = (challenge.0 - 9) as usize;
                let column = get_column(col_index, &permuted_sudoku.permuted_sudoku);
                let random_values = get_column(col_index, &permuted_sudoku.randomness);
                Ok((column, random_values))
            }
            18..=26 => {
                let subgrid_number = (challenge.0 - 18) as usize;
                let subgrid = get_subgrid(subgrid_number, &permuted_sudoku.permuted_sudoku);
                let random_values = get_subgrid(subgrid_number, &permuted_sudoku.randomness);
                Ok((subgrid, random_values))
            }
            27 => {
                let mut known_values: Vec<u8> = Vec::new();
                let mut random_values: Vec<Randomness> = Vec::new();
                for (index, value) in self.statement.sudoku.iter().enumerate() {
                    if let Some(v) = value {
                        known_values.push(permuted_sudoku.permuted_sudoku[index].clone());
                    } else {
                        known_values.push(0);
                    }

                    random_values.push(permuted_sudoku.randomness[index].clone());
                }
                Ok((known_values, random_values))
            }
            _ => Err(ProofError::InvalidChallenge)
        }
    }
}