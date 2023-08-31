use grid::Grid;

enum Foo {
    Flagged,
    Revealed,
    Unrevealed
}

pub struct BoardCell {
    mine: bool,
    revealed: bool,
    flagged: bool, // Flagged and revealed are mutually exclusive
}

pub struct Board {
    cells: Grid<BoardCell>,
    state: BoardState,
}

pub enum BoardState {
    InProgress,
    Won,
    Lost,
}

// Actions a player can take: reveal
// Queries needed: Whether the board is won, lost or in-play

impl Board {
    pub fn from_configuration(configuration: Grid<BoardCell>) -> Self {
        Self {
            cells: configuration,
            state: BoardState::InProgress // This would have to be calculated from the board
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) -> Result<(), String> {
        match self.cells.get_mut(y, x) {
            Some(BoardCell { mine, revealed: true, flagged: _} ) => Err("Cell already revealed".to_string()),
            Some(BoardCell { mine, revealed, flagged: _} ) => {
                *revealed = true;
                return Ok(());
            },
            None => Err("Coordinates out of range".to_string()),
        }
    }

    pub fn state() -> BoardState {

        todo!()
    }
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_board
// }
