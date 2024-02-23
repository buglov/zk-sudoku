use crate::commitments::{Commitment, Randomness, get_column, get_row, get_subgrid};
use crate::types::{ProofError, Challenge, Statement, Witness};
use crate::prover::{Prover, PermutedCommittedSudoku};
use sudoku::Sudoku;
use rand::prelude::SliceRandom;
use std::collections::HashSet;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

pub struct Verifier {
    statement: Statement,
    commitments: Vec<Commitment>
}

impl Verifier {
    pub fn new(statement: Statement, commitments: Vec<Commitment>) -> Result<Verifier, ProofError> {
        if statement.sudoku.clone().solutions_count_up_to(1) != 1 {
            return Err(ProofError::UnsolvableSudoku);
        }

        Ok(Verifier{
            statement,
            commitments
        })
    }

    pub fn generate_challenge(&self) -> Challenge {
        let mut rng = StdRng::from_entropy();
        let random_number = rng.gen_range(0..27);
        Challenge(random_number)
    }

    pub fn check(
        &self,
        challenge: &Challenge,
        decommitments: &(Vec<u8>, Vec<Randomness>),
    ) -> Result<(), ProofError> {
        let (messages, randomness) = decommitments;

        if challenge.0 != 27 && messages.len() != HashSet::<&u8>::from_iter(messages.iter()).len() {
            return Err(ProofError::NonUniqueValues);
        }

        match challenge.0 {
            0..=8 => {
                let row_index = (challenge.0 as usize) + 1;
                let commitments = get_row(row_index, &self.commitments);

                for i in 0..9 {
                    if !commitments[i].verify(&[messages[i]], &randomness[i]) {
                        return Err(ProofError::CommitmentMismatch);
                    }
                }
            }
            9..=17 => {
                let col_index = (challenge.0 - 9) as usize;
                let commitments = get_column(col_index, &self.commitments);

                for i in 0..9 {
                    if !commitments[i].verify(&[messages[i]], &randomness[i]) {
                        return Err(ProofError::CommitmentMismatch);
                    }
                }
            }
            18..=26 => {
                let subgrid_number = (challenge.0 - 18) as usize;
                let commitments = get_subgrid(subgrid_number, &self.commitments);

                for i in 0..9 {
                    if !commitments[i].verify(&[messages[i]], &randomness[i]) {
                        return Err(ProofError::CommitmentMismatch);
                    }
                }
            }
            27 => {
                for (i, value) in self.statement.sudoku.iter().enumerate() {
                    if let Some(v) = value {
                        if !&self.commitments[i].verify(&[messages[i]], &randomness[i]) {
                            return Err(ProofError::CommitmentMismatch);
                        }
                    }
                }
            }
            _ => {
                return Err(ProofError::InvalidChallenge);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prover_verifier() {
        let sudoku: Sudoku = Sudoku::generate();
        let solved: Sudoku = sudoku.solution().unwrap();

        let prover: Prover = Prover::sudoku_instance(sudoku.clone(), solved.clone());

        let permuted_committed_sudoku: PermutedCommittedSudoku = prover.permute_and_commit();

        let verifier: Verifier = Verifier::new(prover.statement.clone(), permuted_committed_sudoku.commitments.clone()).unwrap();
        
        let challenge: Challenge = verifier.generate_challenge();

        let proof = prover
            .reveal(&permuted_committed_sudoku, &challenge)
            .unwrap();
        verifier.check(&challenge, &proof).unwrap();
    }
}