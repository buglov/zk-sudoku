use rand::{Rng, SeedableRng, rngs::StdRng};
use sha2::{Digest, Sha256};
use sudoku::Sudoku;
use rand::prelude::SliceRandom;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Commitment(Vec<u8>);

#[derive(Clone, Debug)]
pub struct Randomness(Vec<u8>);

impl Commitment {
    pub fn new(m: &[u8]) -> (Commitment, Randomness) {
        // generate long random string as vec
        let mut rand = StdRng::from_entropy();
        let mut r = vec![0; 64];
        rand.fill(&mut r[..]);

        // concatened
        let mut concatened = Vec::new();
        concatened.extend_from_slice(m);
        concatened.extend_from_slice(&r);

        // Calculate hash
        let mut hash = Sha256::new();
        hash.update(&concatened);
        let commitment = hash.finalize().to_vec();

        (Commitment(commitment), Randomness(r))
    }

    pub fn verify(&self, m: &[u8], r: &Randomness) -> bool {
        let mut concatened = Vec::new();
        concatened.extend_from_slice(m);
        concatened.extend_from_slice(&r.0);

        // Calculate hash
        let mut hash = Sha256::new();
        hash.update(&concatened);
        let hash = hash.finalize();

        hash.as_slice() == self.0.as_slice()
    }
}

pub fn get_row<T: Clone>(row: usize, board: &[T]) -> Vec<T> {
    let start = (row - 1) * 9;
    board[start..start + 9].to_vec()
}

pub fn get_column<T: Clone>(column: usize, board: &[T]) -> Vec<T> {
    (0..9).map(|row| board[row * 9 + column - 1].clone()).collect()
}

pub fn get_subgrid<T: Clone>(subgrid_number: usize, sudoku: &[T]) -> Vec<T> {
    let start_row = (subgrid_number - 1) / 3 * 3;
    let start_col = (subgrid_number - 1) % 3 * 3;

    (0..3)
        .flat_map(|i| {
            (0..3)
                .map(move |j| sudoku[(start_row + i) * 9 + start_col + j].clone())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment() {
        let message = "My first test message".as_bytes();
        let (commitment, randomness) = Commitment::new(message);

        // verify commitment
        assert!(commitment.verify(message, &randomness));

        // should not verify - with wrong message
        let corrupted_message = "MY second test message".as_bytes();
        assert!(!commitment.verify(corrupted_message, &randomness));

        // should not verify - with wrong randomness
        let r = Randomness("test_randomness".as_bytes().to_vec());
        assert!(!commitment.verify(message, &r));
    }

    #[test]
    fn test_get_row() {
        let sudoku = Sudoku::generate().to_bytes();
        let get_row = get_row(3, &sudoku);
        let third_row: Vec<u8> = sudoku.to_vec()[18..27].to_vec();

        assert!(get_row == third_row);
    }

    #[test]
    fn test_get_column() {
        let sudoku = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            4, 5, 6, 7, 8, 9, 1, 2, 3,
            7, 8, 9, 1, 2, 3, 4, 5, 6, 
            2, 3, 4, 5, 6, 7, 8, 9, 1,
            5, 6, 7, 8, 9, 1, 2, 3, 4,
            8, 9, 1, 2, 3, 4, 5, 6, 7, 
            3, 4, 5, 6, 7, 8, 9, 1, 2,
            6, 7, 8, 9, 1, 2, 3, 4, 5,
            9, 1, 2, 3, 4, 5, 6, 7, 8,
        ];

        assert_eq!(get_column(2, &sudoku), vec![2, 5, 8, 3, 6, 9, 4, 7, 1]);
    }

    #[test]
    fn test_get_subgrid() {
        let sudoku = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9,
            4, 5, 6, 7, 8, 9, 1, 2, 3,
            7, 8, 9, 1, 2, 3, 4, 5, 6, 
            2, 3, 4, 5, 6, 7, 8, 9, 1,
            5, 6, 7, 8, 9, 1, 2, 3, 4,
            8, 9, 1, 2, 3, 4, 5, 6, 7, 
            3, 4, 5, 6, 7, 8, 9, 1, 2,
            6, 7, 8, 9, 1, 2, 3, 4, 5,
            9, 1, 2, 3, 4, 5, 6, 7, 8,
        ];

        assert_eq!(get_subgrid(5, &sudoku), vec![5, 6, 7, 8, 9, 1, 2, 3, 4]);
    }
}