use std::io;

#[derive(Debug)]
pub enum RunPuzzleError<PuzzleError> {
    NoInputFilePathGiven,
    ReadingInputFilePath(io::Error),
    RunningPuzzle(PuzzleError),
}

pub fn run<E, F: FnOnce(String) -> Result<String, E>>(f: F) -> Result<String, RunPuzzleError<E>> {
    let Some(file_path) = std::env::args().nth(1) else {
        return Err(RunPuzzleError::NoInputFilePathGiven);
    };
    let content =
        std::fs::read_to_string(file_path).map_err(RunPuzzleError::ReadingInputFilePath)?;
    f(content).map_err(RunPuzzleError::RunningPuzzle)
}
