use log::*;
use serde::{Deserialize, Serialize};

/// An array of all the white chess pieces.
///
/// There is only one piece per type, so all pieces of a certain type is a reference to that.
pub static WHITE: [Piece; 6] = [
    Piece {
        color: Color::White,
        kind: Kind::Pawn,
    },
    Piece {
        color: Color::White,
        kind: Kind::Rook,
    },
    Piece {
        color: Color::White,
        kind: Kind::Knight,
    },
    Piece {
        color: Color::White,
        kind: Kind::Bishop,
    },
    Piece {
        color: Color::White,
        kind: Kind::Queen,
    },
    Piece {
        color: Color::White,
        kind: Kind::King,
    },
];

/// An array of all the black chess pieces.
///
/// There is only one piece per type, so all pieces of a certain type is a reference to that.
pub static BLACK: [Piece; 6] = [
    Piece {
        color: Color::Black,
        kind: Kind::Pawn,
    },
    Piece {
        color: Color::Black,
        kind: Kind::Rook,
    },
    Piece {
        color: Color::Black,
        kind: Kind::Knight,
    },
    Piece {
        color: Color::Black,
        kind: Kind::Bishop,
    },
    Piece {
        color: Color::Black,
        kind: Kind::Queen,
    },
    Piece {
        color: Color::Black,
        kind: Kind::King,
    },
];

/// The different kinds of chess pieces.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Kind {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Kind::King => write!(f, "king"),
            Kind::Queen => write!(f, "queen"),
            Kind::Knight => write!(f, "knight"),
            Kind::Bishop => write!(f, "bishop"),
            Kind::Rook => write!(f, "rook"),
            Kind::Pawn => write!(f, "pawn"),
        }
    }
}

/// The different colors of chess pieces.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Color::White => write!(f, "white"),
            Color::Black => write!(f, "black"),
        }
    }
}

/// The different types of victories.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum VictoryStatus {
    Checkmate,
    Stalemate,
    Draw,
    InProgress,
}

impl std::fmt::Display for VictoryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            VictoryStatus::Checkmate => write!(f, "checkmate"),
            VictoryStatus::Stalemate => write!(f, "stalemate"),
            VictoryStatus::Draw => write!(f, "draw"),
            VictoryStatus::InProgress => write!(f, "inprogress"),
        }
    }
}

/// The chess piece struct.
#[derive(PartialEq, Debug)]
pub struct Piece {
    /// The color of the chess piece.
    pub color: Color,
    /// The type of chess piece.
    pub kind: Kind,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.color, self.kind)
    }
}

