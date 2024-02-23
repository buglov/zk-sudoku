use sudoku::Sudoku;

#[derive(Debug)]
pub enum ProofError {
    UnsolvableSudoku,
    InvalidChallenge,
    NonUniqueValues,
    CommitmentMismatch
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub sudoku: Sudoku
}

#[derive(Debug)]
pub struct Witness {
    pub solution: Sudoku
}

#[derive(Debug)]
pub struct Challenge(pub u8);