/// The game struct.
///
/// The coordinates used to access pieces are 0-indexed tuples of (usize, usize),
/// and they follow the standard chess notation, so (0,0) corresponds to A1 in the bottom left corner,
/// and (7,7) corresponds to H8 in the top right corner, seen from the white side.
///
/// The pieces are stored as Option<&Piece>, and are references to the pieces in the WHITE and
/// BLACK array.
///
/// # Eksamples
///
/// ```
/// # use chess::*;
/// // Create a new game, with all pieces in their initial position.
/// let mut game = Game::new();
///
/// // The piece at C1 is supposed to be a bishop.
/// let bishop = game.get_from_pos((2,0));
/// if let Some(piece) = bishop {
///     assert_eq!(piece.kind, Kind::Bishop);
///     assert_eq!(piece.color, Color::White);
/// } else {
///     panic!("The piece at A1 should be a bishop.");
/// }
/// ```
#[allow(clippy::all)]
#[derive(Clone)]
pub struct Game<'a> {
    /// The current turn number.
    turn: u32,
    /// The game board. Contains references to the WHITE and BLACK arrays.
    board: [[Option<&'a Piece>; 8]; 8],
    ignore_kings: bool,
    ignore_check: bool,
    last: ((usize, usize), (usize, usize)),
    black_can_castle_right: bool,
    black_can_castle_left: bool,
    white_can_castle_right: bool,
    white_can_castle_left: bool,
    board_history: Vec<[[Option<&'a Piece>; 8]; 8]>,
    seventy_five_move_rule: u32,
    last_color: Color,
}
#[allow(clippy::all)]
impl<'a> Game<'a> {
    /// Creates a new game, with all the pieces in the correct starting position.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new();
    /// ```
    pub fn new() -> Game<'a> {
        let mut board: [[Option<&'a Piece>; 8]; 8] = [[None; 8]; 8];

        for i in 0..8 {
            board[i][1] = Some(&WHITE[0]);
            board[i][6] = Some(&BLACK[0]);
        }
        for i in 0..3 {
            board[i][0] = Some(&WHITE[1 + i]);
            board[7 - i][0] = Some(&WHITE[1 + i]);
            board[i][7] = Some(&BLACK[1 + i]);
            board[7 - i][7] = Some(&BLACK[1 + i]);
        }
        board[4][0] = Some(&WHITE[5]);
        board[3][0] = Some(&WHITE[4]);
        board[4][7] = Some(&BLACK[5]);
        board[3][7] = Some(&BLACK[4]);

        let mut game = Game {
            turn: 1,
            board,
            ignore_kings: false,
            ignore_check: false,
            last: ((0, 0), (0, 0)),
            white_can_castle_right: true,
            black_can_castle_right: true,
            white_can_castle_left: true,
            black_can_castle_left: true,
            board_history: Vec::new(),
            seventy_five_move_rule: 0,
            last_color: Color::Black,
        };
        game.save_board();

        game
    }

    /// Creates a new game with an empty board.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new_empty();
    /// assert_eq!(game.by_color(Color::White).len(), 0);
    /// assert_eq!(game.by_color(Color::Black).len(), 0);
    /// ```
    pub fn new_empty() -> Game<'a> {
        let mut game = Game {
            turn: 1,
            board: [[None; 8]; 8],
            ignore_kings: false,
            ignore_check: false,
            last: ((0, 0), (0, 0)),
            white_can_castle_right: true,
            black_can_castle_right: true,
            white_can_castle_left: true,
            black_can_castle_left: true,
            board_history: Vec::new(),
            seventy_five_move_rule: 0,
            last_color: Color::Black,
        };
        game.save_board();

        game
    }

    /// Clears the board.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new();
    /// assert_eq!(game.by_color(Color::White).len(), 16);
    ///
    /// game.clear();
    /// assert_eq!(game.by_color(Color::White).len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.board = [[None; 8]; 8];
        self.last = ((0, 0), (0, 0));
    }

    /// Tells the game whether to ignore a lack of kings.
    ///
    /// The game still sees if a possible move puts a king in check, but it no longer panics if one
    /// or both kings are missing. This can be useful when setting up special challenges.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new_empty();
    /// game.set_at_pos((3, 3), Some(&WHITE[3]));
    ///
    /// // This would cause a panic
    /// // game.valid_moves((3, 3));
    ///
    /// game.ignore_kings(true);
    /// game.valid_moves((3, 3));
    /// ```
    pub fn ignore_kings(&mut self, ignore: bool) {
        self.ignore_kings = ignore;
    }

    /// Tells the game whether to ignore check tests.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new_empty();
    /// // Put a black queen on D4 and a white king on C2
    /// game.set_at_pos((3, 3), Some(&BLACK[4]));
    /// game.set_at_pos((2, 1), Some(&WHITE[5]));
    ///
    /// // With check tests in place, the king can only move to B1, B3 and C1.
    /// assert_eq!(game.valid_moves((2, 1)).len(), 3);
    ///
    /// // Whithout check tests the king can also move to B2, C3, D1, D2 and D3.
    /// game.ignore_check(true);
    /// assert_eq!(game.valid_moves((2, 1)).len(), 8);
    /// ```
    pub fn ignore_check(&mut self, ignore: bool) {
        self.ignore_check = ignore;
    }

    /// Gets the piece at the given position on the board.
    ///
    /// Returns an Option where Some contains a reference to the piece,
    /// and None means there was no piece at the given position.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new();
    ///
    /// // Get the piece from A1
    /// match game.get_from_pos((0, 0)) {
    ///     Some(piece) => assert_eq!(piece.kind, Kind::Rook),
    ///     None => panic!("There should be a rook here."),
    /// }
    ///
    /// // Returns None if the position is empty.
    /// assert_eq!(game.get_from_pos((3, 4)), None);
    /// ```
    pub fn get_from_pos(&self, pos: (usize, usize)) -> Option<&'a Piece> {
        self.board[pos.0][pos.1]
    }

    /// Sets the piece at the given position on the board.
    ///
    /// The piece is passed as an Option, where the Some should contain a
    /// reference to the WHITE or BLACK arrays. Pass None to remove an existing piece.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new();
    ///
    /// // Replace all white pawns with queens.
    /// for i in 0..8 {
    ///     game.set_at_pos((i, 1), Some(&WHITE[4]));
    ///     match game.get_from_pos((i, 1)) {
    ///         Some(piece) => {
    ///             assert_eq!(piece.kind, Kind::Queen);
    ///             assert_eq!(piece.color, Color::White);
    ///         },
    ///         None => panic!("There should be a queen here."),
    ///     }
    /// }
    /// ```
    pub fn set_at_pos(&mut self, pos: (usize, usize), piece: Option<&'a Piece>) {
        if let Some(p) = piece {
            self.last_color = p.color;
        }
        self.board[pos.0][pos.1] = piece;
    }

    /// Returns the current turn.
    pub fn get_turn(&self) -> u32 {
        self.turn
    }

    /// Advances the game to the next turn.
    pub fn next_turn(&mut self) {
        self.turn += 1;
    }

    /// Returns a vector of all pieces of a given color, and their position on the board.
    ///
    /// The pieces are arrenged in the order they are found, starting at A1 through H1, then A2
    /// through H2, until it reaches H8.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let game = Game::new();
    ///
    /// // At the start of a chess game there should be 16 pieces of each color.
    /// assert_eq!(game.by_color(Color::White).len(), 16);
    /// assert_eq!(game.by_color(Color::Black).len(), 16);
    ///
    /// // The 9th white piece should be the pawn at A2.
    /// let pieces = game.by_color(Color::White);
    /// assert_eq!(pieces[8].0, (0, 1));
    /// assert_eq!(pieces[8].1.kind, Kind::Pawn);
    /// assert_eq!(pieces[8].1.color, Color::White);
    /// ```
    pub fn by_color(&self, color: Color) -> Vec<((usize, usize), &'a Piece)> {
        let mut pieces: Vec<((usize, usize), &'a Piece)> = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.board[x][y] {
                    if piece.color == color {
                        pieces.push(((x, y), piece));
                    }
                }
            }
        }
        pieces
    }

    /// Returns a vector of all pieces of a given kind, and their position on the board.
    ///
    /// The pieces are arrenged in the order they are found, starting at A1 through H1, then A2
    /// through H2, until it reaches H8.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let game = Game::new();
    ///
    /// // At the start of a chess game there should be 16 pawns and 2 kings.
    /// assert_eq!(game.by_kind(Kind::Pawn).len(), 16);
    /// assert_eq!(game.by_kind(Kind::King).len(), 2);
    ///
    /// // The 13th pawn should be the black pawn at E7..
    /// let pawns = game.by_kind(Kind::Pawn);
    /// assert_eq!(pawns[12].0, (4, 6));
    /// assert_eq!(pawns[12].1.kind, Kind::Pawn);
    /// assert_eq!(pawns[12].1.color, Color::Black);
    /// ```
    pub fn by_kind(&self, kind: Kind) -> Vec<((usize, usize), &'a Piece)> {
        let mut pieces: Vec<((usize, usize), &'a Piece)> = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.board[x][y] {
                    if piece.kind == kind {
                        pieces.push(((x, y), piece));
                    }
                }
            }
        }
        pieces
    }

    /// Returns a vector of all pieces of a given kind and color, and their position on the board.
    ///
    /// The pieces are arrenged in the order they are found, starting at A1 through H1, then A2
    /// through H2, until it reaches H8.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let game = Game::new();
    ///
    /// // At the start of a chess game there should be 8 black pawns and 2 white knights.
    /// assert_eq!(game.by_kind_and_color(Kind::Pawn, Color::Black).len(), 8);
    /// assert_eq!(game.by_kind_and_color(Kind::Knight, Color::White).len(), 2);
    ///
    /// // The 2nd black bishop should be at F8.
    /// let bishops = game.by_kind_and_color(Kind::Bishop, Color::Black);
    /// assert_eq!(bishops[1].0, (5, 7));
    /// assert_eq!(bishops[1].1.kind, Kind::Bishop);
    /// assert_eq!(bishops[1].1.color, Color::Black);
    /// ```
    pub fn by_kind_and_color(&self, kind: Kind, color: Color) -> Vec<((usize, usize), &'a Piece)> {
        let mut pieces: Vec<((usize, usize), &'a Piece)> = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = self.board[x][y] {
                    if piece.kind == kind && piece.color == color {
                        pieces.push(((x, y), piece));
                    }
                }
            }
        }
        pieces
    }

    /// Moves a piece from one position to another.
    ///
    /// The return value is an Option containing a reference to the captured piece (if any), or
    /// None if either of the positions given were empty. Trying to move from a position that
    /// doesn't contain a piece therefore returns None.
    ///
    /// This function doesn't check whether the move is valid, only that the positions are within
    /// bounds. Therefore this should always be used together with valid_moves when playing proper
    /// chess.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new();
    ///
    /// // Move a pawn from D2 to D3.
    /// // This returns None because no pieces were captured.
    /// assert_eq!(game.move_piece((3, 1), (3, 2)), None);
    ///
    /// // The original position is now empty.
    /// assert_eq!(game.get_from_pos((3, 1)), None);
    /// // And the new position contains the pawn.
    /// match game.get_from_pos((3, 2)) {
    ///     Some(piece) => {
    ///         assert_eq!(piece.kind, Kind::Pawn);
    ///         assert_eq!(piece.color, Color::White);
    ///     },
    ///     None => panic!("There should be a pawn here."),
    /// }
    ///
    /// // Moving a pawn from D3 to H8 is illegal in chess, but can be done here.
    /// // The captured rook is removed from the board, and returned.
    /// let captured = game.move_piece((3, 2), (7, 7));
    /// match captured {
    ///     Some(piece) => {
    ///         assert_eq!(piece.kind, Kind::Rook);
    ///         assert_eq!(piece.color, Color::Black);
    ///     },
    ///     None => panic!("There should be a captured piece here."),
    /// }
    ///
    /// // There is no piece at B4, so trying to move from there is just returning None.
    /// assert_eq!(game.move_piece((1, 3), (4, 0)), None);
    /// ```
    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Option<&'a Piece> {
        if from.0 > 7 || from.1 > 7 || to.0 > 7 || to.1 > 7 {
            return None;
        }
        let mut moving = self.get_from_pos(from);
        let other = self.get_from_pos(to);
        match moving {
            Some(p) => {
                if let Some(_) = other {
                    self.seventy_five_move_rule = 0;
                } else {
                    self.seventy_five_move_rule += 1;
                }

                if p.kind == Kind::Pawn {
                    self.seventy_five_move_rule = 0;
                    if p.color == Color::White && to.1 == 7 {
                        moving = Some(&WHITE[4]);
                    } else if p.color == Color::Black && to.1 == 0 {
                        moving = Some(&BLACK[4]);
                    }
                } else if p.kind == Kind::King {
                    match p.color {
                        Color::White => {
                            self.white_can_castle_left = false;
                            self.white_can_castle_right = false;
                        }
                        Color::Black => {
                            self.black_can_castle_left = false;
                            self.black_can_castle_right = false;
                        }
                    }
                } else if p.kind == Kind::Rook {
                    match p.color {
                        Color::White => {
                            if from.0 == 0 {
                                self.white_can_castle_left = false;
                            } else if from.0 == 7 {
                                self.white_can_castle_right = false;
                            }
                        }
                        Color::Black => {
                            if from.0 == 0 {
                                self.black_can_castle_left = false;
                            } else if from.0 == 7 {
                                self.black_can_castle_right = false;
                            }
                        }
                    }
                }

                self.set_at_pos(to, moving);
                self.set_at_pos(from, None);
                self.last = (from, to);
                other
            }
            None => None,
        }
    }

    /// Executes several moves, as stated in the given array.
    ///
    /// The return value is Some containing the last captured piece (if any), or None if no pieces
    /// were captured or no pieces were moved. If one of the moves is out of bounds no moves are
    /// executed, and None is returned.
    ///
    /// In cases where only one piece must be moved manually, move_piece is preferred.
    ///
    /// This function is supposed to be called with the result of valid_moves. It is used instead
    /// of move_piece in case complex moves where several pieces is moved, like castling, is
    /// nessessary. This function doesn't check whether the moves are legal.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new();
    /// let mut moves: Vec<((usize, usize), (usize, usize))>;
    ///
    /// // Move a pawn from E2 forwards twice.
    /// moves = vec![((4, 1), (4, 2)), ((4, 2), (4, 3))];
    /// assert_eq!(game.move_pieces(&moves), None);
    /// match game.get_from_pos((4, 3)) {
    ///     Some(piece) => {
    ///         assert_eq!(piece.kind, Kind::Pawn);
    ///         assert_eq!(piece.color, Color::White);
    ///     },
    ///     None => panic!("There should be a pawn here."),
    /// }
    ///
    /// // When two pieces are captured only the last one is returned.
    /// // Moves the pawn from E4, captures the queen at D8, then captures the rook at H8.
    /// moves = vec![((4, 3), (3, 7)), ((3, 7), (7, 7))];
    /// let captured = game.move_pieces(&moves);
    /// match captured {
    ///     Some(piece) => {
    ///         assert_eq!(piece.kind, Kind::Rook);
    ///         assert_eq!(piece.color, Color::Black);
    ///     },
    ///     None => panic!("There should be a rook here."),
    /// }
    /// ```
    #[allow(clippy::all)]
    pub fn move_pieces(&mut self, moves: &[((usize, usize), (usize, usize))]) -> Option<&'a Piece> {
        let mut to: (usize, usize);
        let mut from: (usize, usize);
        let mut captured: Option<&'a Piece> = None;
        let mut tmp: Option<&'a Piece>;

        for v in moves {
            from = v.0;
            to = v.1;
            if from.0 > 7 || from.1 > 7 || to.0 > 7 || to.1 > 7 {
                return None;
            }
        }

        for v in moves {
            from = v.0;
            to = v.1;
            tmp = self.move_piece(from, to);
            if let Some(_) = tmp {
                captured = tmp;
                self.board_history.clear();
            }
            self.save_board();
        }

        captured
    }

    /// Returns a vector of all the moves the piece at the given position can make.
    ///
    /// The returned vector contains vectors of moves, as a tuple of the current location and the
    /// destination. This is done so that more complex moves that involve moving several pieces,
    /// such as castling, can be carried out. Each of these vectors can be passed to move_pieces to
    /// be executed.
    ///
    /// If the given position doesn't contain a piece, a vector with size 0 is returned.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new();
    ///
    /// // The pawn at E2 can only move forwards one or two squares.
    /// let moves = game.valid_moves((4, 1));
    /// // The returned vector contains two possible moves, each requiering only one move
    /// // to be carried out.
    /// assert_eq!(moves.len(), 2);
    /// assert_eq!(moves[0].len(), 1);
    /// assert_eq!(moves[1].len(), 1);
    ///
    /// // The pawn can be moved to squares forwards.
    /// assert_eq!(moves[0][0].0, (4, 1));
    /// assert_eq!(moves[0][0].1, (4, 3));
    ///
    /// // Or one step forwards.
    /// assert_eq!(moves[1][0].0, (4, 1));
    /// assert_eq!(moves[1][0].1, (4, 2));
    ///
    /// // Lets move it two steps forwards, to E4.
    /// game.move_pieces(&moves[0]);
    /// assert!(game.get_from_pos((4, 3)) != None);
    /// // Advance the turn. This is nessessary for some internal handling.
    /// game.next_turn();
    ///
    /// // Now we move a black pawn from D7 to D5.
    /// for v in game.valid_moves((3, 6)) {
    ///     if v.len() == 1 && v[0].1 == (3, 4) {
    ///         game.move_pieces(&v);
    ///         assert!(game.get_from_pos((3, 4)) != None);
    ///         game.next_turn();
    ///         break;
    ///     }
    /// }
    ///
    /// // Now the white pawn can capture the black pawn at D5.
    /// for v in game.valid_moves((4, 3)) {
    ///     if v.len() == 1 && v[0].1 == (3, 4) {
    ///         assert!(game.move_pieces(&v) != None);
    ///         match game.get_from_pos((3, 4)) {
    ///             Some(piece) => {
    ///                 assert_eq!(piece.kind, Kind::Pawn);
    ///                 assert_eq!(piece.color, Color::White);
    ///             },
    ///             None => panic!("There should be a piece here."),
    ///         }
    ///         game.next_turn();
    ///         break;
    ///     }
    /// }
    /// ```
    #[allow(clippy::all)]
    pub fn valid_moves(&self, pos: (usize, usize)) -> Vec<Vec<((usize, usize), (usize, usize))>> {
        self.check_valid_moves(pos, true)
    }

    fn check_valid_moves(
        &self,
        pos: (usize, usize),
        test_check: bool,
    ) -> Vec<Vec<((usize, usize), (usize, usize))>> {
        info!(
            "check_valid_moves called with args: pos: ({}, {}), test_check: {}",
            pos.0, pos.1, test_check
        );
        let mut result: Vec<Vec<((usize, usize), (usize, usize))>> = self.raw_moves(pos);

        let mut index: Vec<usize> = Vec::new();
        let mut from: (usize, usize);
        let mut to: (usize, usize);
        let mut game: Game;
        'outer: for i in 0..result.len() {
            game = self.clone();
            for j in 0..result[i].len() {
                from = result[i][j].0;
                to = result[i][j].1;
                if from.0 > 7 || from.1 > 7 || to.0 > 7 || to.1 > 7 {
                    info!(
                        "from: ({}, {}) to: ({}, {}) excluded, being out of bounds",
                        from.0, from.1, to.0, to.1
                    );
                    index.insert(0, i);
                    continue 'outer;
                } else if let Some(piece) = game.get_from_pos(from) {
                    if let Some(other) = game.get_from_pos(to) {
                        if other.color == piece.color {
                            info!("from: ({}, {}) to: ({}, {}) excluded because it was targeting a friendly", from.0, from.1, to.0, to.1);
                            index.insert(0, i);
                            continue 'outer;
                        }
                    }
                    if test_check && game.check_for_check(from, to) {
                        info!("from: ({}, {}) to: ({}, {}) at index {} excluded because it would put it in check", from.0, from.1, to.0, to.1, i);
                        index.insert(0, i);
                        continue 'outer;
                    }
                } else {
                    panic!("No piece at ({}, {})", from.0, from.1);
                }
                game.move_piece(from, to);
            }
        }
        for v in index {
            result.remove(v);
        }

        info!("check_valid_moves finished");
        result
    }

    fn raw_moves(&self, pos: (usize, usize)) -> Vec<Vec<((usize, usize), (usize, usize))>> {
        let mut result: Vec<Vec<((usize, usize), (usize, usize))>> = Vec::new();
        let mut moves: Vec<(usize, usize)> = Vec::new();

        match self.get_from_pos(pos) {
            None => {}
            Some(piece) => {
                let mut passant: bool;
                match piece.kind {
                    Kind::Pawn => {
                        match piece.color {
                            Color::White => {
                                if pos.1 == 1 {
                                    if let None = self.get_from_pos((pos.0, pos.1 + 1)) {
                                        if let None = self.get_from_pos((pos.0, pos.1 + 2)) {
                                            moves.push((pos.0, pos.1 + 2));
                                        }
                                    }
                                }

                                if pos.1 < 7 {
                                    if let None = self.get_from_pos((pos.0, pos.1 + 1)) {
                                        moves.push((pos.0, pos.1 + 1));
                                    }
                                }

                                if pos.0 > 0 && pos.1 < 7 {
                                    passant = false;
                                    if let Some(other) = self.get_from_pos((pos.0 - 1, pos.1)) {
                                        if other.color != piece.color
                                            && pos.1 == 4
                                            && (self.last.0).0 == pos.0 - 1
                                            && (self.last.0).1 == pos.1 + 2
                                            && (self.last.1).0 == pos.0 - 1
                                            && (self.last.1).1 == pos.1
                                        {
                                            passant = true;
                                            result.push(vec![
                                                ((pos.0, pos.1), (pos.0 - 1, pos.1)),
                                                ((pos.0 - 1, pos.1), (pos.0 - 1, pos.1 + 1)),
                                            ]);
                                        }
                                    }
                                    if let Some(_) = self.get_from_pos((pos.0 - 1, pos.1 + 1)) {
                                        if !passant {
                                            moves.push((pos.0 - 1, pos.1 + 1));
                                        }
                                    }
                                }
                                if pos.0 < 7 && pos.1 < 7 {
                                    passant = false;
                                    if let Some(other) = self.get_from_pos((pos.0 + 1, pos.1)) {
                                        if other.color != piece.color
                                            && pos.1 == 4
                                            && (self.last.0).0 == pos.0 + 1
                                            && (self.last.0).1 == pos.1 + 2
                                            && (self.last.1).0 == pos.0 + 1
                                            && (self.last.1).1 == pos.1
                                        {
                                            passant = true;
                                            result.push(vec![
                                                ((pos.0, pos.1), (pos.0 + 1, pos.1)),
                                                ((pos.0 + 1, pos.1), (pos.0 + 1, pos.1 + 1)),
                                            ]);
                                        }
                                    }
                                    if let Some(_) = self.get_from_pos((pos.0 + 1, pos.1 + 1)) {
                                        if !passant {
                                            moves.push((pos.0 + 1, pos.1 + 1));
                                        }
                                    }
                                }
                            }
                            Color::Black => {
                                if pos.1 == 6 {
                                    if let None = self.get_from_pos((pos.0, pos.1 - 1)) {
                                        if let None = self.get_from_pos((pos.0, pos.1 - 2)) {
                                            moves.push((pos.0, pos.1 - 2));
                                        }
                                    }
                                }

                                if pos.1 > 0 {
                                    if let None = self.get_from_pos((pos.0, pos.1 - 1)) {
                                        moves.push((pos.0, pos.1 - 1));
                                    }
                                }

                                if pos.0 > 0 && pos.1 > 0 {
                                    passant = false;
                                    if let Some(other) = self.get_from_pos((pos.0 - 1, pos.1)) {
                                        if other.color != piece.color
                                            && pos.1 == 3
                                            && (self.last.0).0 == pos.0 - 1
                                            && (self.last.0).1 == pos.1 - 2
                                            && (self.last.1).0 == pos.0 - 1
                                            && (self.last.1).1 == pos.1
                                        {
                                            passant = true;
                                            result.push(vec![
                                                ((pos.0, pos.1), (pos.0 - 1, pos.1)),
                                                ((pos.0 - 1, pos.1), (pos.0 - 1, pos.1 - 1)),
                                            ]);
                                        }
                                    }
                                    if let Some(_) = self.get_from_pos((pos.0 - 1, pos.1 - 1)) {
                                        if !passant {
                                            moves.push((pos.0 - 1, pos.1 - 1));
                                        }
                                    }
                                }
                                if pos.0 < 7 && pos.1 > 0 {
                                    passant = false;
                                    if let Some(other) = self.get_from_pos((pos.0 + 1, pos.1)) {
                                        if other.color != piece.color
                                            && pos.1 == 3
                                            && (self.last.0).0 == pos.0 + 1
                                            && (self.last.0).1 == pos.1 - 2
                                            && (self.last.1).0 == pos.0 + 1
                                            && (self.last.1).1 == pos.1
                                        {
                                            passant = true;
                                            result.push(vec![
                                                ((pos.0, pos.1), (pos.0 + 1, pos.1)),
                                                ((pos.0 + 1, pos.1), (pos.0 + 1, pos.1 - 1)),
                                            ]);
                                        }
                                    }
                                    if let Some(_) = self.get_from_pos((pos.0 + 1, pos.1 - 1)) {
                                        if !passant {
                                            moves.push((pos.0 + 1, pos.1 - 1));
                                        }
                                    }
                                }
                            }
                        };
                    }
                    Kind::Rook => {
                        let mut x: usize = pos.0;
                        let mut y: usize = pos.1;
                        // Vertically/horisontally
                        while x < 7 {
                            x += 1;
                            moves.push((x, pos.1));
                            if let Some(_) = self.get_from_pos((x, pos.1)) {
                                break;
                            }
                        }
                        x = pos.0;
                        while x > 0 {
                            x -= 1;
                            moves.push((x, pos.1));
                            if let Some(_) = self.get_from_pos((x, pos.1)) {
                                break;
                            }
                        }

                        while y < 7 {
                            y += 1;
                            moves.push((pos.0, y));
                            if let Some(_) = self.get_from_pos((pos.0, y)) {
                                break;
                            }
                        }
                        y = pos.1;
                        while y > 0 {
                            y -= 1;
                            moves.push((pos.0, y));
                            if let Some(_) = self.get_from_pos((pos.0, y)) {
                                break;
                            }
                        }
                    }
                    Kind::Bishop => {
                        let mut x: usize = pos.0;
                        let mut y: usize = pos.1;
                        // Diagonally
                        while x < 7 && y < 7 {
                            x += 1;
                            y += 1;
                            moves.push((x, y));
                            if let Some(_) = self.get_from_pos((x, y)) {
                                break;
                            }
                        }

                        x = pos.0;
                        y = pos.1;
                        while x < 7 && y > 0 {
                            x += 1;
                            y -= 1;
                            moves.push((x, y));
                            if let Some(_) = self.get_from_pos((x, y)) {
                                break;
                            }
                        }

                        x = pos.0;
                        y = pos.1;
                        while x > 0 && y < 7 {
                            x -= 1;
                            y += 1;
                            moves.push((x, y));
                            if let Some(_) = self.get_from_pos((x, y)) {
                                break;
                            }
                        }

                        x = pos.0;
                        y = pos.1;
                        while x > 0 && y > 0 {
                            x -= 1;
                            y -= 1;
                            moves.push((x, y));
                            if let Some(_) = self.get_from_pos((x, y)) {
                                break;
                            }
                        }
                    }
                    Kind::Queen => {
                        let mut x: usize = pos.0;
                        let mut y: usize = pos.1;
                        // Diagonally
                        while x < 7 && y < 7 {
                            x += 1;
                            y += 1;
                            moves.push((x, y));
                            if let Some(_) = self.get_from_pos((x, y)) {
                                break;
                            }
                        }

                        x = pos.0;
                        y = pos.1;
                        while x < 7 && y > 0 {
                            x += 1;
                            y -= 1;
                            moves.push((x, y));
                            if let Some(_) = self.get_from_pos((x, y)) {
                                break;
                            }
                        }

                        x = pos.0;
                        y = pos.1;
                        while x > 0 && y < 7 {
                            x -= 1;
                            y += 1;
                            moves.push((x, y));
                            if let Some(_) = self.get_from_pos((x, y)) {
                                break;
                            }
                        }

                        x = pos.0;
                        y = pos.1;
                        while x > 0 && y > 0 {
                            x -= 1;
                            y -= 1;
                            moves.push((x, y));
                            if let Some(_) = self.get_from_pos((x, y)) {
                                break;
                            }
                        }

                        // Vertically/horisontally
                        x = pos.0;
                        while x < 7 {
                            x += 1;
                            moves.push((x, pos.1));
                            if let Some(_) = self.get_from_pos((x, pos.1)) {
                                break;
                            }
                        }
                        x = pos.0;
                        while x > 0 {
                            x -= 1;
                            moves.push((x, pos.1));
                            if let Some(_) = self.get_from_pos((x, pos.1)) {
                                break;
                            }
                        }

                        y = pos.1;
                        while y < 7 {
                            y += 1;
                            moves.push((pos.0, y));
                            if let Some(_) = self.get_from_pos((pos.0, y)) {
                                break;
                            }
                        }
                        y = pos.1;
                        while y > 0 {
                            y -= 1;
                            moves.push((pos.0, y));
                            if let Some(_) = self.get_from_pos((pos.0, y)) {
                                break;
                            }
                        }
                    }
                    Kind::Knight => {
                        if pos.0 >= 1 {
                            if pos.1 >= 2 {
                                moves.push((pos.0 - 1, pos.1 - 2));
                            }
                            if pos.1 <= 5 {
                                moves.push((pos.0 - 1, pos.1 + 2));
                            }
                        }
                        if pos.0 <= 6 {
                            if pos.1 >= 2 {
                                moves.push((pos.0 + 1, pos.1 - 2));
                            }
                            if pos.1 <= 5 {
                                moves.push((pos.0 + 1, pos.1 + 2));
                            }
                        }
                        if pos.0 >= 2 {
                            if pos.1 >= 1 {
                                moves.push((pos.0 - 2, pos.1 - 1));
                            }
                            if pos.1 <= 6 {
                                moves.push((pos.0 - 2, pos.1 + 1));
                            }
                        }
                        if pos.0 <= 5 {
                            if pos.1 >= 1 {
                                moves.push((pos.0 + 2, pos.1 - 1));
                            }
                            if pos.1 <= 6 {
                                moves.push((pos.0 + 2, pos.1 + 1));
                            }
                        }
                    }
                    Kind::King => {
                        if pos.0 > 0 {
                            moves.push((pos.0 - 1, pos.1));
                            if pos.1 > 0 {
                                moves.push((pos.0 - 1, pos.1 - 1));
                            }
                            if pos.1 < 7 {
                                moves.push((pos.0 - 1, pos.1 + 1));
                            }
                        }
                        if pos.0 < 7 {
                            moves.push((pos.0 + 1, pos.1));
                            if pos.1 > 0 {
                                moves.push((pos.0 + 1, pos.1 - 1));
                            }
                            if pos.1 < 7 {
                                moves.push((pos.0 + 1, pos.1 + 1));
                            }
                        }

                        if pos.1 > 0 {
                            moves.push((pos.0, pos.1 - 1));
                        }
                        if pos.1 < 7 {
                            moves.push((pos.0, pos.1 + 1));
                        }

                        let mut left: Vec<((usize, usize), (usize, usize))> = Vec::new();
                        let mut right: Vec<((usize, usize), (usize, usize))> = Vec::new();
                        let mut game: Game;
                        let mut p: (usize, usize);
                        match piece.color {
                            Color::White => {
                                if pos.0 == 4 && pos.1 == 0 {
                                    if self.white_can_castle_left {
                                        game = self.clone();
                                        for i in 1..4 {
                                            if i == 3 {
                                                if let None = game.get_from_pos((1, pos.1)) {
                                                    if let Some(rook) =
                                                        game.get_from_pos((0, pos.1))
                                                    {
                                                        if rook.color == piece.color
                                                            && rook.kind == Kind::Rook
                                                        {
                                                            left.push(((0, pos.1), (3, pos.1)));
                                                            result.push(left);
                                                        }
                                                    }
                                                }
                                                break;
                                            }
                                            p = (pos.0 - i, pos.1);

                                            if let Some(_) = game.move_piece(pos, p) {
                                                break;
                                            }

                                            if game.in_check(piece.color) {
                                                break;
                                            }

                                            left.push(((p.0 + 1, p.1), p));
                                        }
                                    }
                                    if self.white_can_castle_right {
                                        game = self.clone();
                                        for i in 1..4 {
                                            if i == 3 {
                                                if let None = game.get_from_pos((6, pos.1)) {
                                                    if let Some(rook) =
                                                        game.get_from_pos((7, pos.1))
                                                    {
                                                        if rook.color == piece.color
                                                            && rook.kind == Kind::Rook
                                                        {
                                                            right.push(((7, pos.1), (5, pos.1)));
                                                            result.push(right);
                                                        }
                                                    }
                                                }
                                                break;
                                            }
                                            p = (pos.0 + i, pos.1);

                                            if let Some(_) = game.move_piece(pos, p) {
                                                break;
                                            }

                                            if game.in_check(piece.color) {
                                                break;
                                            }

                                            right.push(((p.0 - 1, p.1), p));
                                        }
                                    }
                                }
                            }
                            Color::Black => {
                                if pos.0 == 4 && pos.1 == 7 {
                                    if self.black_can_castle_left {
                                        game = self.clone();
                                        for i in 1..4 {
                                            if i == 3 {
                                                if let None = game.get_from_pos((1, pos.1)) {
                                                    if let Some(rook) =
                                                        game.get_from_pos((0, pos.1))
                                                    {
                                                        if rook.color == piece.color
                                                            && rook.kind == Kind::Rook
                                                        {
                                                            left.push(((0, pos.1), (3, pos.1)));
                                                            result.push(left);
                                                        }
                                                    }
                                                }
                                                break;
                                            }
                                            p = (pos.0 - i, pos.1);

                                            if let Some(_) = game.move_piece(pos, p) {
                                                break;
                                            }

                                            if game.in_check(piece.color) {
                                                break;
                                            }

                                            left.push(((p.0 + 1, p.1), p));
                                        }
                                    }
                                    if self.black_can_castle_right {
                                        game = self.clone();
                                        for i in 1..4 {
                                            if i == 3 {
                                                if let None = game.get_from_pos((6, pos.1)) {
                                                    if let Some(rook) =
                                                        game.get_from_pos((7, pos.1))
                                                    {
                                                        if rook.color == piece.color
                                                            && rook.kind == Kind::Rook
                                                        {
                                                            right.push(((7, pos.1), (5, pos.1)));
                                                            result.push(right);
                                                        }
                                                    }
                                                }
                                                break;
                                            }
                                            p = (pos.0 + i, pos.1);

                                            if let Some(_) = game.move_piece(pos, p) {
                                                break;
                                            }

                                            if game.in_check(piece.color) {
                                                break;
                                            }

                                            right.push(((p.0 - 1, p.1), p));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for v in moves {
            result.push(vec![(pos, v)]);
        }

        result
    }

    /// Sees whether the king of the given color is currently in check or not.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// // Clear the board, then put a black king at C5, and a white pawn at D4.
    /// let mut game = Game::new_empty();
    /// game.set_at_pos((3, 3), Some(&WHITE[0]));
    /// game.set_at_pos((2, 4), Some(&BLACK[5]));
    ///
    /// assert!(game.in_check(Color::Black));
    /// ```
    pub fn in_check(&self, color: Color) -> bool {
        info!("in_check called with args: color: {}", color);
        if self.ignore_check {
            return false;
        }
        let other = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        let list = self.by_kind_and_color(Kind::King, color);
        if list.len() == 0 {
            if self.ignore_kings {
                return false;
            } else {
                panic!("There is no king");
            }
        }
        let king = list[0];

        for piece in self.by_color(other) {
            for moves in self.check_valid_moves(piece.0, false) {
                for v in moves {
                    if v.1 == king.0 {
                        info!("In check");
                        return true;
                    }
                }
            }
        }
        info!("Not in check");
        false
    }
    #[allow(clippy::all)]
    fn check_for_check(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        info!(
            "check_for_check called with args: from ({}, {}) to: ({}, {})",
            from.0, from.1, to.0, to.1
        );
        let mut game = self.clone();
        let color: Color;
        match game.get_from_pos(from) {
            Some(piece) => color = piece.color,
            None => panic!("No piece found at position ({}, {}).", from.0, from.1),
        }
        game.move_piece(from, to);
        game.in_check(color)
    }

    /// Checks whether the game is won, and returns the victory type and the color of the victor,
    /// or None if the game isn't won yet. In case of a draw a random color is returned.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// // Clear the board, then put a black king at A1, and a white queen at B2. We also need a
    /// // white king on the board, if not the program panics.
    /// let mut game = Game::new_empty();
    /// game.set_at_pos((1, 1), Some(&WHITE[4]));
    /// game.set_at_pos((0, 0), Some(&BLACK[5]));
    /// game.set_at_pos((6, 7), Some(&WHITE[5]));
    ///
    /// // The king is in check, but it can still move and take out the queen.
    /// assert_eq!(game.check_victory(), None);
    ///
    /// // Move the queen to B3
    /// game.set_at_pos((1, 2), Some(&WHITE[4]));
    /// game.set_at_pos((1, 1), None);
    ///
    /// // Now the king isn't in check, but the king can't move so it's a stalemate draw.
    ///
    /// // Add another queen at C3
    /// game.set_at_pos((2, 2), Some(&WHITE[4]));
    ///
    /// // Now the king is in check, and can't move, so white has won by checkmate.
    /// ```
    #[allow(clippy::all)]
    pub fn check_victory(&self) -> Option<(VictoryStatus, Color)> {
        if self.seventy_five_move_rule >= 75 {
            return Some((VictoryStatus::Draw, Color::White));
        }
        if self.board_history.len() >= 5 {
            info!("Checking for five fold repetition");
            let mut matches = 0;
            let last = match self.board_history.last() {
                Some(v) => v,
                None => panic!(),
            };
            'rep: for v in &self.board_history {
                for x in 0..8 {
                    for y in 0..8 {
                        if v[x][y] != last[x][y] {
                            continue 'rep;
                        }
                    }
                }
                matches += 1;
            }

            if matches >= 5 {
                return Some((VictoryStatus::Draw, Color::White));
            }
        }

        'outer: for color in vec![Color::Black, Color::White] {
            let pieces = self.by_color(color);

            for (pos, _) in pieces {
                if self.valid_moves(pos).len() > 0 {
                    continue 'outer;
                }
            }

            let opposite: Color = if color == Color::White {
                Color::Black
            } else {
                Color::White
            };

            if self.in_check(color) {
                return Some((VictoryStatus::Checkmate, opposite));
            } else if self.last_color != color {
                return Some((VictoryStatus::Stalemate, opposite));
            }
        }

        None
    }

    /// Turns a move, as returned from `valid_moves`, into [algebraic
    /// notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)) (AN).
    ///
    /// If `result` is `true` the function will detect whether a checkmate or a stalemate has
    /// occured, and add "1-0", "0-1" or "½-½".
    ///
    /// If `unicode` is `true` the pieces are represented by unicode symbols instead of letters.
    /// Only black pieces are used, as they are easier to see.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new();
    ///
    /// // Turn a string from AN into a move, and back into AN.
    /// let mut m = game.an_to_move("Nc3", Color::White).unwrap();
    /// assert_eq!(game.move_to_an(&m, true, false), "Nc3");
    /// assert_eq!(game.move_to_an(&m, true, true), "♞c3");
    ///
    /// m = game.an_to_move("e5", Color::Black).unwrap();
    /// assert_eq!(game.move_to_an(&m, true, false), "e5");
    /// ```
    #[allow(clippy::all)]
    pub fn move_to_an(
        &self,
        m: &[((usize, usize), (usize, usize))],
        result: bool,
        unicode: bool,
    ) -> String {
        let mut s = String::new();
        let piece = match self.get_from_pos(m[0].0) {
            Some(p) => p,
            None => panic!("No piece at position ({}, {}).", (m[0].0).0, (m[0].0).1),
        };
        let dest = m.last().unwrap().1;
        let mut capture: Option<&Piece> = None;
        for v in m {
            if let Some(p) = self.get_from_pos(v.1) {
                if piece.color != p.color {
                    capture = Some(p);
                }
            }
        }

        if m.len() == 3 {
            if (m[0].1).0 == 3 {
                s.push_str("0-0-0");
            } else if (m[0].1).0 == 5 {
                s.push_str("0-0");
            } else {
                panic!("Invalid castling move.");
            }
        } else {
            if piece.kind == Kind::Pawn {
                if let Some(_) = capture {
                    s.push(match (m[0].0).0 {
                        0 => 'a',
                        1 => 'b',
                        2 => 'c',
                        3 => 'd',
                        4 => 'e',
                        5 => 'f',
                        6 => 'g',
                        7 => 'h',
                        _ => panic!(),
                    });
                }
            } else if unicode {
                s.push(match piece.kind {
                    Kind::Rook => '\u{265c}',
                    Kind::Knight => '\u{265e}',
                    Kind::Bishop => '\u{265d}',
                    Kind::Queen => '\u{265b}',
                    Kind::King => '\u{265a}',
                    _ => panic!(),
                });
            } else {
                match piece.kind {
                    Kind::Rook => s.push('R'),
                    Kind::Knight => s.push('N'),
                    Kind::Bishop => s.push('B'),
                    Kind::Queen => s.push('Q'),
                    Kind::King => s.push('K'),
                    _ => panic!(),
                }
            }

            let mut row = false;
            let mut col = false;
            for i in self.by_kind_and_color(piece.kind, piece.color) {
                let (pos, _) = i;
                if pos.0 != (m[0].0).0 && pos.1 != (m[0].0).1 {
                    for v in self.valid_moves(pos) {
                        let (tmp_x, tmp_y) = v.last().unwrap().1;
                        if tmp_x == dest.0 && tmp_y == dest.1 {
                            if pos.0 == (m[0].0).0 {
                                row = true;
                            } else {
                                col = true;
                            }
                        }
                    }
                }
            }

            if col {
                s.push(match (m[0].0).0 {
                    0 => 'a',
                    1 => 'b',
                    2 => 'c',
                    3 => 'd',
                    4 => 'e',
                    5 => 'f',
                    6 => 'g',
                    7 => 'h',
                    _ => panic!(),
                });
            }
            if row {
                s.push(match (m[0].0).1 {
                    0 => '1',
                    1 => '2',
                    2 => '3',
                    3 => '4',
                    4 => '5',
                    5 => '6',
                    6 => '7',
                    7 => '8',
                    _ => panic!(),
                });
            }

            if let Some(_) = capture {
                s.push('x');
            }

            s.push(match dest.0 {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => panic!(),
            });

            s.push(match dest.1 {
                0 => '1',
                1 => '2',
                2 => '3',
                3 => '4',
                4 => '5',
                5 => '6',
                6 => '7',
                7 => '8',
                _ => panic!(),
            });

            if m.len() == 2 {
                if let Kind::Pawn = piece.kind {
                    s.push_str("e.p.");
                } else {
                    panic!("Only pawns should be able to have moves that consists of two moves.");
                }
            }
            if piece.kind == Kind::Pawn && (dest.1 == 7 || dest.1 == 0) {
                s.push_str("=Q");
            }
        }

        let other_color = match piece.color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        let mut g = self.clone();

        g.move_pieces(m);
        if let Some(v) = g.check_victory() {
            if result {
                if let VictoryStatus::Checkmate = v.0 {
                    s.push('#');
                    match piece.color {
                        Color::White => s.push_str(" 1-0"),
                        Color::Black => s.push_str(" 0-1"),
                    }
                } else {
                    s.push_str(" ½-½");
                }
            }
        } else if g.in_check(other_color) {
            s.push('+');
        }

        s
    }

    /// Turns a string in [algebraic
    /// notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)) (AN) into a move that can be passed to `move_pieces`.
    ///
    /// This function supports abbreviated algebraic notation, which means that certain characters
    /// can be removed, as long as it is unambiguous. For example, an 'x' (which signals a capture)
    /// is completely ignored, and can even be added to moves that doesn't end with a capture. The
    /// same goes for '=Q' (which signals a pawn promotion) and 'e.p.' (which signals *en passant*).
    ///
    /// The pieces can be represented by both letters and unicode symbols.
    ///
    /// To get the proper algebraic notation instead of the abbreviated one from a user, pass the
    /// result of `an_to_move` to `move_to_an`.
    ///
    /// This function returns `None` both if the input is malformed and if the move is invalid.
    /// There is currently no way to distinguish the two.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let mut game = Game::new();
    /// // Start by moving a knight from B1 to C3.
    /// let mut m = game.an_to_move("Nc3", Color::White);
    /// assert_eq!(m, Some(vec![((1, 0), (2, 2))]));
    /// game.move_pieces(&m.unwrap());
    ///
    /// // Move a black pawn from D7 to D5.
    /// m = game.an_to_move("d5", Color::Black);
    /// assert_eq!(m, Some(vec![((3, 6), (3, 4))]));
    /// game.move_pieces(&m.unwrap());
    ///
    /// m = game.an_to_move("e4", Color::White);
    /// assert_eq!(m, Some(vec![((4, 1), (4, 3))]));
    /// game.move_pieces(&m.unwrap());
    ///
    /// // Now the pawn at D5 can capture the pawn at E4.
    /// m = game.an_to_move("dxe4", Color::Black);
    /// assert_eq!(m, Some(vec![((3, 4), (4, 3))]));
    /// // Abbreviated notation is also valid.
    /// assert_eq!(m, game.an_to_move("de4", Color::Black));
    /// assert_eq!(m, game.an_to_move("de", Color::Black));
    /// game.move_pieces(&m.unwrap());
    ///
    /// // Fast-forwards a little.
    /// m = game.an_to_move("Nf3", Color::White);
    /// game.move_pieces(&m.unwrap());
    /// m = game.an_to_move("Ng5", Color::White);
    /// game.move_pieces(&m.unwrap());
    ///
    /// // Now both white knights can reach E4, so "Ne4" isn't enough.
    /// m = game.an_to_move("Ne4", Color::White);
    /// assert_eq!(m, None);
    ///
    /// // ...so we must specify the file the knight is moving from.
    /// m = game.an_to_move("Nce4", Color::White);
    /// assert_eq!(m, Some(vec![((2, 2), (4, 3))]));
    ///
    /// // We could also specify the rank, or both the rank and the file.
    /// assert_eq!(m, game.an_to_move("N3e4", Color::White));
    /// assert_eq!(m, game.an_to_move("Nc3e4", Color::White));
    /// game.move_pieces(&m.unwrap());
    ///
    /// // Fast forwards some more.
    /// m = game.an_to_move("Qf3", Color::White);
    /// game.move_pieces(&m.unwrap());
    /// m = game.an_to_move("Be2", Color::White);
    /// game.move_pieces(&m.unwrap());
    /// m = game.an_to_move("b3", Color::White);
    /// game.move_pieces(&m.unwrap());
    /// m = game.an_to_move("Bb2", Color::White);
    /// game.move_pieces(&m.unwrap());
    ///
    /// // Kingside castling.
    /// m = game.an_to_move("0-0", Color::White);
    /// assert_eq!(m, Some(vec![((4, 0), (5, 0)), ((5, 0), (6, 0)), ((7, 0), (5, 0))]));
    ///
    /// // Queenside castling.
    /// m = game.an_to_move("0-0-0", Color::White);
    /// assert_eq!(m, Some(vec![((4, 0), (3, 0)), ((3, 0), (2, 0)), ((0, 0), (3, 0))]));
    /// ```
    #[allow(clippy::all)]
    pub fn an_to_move(
        &self,
        s: &str,
        color: Color,
    ) -> Option<Vec<((usize, usize), (usize, usize))>> {
        let mut len = s.len();
        let mut result: Option<Vec<((usize, usize), (usize, usize))>> = None;
        let mut pos_x: Option<usize> = None;
        let mut pos_y: Option<usize> = None;
        let target_pos_x: Option<usize>;
        let mut target_pos_y: Option<usize> = None;

        if len < 2 {
            return None;
        }

        if s == "0-0" || s == "0-0-0" {
            let tmp = self.by_kind_and_color(Kind::King, color);
            let v = tmp.last().unwrap();
            for m in self.valid_moves(v.0) {
                if (s == "0-0" && (m[0].1).0 == 5) || (s == "0-0-0" && (m[0].1).0 == 3) {
                    return Some(m);
                }
            }
            return None;
        }

        let kind = match s.chars().nth(0).unwrap() {
            'R' | '\u{2656}' | '\u{265c}' => Kind::Rook,
            'N' | '\u{2658}' | '\u{265e}' => Kind::Knight,
            'B' | '\u{2657}' | '\u{265d}' => Kind::Bishop,
            'Q' | '\u{2655}' | '\u{265b}' => Kind::Queen,
            'K' | '\u{2654}' | '\u{265a}' => Kind::King,
            _ => Kind::Pawn,
        };

        if let Kind::Pawn = kind {
            if len >= 6 && &s[len - 4..len] == "e.p." {
                len -= 4;
            } else if len >= 4 && &s[len - 2..len] == "=Q" {
                len -= 2;
            }

            match string_to_pos(&s[len - 2..len]) {
                Ok(pos) => {
                    target_pos_x = Some(pos.0);
                    target_pos_y = Some(pos.1);
                }
                Err(_) => {
                    let mut last = s.chars().nth(len - 1).unwrap().to_string();
                    last.push('1');
                    match string_to_pos(&last) {
                        Ok(pos) => {
                            target_pos_x = Some(pos.0);
                        }
                        Err(_) => return None,
                    }
                }
            }

            if len >= 2 {
                match string_to_pos(&s[0..2]) {
                    Ok(pos) => {
                        if len > 2 {
                            pos_x = Some(pos.0);
                            pos_y = Some(pos.1);
                        }
                    }
                    Err(_) => {
                        let mut last = s.chars().nth(0).unwrap().to_string();
                        last.push('1');
                        match string_to_pos(&last) {
                            Ok(pos) => {
                                pos_x = Some(pos.0);
                            }
                            Err(_) => return None,
                        }
                    }
                }
            }
        } else {
            if len < 3 {
                return None;
            } else if len > 3 {
                match string_to_pos(&s[1..3]) {
                    Ok(pos) => {
                        pos_x = Some(pos.0);
                        pos_y = Some(pos.1);
                    }
                    Err(_) => {
                        let mut tile = s.chars().nth(1).unwrap().to_string();
                        if tile != "x" {
                            tile.push('1');
                            match string_to_pos(&tile) {
                                Ok(pos) => {
                                    pos_x = Some(pos.0);
                                }
                                Err(_) => {
                                    let mut rank = "E".to_string();
                                    rank.push(s.chars().nth(1).unwrap());
                                    match string_to_pos(&rank) {
                                        Ok(pos) => {
                                            pos_y = Some(pos.1);
                                        }
                                        Err(_) => return None,
                                    }
                                }
                            }
                        }
                    }
                }
            }

            match string_to_pos(&s[len - 2..len]) {
                Ok(pos) => {
                    target_pos_x = Some(pos.0);
                    target_pos_y = Some(pos.1);
                }
                Err(_) => return None,
            }
        }

        let mut last: (usize, usize);
        let mut found = false;
        for p in self.by_kind_and_color(kind, color) {
            if pos_x.unwrap_or((p.0).0) == (p.0).0 && pos_y.unwrap_or((p.0).1) == (p.0).1 {
                for v in self.valid_moves(p.0) {
                    last = v.last().unwrap().1;
                    if target_pos_x.unwrap_or(last.0) == last.0
                        && target_pos_y.unwrap_or(last.1) == last.1
                    {
                        if found {
                            return None;
                        } else {
                            found = true;
                            result = Some(v);
                        }
                    }
                }
            }
        }

        result
    }

    /// Turns a move tuple into a human readable description.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let game = Game::new();
    /// let m = game.valid_moves((4, 1));
    /// assert_eq!(game.move_to_string(&m[0][0]), "Moving white pawn from E2 to E4");
    /// ```
    #[allow(clippy::all)]
    pub fn move_to_string(&self, m: &((usize, usize), (usize, usize))) -> String {
        let mut s = String::new();
        let from = m.0;
        let to = m.1;

        let from_string = match pos_to_string(from) {
            Ok(s) => s,
            Err(e) => panic!(
                "Invalid position ({}, {}). Error code {}",
                from.0, from.1, e
            ),
        };
        let to_string = match pos_to_string(to) {
            Ok(s) => s,
            Err(e) => panic!("Invalid position ({}, {}). Error code {}", to.0, to.1, e),
        };

        if let Some(p) = self.get_from_pos(from) {
            s.push_str(&format!("Moving {} {} ", p.color, p.kind));
        } else {
            s.push_str("Moving ");
        }
        s.push_str(&format!("from {} to ", from_string));

        if let Some(p) = self.get_from_pos(to) {
            s.push_str(&format!("{} {} at ", p.color, p.kind));
        }
        s.push_str(&format!("{}", to_string));

        s
    }

    /// Turns an array of move tuples, like entries returned from valid_moves, into a human readable description.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let game = Game::new();
    /// let m = game.valid_moves((4, 1));
    /// assert_eq!(game.moves_to_string(&m[0]), "Moving white pawn from E2 to E4");
    /// ```
    #[allow(clippy::all)]
    pub fn moves_to_string(&self, m: &[((usize, usize), (usize, usize))]) -> String {
        let mut s = String::new();
        let mut first = true;
        for v in m {
            if !first {
                s.push_str("\n");
            }
            s.push_str(&self.move_to_string(v));
            first = false;
        }

        s
    }

    /// Returns the game board as a string.
    ///
    /// Set `unicode` to true if you want the pieces represented by their [unicode symbols]
    /// (https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode) instead of letters.
    /// If `unicode` is false the same letters that are used in [algebraic
    /// notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)) is
    /// used, with the addition of 'P' for pawns. The white pieces are represented by uppercase
    /// letters, while black are lowercase.
    ///
    /// # Eksamples
    ///
    /// ```
    /// # use chess::*;
    /// let game = Game::new();
    /// let mut board = game.board_to_string(false);
    /// assert_eq!(board,
    ///            "rnbqkbnr\
    ///           \npppppppp\
    ///           \n        \
    ///           \n        \
    ///           \n        \
    ///           \n        \
    ///           \nPPPPPPPP\
    ///           \nRNBQKBNR");
    ///
    /// board = game.board_to_string(true);
    /// assert_eq!(board,
    ///            "♜♞♝♛♚♝♞♜\
    ///           \n♟♟♟♟♟♟♟♟\
    ///           \n        \
    ///           \n        \
    ///           \n        \
    ///           \n        \
    ///           \n♙♙♙♙♙♙♙♙\
    ///           \n♖♘♗♕♔♗♘♖");
    /// ```
    #[allow(clippy::all)]
    pub fn board_to_string(&self, unicode: bool) -> String {
        let mut s = String::new();
        let mut y: usize;

        for y1 in 0..8 {
            y = 7 - y1;
            for x in 0..8 {
                s.push(if let Some(p) = self.get_from_pos((x, y)) {
                    match p.color {
                        Color::White => {
                            if unicode {
                                match p.kind {
                                    Kind::Pawn => '\u{2659}',
                                    Kind::Rook => '\u{2656}',
                                    Kind::Knight => '\u{2658}',
                                    Kind::Bishop => '\u{2657}',
                                    Kind::Queen => '\u{2655}',
                                    Kind::King => '\u{2654}',
                                }
                            } else {
                                match p.kind {
                                    Kind::Pawn => 'P',
                                    Kind::Rook => 'R',
                                    Kind::Knight => 'N',
                                    Kind::Bishop => 'B',
                                    Kind::Queen => 'Q',
                                    Kind::King => 'K',
                                }
                            }
                        }
                        Color::Black => {
                            if unicode {
                                match p.kind {
                                    Kind::Pawn => '\u{265f}',
                                    Kind::Rook => '\u{265c}',
                                    Kind::Knight => '\u{265e}',
                                    Kind::Bishop => '\u{265d}',
                                    Kind::Queen => '\u{265b}',
                                    Kind::King => '\u{265a}',
                                }
                            } else {
                                match p.kind {
                                    Kind::Pawn => 'p',
                                    Kind::Rook => 'r',
                                    Kind::Knight => 'n',
                                    Kind::Bishop => 'b',
                                    Kind::Queen => 'q',
                                    Kind::King => 'k',
                                }
                            }
                        }
                    }
                } else {
                    ' '
                });
            }

            if y != 0 {
                s.push_str("\n");
            }
        }
        s
    }

    fn save_board(&mut self) {
        self.board_history.push(self.board);
    }

    /// Checks whether there has occured a three fold repetition.
    #[allow(clippy::all)]
    pub fn three_fold_repetition(&self) -> bool {
        if self.board_history.len() >= 3 {
            info!("Checking for three fold repetition");
            let mut matches = 0;
            let last = match self.board_history.last() {
                Some(v) => v,
                None => panic!(),
            };
            'rep: for v in &self.board_history {
                for x in 0..8 {
                    for y in 0..8 {
                        if v[x][y] != last[x][y] {
                            continue 'rep;
                        }
                    }
                }
                matches += 1;
            }

            if matches >= 3 {
                return true;
            }
        }

        false
    }

    /// Checks whether a player can invoke the fifty-move-rule
    pub fn fifty_move_rule(&self) -> bool {
        self.seventy_five_move_rule >= 50
    }
}

/// Turns a position on the board from a string, like B3, to a tuple, like (1, 2).
///
/// Returns a Result containing the tuple, or an error if the given string was too long, or wasn't
/// a valid position. Remember to trimming or slicing user input before running it through this
/// function.
///
/// # Eksamples
///
/// ```
/// # use chess::*;
/// assert_eq!(string_to_pos("A1"), Ok((0, 0)));
/// assert_eq!(string_to_pos("F3"), Ok((5, 2)));
///
/// // Too long strings causes Err(1)
/// assert_eq!(string_to_pos("A1 "), Err(1));
/// // Invalid positions causes Err(2)
/// assert_eq!(string_to_pos("C9"), Err(2));
/// ```
pub fn string_to_pos(string: &str) -> Result<(usize, usize), i32> {
    if string.len() != 2 {
        return Err(1);
    }

    let bytes = string.as_bytes();
    let x: u8;
    let y: u8;
    if bytes[0] >= 65 && bytes[0] <= 72 {
        x = bytes[0] - 65;
    } else if bytes[0] >= 97 && bytes[0] <= 104 {
        x = bytes[0] - 97;
    } else {
        return Err(2);
    }

    if bytes[1] >= 49 && bytes[1] <= 56 {
        y = bytes[1] - 49;
    } else {
        return Err(2);
    }

    Ok((x as usize, y as usize))
}

/// Turns a position on the board from a tuple, like (3, 5), to proper chess notation, like D6.
///
/// Returns a Result containing the string, or an error if the given tuple was out of bounds.
///
/// # Eksamples
///
/// ```
/// # use chess::*;
/// assert_eq!(pos_to_string((3, 5)), Ok("D6".to_string()));
/// assert_eq!(pos_to_string((0, 0)), Ok("A1".to_string()));
/// assert_eq!(pos_to_string((7, 7)), Ok("H8".to_string()));
///
/// // Returns Err(1) when the values are out of bounds.
/// assert_eq!(pos_to_string((8, 8)), Err(1));
/// ```
#[allow(clippy)]
pub fn pos_to_string(pos: (usize, usize)) -> Result<String, i32> {
    if pos.0 > 7 || pos.1 > 7 {
        return Err(1);
    }

    let mut x: u8 = 0;
    let mut y: u8 = 0;
    for _ in 0..pos.0 {
        x += 1;
    }
    for _ in 0..pos.1 {
        y += 1;
    }

    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(65 + x);
    bytes.push(49 + y);

    match String::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(_) => Err(2),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_string_to_pos() {
//         assert_eq!(string_to_pos("A1"), Ok((0, 0)));
//         assert_eq!(string_to_pos("C6"), Ok((2, 5)));
//         assert_eq!(string_to_pos("c6"), Ok((2, 5)));
//         assert_eq!(string_to_pos("H8"), Ok((7, 7)));

//         assert_eq!(string_to_pos("C9"), Err(2));
//         assert_eq!(string_to_pos("I5"), Err(2));
//         assert_eq!(string_to_pos("I59"), Err(1));
//         assert_eq!(string_to_pos("C5 "), Err(1));
//         assert_eq!(string_to_pos("5C"), Err(2));
//     }

//     #[test]
//     fn test_pos_to_string() {
//         assert_eq!(pos_to_string((0,0)), Ok("A1".to_string()));
//         assert_eq!(pos_to_string((7,7)), Ok("H8".to_string()));
//         assert_eq!(pos_to_string((3,5)), Ok("D6".to_string()));

//         assert_eq!(pos_to_string((8,8)), Err(1));
//         assert_eq!(pos_to_string((20,1)), Err(1));
//         assert_eq!(pos_to_string((2,9)), Err(1));
//     }

//     #[test]
//     fn test_raw_moves() {
//         let mut game = Game::new_empty();
//         game.set_at_pos((3,3), Some(&WHITE[1]));
//         let moves = game.raw_moves((3,3));
//         assert_eq!(moves.len(), 14);
//     }

//     #[test]
//     fn test_check_for_check() {
//         let mut game = Game::new_empty();
//         game.set_at_pos((1, 2), Some(&WHITE[4]));
//         game.set_at_pos((0, 0), Some(&BLACK[5]));
//         game.set_at_pos((6, 7), Some(&WHITE[5]));

//         assert!(game.check_for_check((0,0), (1,0)));
//     }

//     #[test]
//     fn test_print() {
//         let game = Game::new();
//         let mut board = game.board_to_string(false);
//         assert_eq!(board,
//                    "rnbqkbnr\
//                   \npppppppp\
//                   \n        \
//                   \n        \
//                   \n        \
//                   \n        \
//                   \nPPPPPPPP\
//                   \nRNBQKBNR");

//         board = game.board_to_string(true);
//         assert_eq!(board,
//                    "♜♞♝♛♚♝♞♜\
//                   \n♟♟♟♟♟♟♟♟\
//                   \n        \
//                   \n        \
//                   \n        \
//                   \n        \
//                   \n♙♙♙♙♙♙♙♙\
//                   \n♖♘♗♕♔♗♘♖");
//     }
// }
