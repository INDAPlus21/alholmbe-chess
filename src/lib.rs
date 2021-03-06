use colored::*;
use std::io;
mod tests;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
  InProgress,
  Check,
  Checkmate,
  Stalemate,
  GameOver,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Position(usize, usize);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
  White,
  Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
  King(Colour),
  Rook(Colour),
  Bishop(Colour),
  Queen(Colour),
  Knight(Colour),
  Pawn(Colour),
}

impl Piece {
  pub fn get_colour(&self) -> Colour {
    // figure out how to pattern match the piece kind because this is ugly
    match self {
      Piece::Pawn(Colour::White) => Colour::White,
      Piece::Pawn(Colour::Black) => Colour::Black,
      Piece::Rook(Colour::White) => Colour::White,
      Piece::Rook(Colour::Black) => Colour::Black,
      Piece::Knight(Colour::White) => Colour::White,
      Piece::Knight(Colour::Black) => Colour::Black,
      Piece::Bishop(Colour::White) => Colour::White,
      Piece::Bishop(Colour::Black) => Colour::Black,
      Piece::Queen(Colour::White) => Colour::White,
      Piece::Queen(Colour::Black) => Colour::Black,
      Piece::King(Colour::White) => Colour::White,
      Piece::King(Colour::Black) => Colour::Black,
    }
  }

  fn to_ascii(&self) -> char {
    match self {
      Piece::Pawn(Colour::White) => '\u{2659}',
      Piece::Rook(Colour::White) => '\u{2656}',
      Piece::Knight(Colour::White) => '\u{2658}',
      Piece::Bishop(Colour::White) => '\u{2657}',
      Piece::Queen(Colour::White) => '\u{2655}',
      Piece::King(Colour::White) => '\u{2654}',
      Piece::Pawn(Colour::Black) => '\u{265F}',
      Piece::Rook(Colour::Black) => '\u{265C}',
      Piece::Knight(Colour::Black) => '\u{265E}',
      Piece::Bishop(Colour::Black) => '\u{265D}',
      Piece::Queen(Colour::Black) => '\u{265B}',
      Piece::King(Colour::Black) => '\u{265A}',
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Castling {
  white_queen: bool,
  white_king: bool,
  black_queen: bool,
  black_king: bool,
}

#[derive(Debug, Clone)]
pub struct Game {
  state: GameState,
  name: String,
  board: [[Option<Piece>; 8]; 8],
  turn: Colour,
  castling: Castling,
  promote: (bool, String),
}

impl Game {
  pub fn new() -> Game {
    let standard_starting_board =
      String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    Game::new_from_fen(standard_starting_board)
  }

  pub fn new_from_fen(fen_string: String) -> Game {
    let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    let lines: Vec<&str> = fen_string.split(' ').collect();
    let positions: Vec<&str> = lines[0].split('/').collect();

    let mut i = 7;
    for row in 0..=7 {
      let mut col = 0;
      for ch in positions[i].chars() {
        board[row][col] = match ch {
          'r' => Some(Piece::Rook(Colour::Black)),
          'n' => Some(Piece::Knight(Colour::Black)),
          'b' => Some(Piece::Bishop(Colour::Black)),
          'q' => Some(Piece::Queen(Colour::Black)),
          'k' => Some(Piece::King(Colour::Black)),
          'p' => Some(Piece::Pawn(Colour::Black)),
          'R' => Some(Piece::Rook(Colour::White)),
          'N' => Some(Piece::Knight(Colour::White)),
          'B' => Some(Piece::Bishop(Colour::White)),
          'Q' => Some(Piece::Queen(Colour::White)),
          'K' => Some(Piece::King(Colour::White)),
          'P' => Some(Piece::Pawn(Colour::White)),
          _ => {
            col += ch.to_digit(10).unwrap() as usize - 1;
            None
          }
        };
        col += 1;
      }

      if i != 0 {
        i -= 1;
      }
    }

    let turn = match lines[1] {
      "w" => Colour::White,
      _ => Colour::Black,
    };

    let mut castling = Castling {
      white_queen: false,
      white_king: false,
      black_queen: false,
      black_king: false,
    };
    for c in lines[2].chars() {
      match c {
        '-' => break,
        'Q' => castling.white_queen = true,
        'K' => castling.white_king = true,
        'q' => castling.black_queen = true,
        'k' => castling.black_king = true,
        _ => break,
      };
    }

    // println!("en passant targets = {}", lines[3]);

    // println!("halfmoves = {}", lines[4]);

    // println!("fullmoves = {}", lines[5]);

    let mut game = Game {
      turn,
      name: String::from("yoo"),
      state: GameState::InProgress,
      board,
      castling,
      promote: (false, String::new()),
    };

    if game.turn == Colour::White {
      if game.check(String::from("white")) {
        game.state = GameState::Check;
      }
    } else {
      if game.check(String::from("black")) {
        game.state = GameState::Check;
      }
    }

    let exists = game.move_exists();
    if !exists && game.state == GameState::Check {
      game.state = GameState::Checkmate;
    } else if !exists && game.state == GameState::InProgress {
      game.state = GameState::Stalemate;
    }

    game
  }

  fn print_board(&self) {
    println!("\n\t  BLACK");
    let mut col_number = 8;
    for row in self.board.iter().rev() {
      let col_number_str = format!("{}", col_number);
      print!("{}", col_number_str.green());
      col_number -= 1;
      for maybe_piece in row {
        // turn piece from Option to beautiful ascii chess pieces
        let c = match maybe_piece {
          Some(piece) => piece.to_ascii(),
          None => ' ',
        };
        print!("{:?}", c);
      }
      println!();
    }
    println!("{}", " 'A''B''C''D''E''F''G''H'".green());
    println!("\t  WHITE\n");
  }

  pub fn play() {
    let mut game = Game::new();
    while game.state != GameState::GameOver {
      game.print_board();
      println!("STATE OF THE GAME = {:?}", game.get_game_state());
      match game.turn {
        Colour::White => {
          println!("move for white (from, to) EXAMPLE a2a4: ");
        }
        Colour::Black => {
          println!("move for black (from, to) EXAMPLE a7a5: ");
        }
      };
      let mut mv = String::new();
      io::stdin().read_line(&mut mv).expect("Failed to read line");
      // skip the rest of the current iteration if we get a illegal input
      let actual_move: String = match mv.trim().parse() {
        Ok(pos) => pos,
        Err(_) => continue,
      };
      if actual_move == "quit" {
        break;
      }
      let mut from = String::from("");
      let mut to = String::from("");
      for (i, c) in actual_move.chars().enumerate() {
        if i == 0 || i == 1 {
          from.push(c);
        } else {
          to.push(c);
        }
      }
      // kolla ifall from och to ar legit moves
      if !Game::check_input(from.clone(), to.clone()) {
        println!("{}", "illegal input!".red());
        continue;
      }
      println!("from: {}, to: {}", from, to);
      match game.make_move(from.clone(), to.clone()) {
        Some(_) => {}
        None => {
          println!("{}", "illegal move!".red());
          continue;
        }
      }

      if game.promote.0 {
        loop {
          println!(
            "{}",
            "Promote your pawn! q for queen, r for rook, k for knight and b for bishop: ".green()
          );
          let mut promotion = String::new();
          io::stdin()
            .read_line(&mut promotion)
            .expect("Failed to read line");
          let promotion: String = match promotion.trim().parse() {
            Ok(piece) => piece,
            Err(_) => continue,
          };
          if promotion == String::from("q") {
            game.set_promotion(game.promote.1.clone(), 'q');
            break;
          } else if promotion == String::from("r") {
            game.set_promotion(game.promote.1.clone(), 'r');
            break;
          } else if promotion == String::from("k") {
            game.set_promotion(game.promote.1.clone(), 'k');
            break;
          } else if promotion == String::from("b") {
            game.set_promotion(game.promote.1.clone(), 'b');
            break;
          } else {
            println!("{}", "Only acceptable input is q, r, k or b.".red());
          }
        }
      }
      game.promote = (false, String::new());

      if game.state == GameState::Checkmate || game.state == GameState::Stalemate {
        println!("THE RESULT OF THE GAME IS = {:?}", game.state);
        game.state = GameState::GameOver;
      }
    }
  }

  fn check_input(from: String, to: String) -> bool {
    let every_tile = Game::get_all_tiles();
    let mut legit: (bool, bool) = (false, false);
    for tile in every_tile {
      if tile == from {
        legit.0 = true;
      }
      if tile == to {
        legit.1 = true;
      }
    }
    if legit == (true, true) {
      true
    } else {
      false
    }
  }

  fn get_piece_at(&self, position: String) -> Option<Piece> {
    let pos = Game::parse_string(&position);
    self.board[pos.0][pos.1]
  }

  pub fn get_all_tiles() -> Vec<String> {
    let mut tiles = vec![];
    for i in 0..=7 {
      for j in 0..=7 {
        tiles.push(Game::parse_coordinates(Position(i, j)));
      }
    }
    tiles
  }

  pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
    let old_position = Game::parse_string(&_from);
    let new_position = Game::parse_string(&_to);
    // Can't move a None piece
    let moving_piece = self.board[old_position.0][old_position.1]?;
    if moving_piece.get_colour() != self.turn {
      return None;
    }

    let mut make_move = false;
    let possible_moves = Game::get_possible_moves(self, _from.clone())?;

    for mv in possible_moves {
      if mv == _to {
        make_move = true;
        break;
      }
    }
    if make_move {
      self.board[new_position.0][new_position.1] = Some(moving_piece);
      self.board[old_position.0][old_position.1] = None;
      // check for promotion
      match (new_position.0, moving_piece) {
        (7, Piece::Pawn(Colour::White)) => {
          self.promote = (true, _to);
        }
        (0, Piece::Pawn(Colour::Black)) => {
          self.promote = (true, _to);
        }
        _ => {}
      };
      self.turn = match self.turn {
        Colour::White => Colour::Black,
        Colour::Black => Colour::White,
      };
    } else {
      // requested move is not possible to make
      return None;
    }

    // guaranteed not in check if we are here
    self.state = GameState::InProgress;

    if self.turn == Colour::Black {
      // check if black is now in check
      if self.check(String::from("black")) {
        self.state = GameState::Check;
      }
      // check blacks possible moves
      // scan board for black pieces
      // for every piece, check if there is a possible move
    } else {
      // check if white is now in check
      if self.check(String::from("white")) {
        self.state = GameState::Check;
      }
      // check whites possible moves
      // scan board for white pieces
      // for every piece, check if there is a possible move
    }

    let exists = self.move_exists();
    if !exists && self.state == GameState::Check {
      self.state = GameState::Checkmate;
    } else if !exists && self.state == GameState::InProgress {
      self.state = GameState::Stalemate;
    }

    // check for possibles moves here
    // if none and in check -> checkmate
    // if none and not in check -> stalemate
    // if moves -> neither check- or stalemate

    Some(self.state)
  }

  fn move_exists(&self) -> bool {
    for i in 0..=7 {
      for j in 0..=7 {
        match self.board[i][j] {
          Some(piece) => {
            if piece.get_colour() == self.turn {
              let moves = self
                .get_possible_moves(Game::parse_coordinates(Position(i, j)))
                .unwrap();
              if moves.len() != 0 {
                return true;
              }
            }
          }
          None => {}
        }
      }
    }

    false
  }

  fn colour_from_string(s: &str) -> Colour {
    if s == "white" {
      Colour::White
    } else {
      Colour::Black
    }
  }

  fn check(&self, possibly_checked_color: String) -> bool {
    let king;
    let c;
    if possibly_checked_color == "white" {
      king = self.get_king(String::from("white"));
      c = Game::colour_from_string("black");

      // SCAN FOR PAWN ATTACKS
      // up left
      if king.0 < 7 && king.1 > 0 {
        if self.board[king.0 + 1][king.1 - 1] == Some(Piece::Pawn(c)) {
          return true;
        }
      }
      // up right
      if king.0 < 7 && king.1 < 7 {
        if self.board[king.0 + 1][king.1 + 1] == Some(Piece::Pawn(c)) {
          return true;
        }
      }
    } else {
      king = self.get_king(String::from("black"));
      c = Game::colour_from_string("white");
      // scan for pawn attacks
      // down left
      if king.0 > 0 && king.1 > 0 {
        if self.board[king.0 - 1][king.1 - 1] == Some(Piece::Pawn(c)) {
          return true;
        }
      }
      // down right
      if king.0 > 0 && king.1 < 7 {
        if self.board[king.0 - 1][king.1 + 1] == Some(Piece::Pawn(c)) {
          return true;
        }
      }
    }

    let king_piece = self.board[king.0][king.1].unwrap();
    // scan for vertical attacks above king
    for row in (king.0 + 1)..=7 {
      match self.board[row][king.1] {
        Some(piece) => {
          if piece == Piece::Rook(c) || piece == Piece::Queen(c) {
            return true;
          } else {
            break;
          }
        }
        None => {}
      }
    }

    // scan for vertical attacks below king
    for row in (0..king.0).rev() {
      match self.board[row][king.1] {
        Some(piece) => {
          if piece == Piece::Rook(c) || piece == Piece::Queen(c) {
            return true;
          } else {
            break;
          }
        }
        None => {}
      }
    }

    // scan for horizontal attacks right to the king
    for col in (king.1 + 1)..=7 {
      match self.board[king.0][col] {
        Some(piece) => {
          if piece == Piece::Rook(c) || piece == Piece::Queen(c) {
            return true;
          } else {
            break;
          }
        }
        None => {}
      }
    }
    // scan for horizontal attacks left to the king
    for col in (0..king.1).rev() {
      match self.board[king.0][col] {
        Some(piece) => {
          if piece == Piece::Rook(c) || piece == Piece::Queen(c) {
            return true;
          } else {
            break;
          }
        }
        None => {}
      }
    }

    // scan for top right diagonal attacks
    let mut i = king.0;
    let mut j = king.1;
    loop {
      if i == 7 || j == 7 {
        break;
      }
      i += 1;
      j += 1;

      match self.board[i][j] {
        Some(piece) => {
          if piece == Piece::Bishop(c) || piece == Piece::Queen(c) {
            return true;
          } else {
            break;
          }
        }
        None => {}
      }
    }

    // scan for down right diagonal attacks
    let mut i = king.0;
    let mut j = king.1;
    loop {
      if i == 0 || j == 7 {
        break;
      }
      i -= 1;
      j += 1;

      match self.board[i][j] {
        Some(piece) => {
          if piece == Piece::Bishop(c) || piece == Piece::Queen(c) {
            return true;
          } else {
            break;
          }
        }
        None => {}
      }
    }

    // scan for top left diagonal attacks
    let mut i = king.0;
    let mut j = king.1;
    loop {
      if i == 7 || j == 0 {
        break;
      }
      i += 1;
      j -= 1;

      match self.board[i][j] {
        Some(piece) => {
          if piece == Piece::Bishop(c) || piece == Piece::Queen(c) {
            return true;
          } else {
            break;
          }
        }
        None => {}
      }
    }

    // scan for down left diagonal attacks
    let mut i = king.0;
    let mut j = king.1;
    loop {
      if i == 0 || j == 0 {
        break;
      }
      i -= 1;
      j -= 1;

      match self.board[i][j] {
        Some(piece) => {
          if piece == Piece::Bishop(c) || piece == Piece::Queen(c) {
            return true;
          } else {
            break;
          }
        }
        None => {}
      }
    }
    // scan for knight attacks
    // top left fw move
    if king.1 > 0 && king.0 < 6 {
      match self.board[king.0 + 2][king.1 - 1] {
        Some(piece) => {
          if piece.get_colour() != king_piece.get_colour() && piece == Piece::Knight(c) {
            return true;
          }
        }
        None => {}
      }
    }

    // top right fw move
    if king.1 < 7 && king.0 < 6 {
      match self.board[king.0 + 2][king.1 + 1] {
        Some(piece) => {
          if piece.get_colour() != king_piece.get_colour() && piece == Piece::Knight(c) {
            return true;
          }
        }
        None => {}
      }
    }

    // left side top move
    if king.1 > 1 && king.0 < 7 {
      match self.board[king.0 + 1][king.1 - 2] {
        Some(piece) => {
          if piece.get_colour() != king_piece.get_colour() && piece == Piece::Knight(c) {
            return true;
          }
        }
        None => {}
      }
    }

    // left side down move
    if king.1 > 1 && king.0 > 0 {
      match self.board[king.0 - 1][king.1 - 2] {
        Some(piece) => {
          if piece.get_colour() != king_piece.get_colour() && piece == Piece::Knight(c) {
            return true;
          }
        }
        None => {}
      }
    }

    // down left move
    if king.1 > 0 && king.0 > 1 {
      match self.board[king.0 - 2][king.1 - 1] {
        Some(piece) => {
          if piece.get_colour() != king_piece.get_colour() && piece == Piece::Knight(c) {
            return true;
          }
        }
        None => {}
      }
    }

    // down right move
    if king.1 < 7 && king.0 > 1 {
      match self.board[king.0 - 2][king.1 + 1] {
        Some(piece) => {
          if piece.get_colour() != king_piece.get_colour() && piece == Piece::Knight(c) {
            return true;
          }
        }
        None => {}
      }
    }

    // right down move
    if king.1 < 6 && king.0 > 0 {
      match self.board[king.0 - 1][king.1 + 2] {
        Some(piece) => {
          if piece.get_colour() != king_piece.get_colour() && piece == Piece::Knight(c) {
            return true;
          }
        }
        None => {}
      }
    }

    // right up move
    if king.1 < 6 && king.0 < 7 {
      match self.board[king.0 + 1][king.1 + 2] {
        Some(piece) => {
          if piece.get_colour() != king_piece.get_colour() && piece == Piece::Knight(c) {
            return true;
          }
        }
        None => {}
      }
    }
    false
  }

  fn get_king(&self, colour: String) -> Position {
    for i in 0..=7 {
      for j in 0..=7 {
        match self.board[i][j] {
          Some(piece) => match piece {
            Piece::King(Colour::White) => {
              if colour == "white" {
                return Position(i, j);
              }
            }
            Piece::King(Colour::Black) => {
              if colour == "black" {
                return Position(i, j);
              }
            }
            _ => {}
          },
          _ => {}
        }
      }
    }
    // will never happen, fix error handling
    Position(0, 0)
  }

  fn parse_string(position: &String) -> Position {
    // turn string into two chars
    // match the first one with a num
    // parse the other one - 1 to a int
    let mut chars = vec![];
    for ch in position.chars() {
      chars.push(ch);
    }

    let col = match chars[0] {
      'a' => 0,
      'b' => 1,
      'c' => 2,
      'd' => 3,
      'e' => 4,
      'f' => 5,
      'g' => 6,
      'h' => 7,
      _ => 8, // handle error
    };
    let row = chars[1].to_digit(10).unwrap() - 1; // handle error

    return Position(row as usize, col);
  }

  fn parse_coordinates(position: Position) -> String {
    let col = match position.1 {
      0 => "a",
      1 => "b",
      2 => "c",
      3 => "d",
      4 => "e",
      5 => "f",
      6 => "g",
      7 => "h",
      _ => "x", // handle error
    };

    let row = (position.0 + 1).to_string();
    let fin = String::from(col) + &row;
    fin
  }

  pub fn set_promotion(&mut self, position: String, new_piece: char) -> () {
    if !self.promote.0 {
      return;
    }
    let piece = self.get_piece_at(position.clone()).unwrap();
    let pos = Game::parse_string(&position);
    match new_piece {
      'q' => self.board[pos.0][pos.1] = Some(Piece::Queen(piece.get_colour())),
      'r' => self.board[pos.0][pos.1] = Some(Piece::Rook(piece.get_colour())),
      'b' => self.board[pos.0][pos.1] = Some(Piece::Bishop(piece.get_colour())),
      'k' => self.board[pos.0][pos.1] = Some(Piece::Knight(piece.get_colour())),
      _ => {}
    }
  }

  pub fn get_game_state(&self) -> &GameState {
    &self.state
  }

  fn not_in_check_after_move(&self, from: &Position, to: &Position, moving_piece: Piece) -> bool {
    // 1. make move
    // clone board
    let mut fake_game = self.clone();
    fake_game.board[to.0][to.1] = Some(moving_piece);
    fake_game.board[from.0][from.1] = None;
    if moving_piece.get_colour() == Colour::White {
      // check if white is still in check
      if fake_game.check(String::from("white")) {
        fake_game.state = GameState::Check;
        return false;
      }
    } else {
      // check if black is still in check
      if fake_game.check(String::from("black")) {
        fake_game.state = GameState::Check;
        return false;
      }
    }
    true
  }

  pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
    let position = Game::parse_string(&_position);
    let moving_piece = self.board[position.0][position.1]?;
    let mut all_moves = Game::get_all_moves(self, position, moving_piece);
    let from = Game::parse_string(&_position);
    all_moves.retain(|to| self.not_in_check_after_move(&from, to, moving_piece));

    let mut str_moves = vec![];
    for mv in all_moves {
      str_moves.push(Game::parse_coordinates(mv));
    }

    Some(str_moves)
  }

  fn get_all_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let moves = match piece {
      Piece::Pawn(_) => self.get_pawn_moves(position, piece),
      Piece::Rook(_) => self.get_rook_moves(position, piece),
      Piece::Knight(_) => self.get_knight_moves(position, piece),
      Piece::Bishop(_) => self.get_bishop_moves(position, piece),
      Piece::Queen(_) => self.get_queen_moves(position, piece),
      Piece::King(_) => self.get_king_moves(position, piece),
    };

    moves
  }

  fn get_pawn_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let mut moves = vec![];

    match piece.get_colour() {
      Colour::White => {
        // 2 forward
        if position.0 == 1 {
          if self.board[3][position.1].is_none() && self.board[2][position.1].is_none() {
            moves.push(Position(3, position.1));
          }
        }

        // 1 forward
        if position.0 < 7 {
          if self.board[position.0 + 1][position.1].is_none() {
            moves.push(Position(position.0 + 1, position.1));
          }
        }

        // 1 diagonal to the right
        if position.1 < 7 {
          match self.board[position.0 + 1][position.1 + 1] {
            Some(attacked_piece) => {
              if attacked_piece.get_colour() == Colour::Black {
                moves.push(Position(position.0 + 1, position.1 + 1));
              }
            }
            None => {}
          }
        }

        // 1 diagonal to the left
        if position.1 > 0 {
          match self.board[position.0 + 1][position.1 - 1] {
            Some(attacked_piece) => {
              if attacked_piece.get_colour() == Colour::Black {
                moves.push(Position(position.0 + 1, position.1 - 1));
              }
            }
            None => {}
          }
        }
      }
      Colour::Black => {
        // 2fw
        if position.0 == 6 {
          if self.board[4][position.1].is_none() && self.board[5][position.1].is_none() {
            moves.push(Position(4, position.1));
          }
        }

        // 1 forward
        if position.0 > 0 {
          if self.board[position.0 - 1][position.1].is_none() {
            moves.push(Position(position.0 - 1, position.1));
          }
        }

        // 1 diagonal to the right
        if position.1 < 7 {
          match self.board[position.0 - 1][position.1 + 1] {
            Some(attacked_piece) => {
              if attacked_piece.get_colour() == Colour::White {
                moves.push(Position(position.0 - 1, position.1 + 1));
              }
            }
            None => {}
          }
        }

        // 1 diagonal to the left
        if position.1 > 0 {
          match self.board[position.0 - 1][position.1 - 1] {
            Some(attacked_piece) => {
              if attacked_piece.get_colour() == Colour::White {
                moves.push(Position(position.0 - 1, position.1 - 1));
              }
            }
            None => {}
          }
        }
      }
    }

    moves
  }

  fn get_rook_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    self.up_down_slides(position, piece)
  }

  fn get_knight_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let mut moves = vec![];
    // maximalt 8 moves
    // check all 8
    // for every check, check bounds, and attacked piece

    // top left fw move
    if position.1 > 0 && position.0 < 6 {
      if Game::is_legal(self.board[position.0 + 2][position.1 - 1], piece) {
        moves.push(Position(position.0 + 2, position.1 - 1));
      }
    }

    // top right fw move
    if position.1 < 7 && position.0 < 6 {
      if Game::is_legal(self.board[position.0 + 2][position.1 + 1], piece) {
        moves.push(Position(position.0 + 2, position.1 + 1));
      }
    }

    // left side top move
    if position.1 > 1 && position.0 < 7 {
      if Game::is_legal(self.board[position.0 + 1][position.1 - 2], piece) {
        moves.push(Position(position.0 + 1, position.1 - 2));
      }
    }

    // left side down move
    if position.1 > 1 && position.0 > 0 {
      if Game::is_legal(self.board[position.0 - 1][position.1 - 2], piece) {
        moves.push(Position(position.0 - 1, position.1 - 2));
      }
    }

    // down left move
    if position.1 > 0 && position.0 > 1 {
      if Game::is_legal(self.board[position.0 - 2][position.1 - 1], piece) {
        moves.push(Position(position.0 - 2, position.1 - 1));
      }
    }

    // down right move
    if position.1 < 7 && position.0 > 1 {
      if Game::is_legal(self.board[position.0 - 2][position.1 + 1], piece) {
        moves.push(Position(position.0 - 2, position.1 + 1));
      }
    }

    // right down move
    if position.1 < 6 && position.0 > 0 {
      if Game::is_legal(self.board[position.0 - 1][position.1 + 2], piece) {
        moves.push(Position(position.0 - 1, position.1 + 2));
      }
    }

    // right up move
    if position.1 < 6 && position.0 < 7 {
      if Game::is_legal(self.board[position.0 + 1][position.1 + 2], piece) {
        moves.push(Position(position.0 + 1, position.1 + 2));
      }
    }

    moves
  }

  fn get_bishop_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    self.diagonal_slides(position, piece)
  }

  fn get_queen_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let mut diagonal_moves = self.diagonal_slides(position.clone(), piece);
    let mut up_down_moves = self.up_down_slides(position.clone(), piece);

    diagonal_moves.append(&mut up_down_moves);
    diagonal_moves
  }

  fn get_king_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let mut moves = vec![];
    // left
    if position.1 > 0 {
      if Game::is_legal(self.board[position.0][position.1 - 1], piece) {
        moves.push(Position(position.0, position.1 - 1));
      }
    }
    // top left
    if position.0 < 7 && position.1 > 0 {
      if Game::is_legal(self.board[position.0 + 1][position.1 - 1], piece) {
        moves.push(Position(position.0 + 1, position.1 - 1));
      }
    }
    // top
    if position.0 < 7 {
      if Game::is_legal(self.board[position.0 + 1][position.1], piece) {
        moves.push(Position(position.0 + 1, position.1));
      }
    }
    // top right
    if position.0 < 7 && position.1 < 7 {
      if Game::is_legal(self.board[position.0 + 1][position.1 + 1], piece) {
        moves.push(Position(position.0 + 1, position.1 + 1));
      }
    }
    // right
    if position.1 < 7 {
      if Game::is_legal(self.board[position.0][position.1 + 1], piece) {
        moves.push(Position(position.0, position.1 + 1));
      }
    }
    // down right
    if position.0 > 0 && position.1 < 7 {
      if Game::is_legal(self.board[position.0 - 1][position.1 + 1], piece) {
        moves.push(Position(position.0 - 1, position.1 + 1));
      }
    }
    // down
    if position.0 > 0 {
      if Game::is_legal(self.board[position.0 - 1][position.1], piece) {
        moves.push(Position(position.0 - 1, position.1));
      }
    }
    // down left
    if position.0 > 0 && position.1 > 0 {
      if Game::is_legal(self.board[position.0 - 1][position.1 - 1], piece) {
        moves.push(Position(position.0 - 1, position.1 - 1));
      }
    }
    moves
  }

  fn diagonal_slides(&self, position: Position, piece: Piece) -> Vec<Position> {
    // bottom left
    let mut moves = vec![];
    if position.0 > 0 && position.1 > 0 {
      let mut i = position.0;
      let mut j = position.1;
      loop {
        i -= 1;
        j -= 1;

        let (should_add, should_break) = Game::is_legal_in_loop(self.board[i][j], piece);
        if should_add {
          moves.push(Position(i, j));
        }
        if should_break {
          break;
        }

        if i == 0 || j == 0 {
          break;
        }
      }
    }

    // top left
    if position.0 < 7 && position.1 > 0 {
      let mut i = position.0;
      let mut j = position.1;
      loop {
        i += 1;
        j -= 1;

        let (should_add, should_break) = Game::is_legal_in_loop(self.board[i][j], piece);
        if should_add {
          moves.push(Position(i, j));
        }
        if should_break {
          break;
        }

        if i == 7 || j == 0 {
          break;
        }
      }
    }

    // top right
    if position.0 < 7 && position.1 < 7 {
      let mut i = position.0;
      let mut j = position.1;
      loop {
        i += 1;
        j += 1;

        let (should_add, should_break) = Game::is_legal_in_loop(self.board[i][j], piece);
        if should_add {
          moves.push(Position(i, j));
        }
        if should_break {
          break;
        }

        if i == 7 || j == 7 {
          break;
        }
      }
    }

    // bottom right
    if position.0 > 0 && position.1 < 7 {
      let mut i = position.0;
      let mut j = position.1;
      loop {
        i -= 1;
        j += 1;

        let (should_add, should_break) = Game::is_legal_in_loop(self.board[i][j], piece);
        if should_add {
          moves.push(Position(i, j));
        }
        if should_break {
          break;
        }

        if i == 0 || j == 7 {
          break;
        }
      }
    }

    moves
  }

  fn up_down_slides(&self, position: Position, piece: Piece) -> Vec<Position> {
    let mut moves = vec![];
    // every tile forward (whites perspective)
    if position.0 < 7 {
      for i in (position.0 + 1)..=7 {
        let (should_add, should_break) = Game::is_legal_in_loop(self.board[i][position.1], piece);
        if should_add {
          moves.push(Position(i, position.1));
        }
        if should_break {
          break;
        }
      }
    }
    // every tile backward
    if position.0 > 0 {
      for i in (0..=position.0 - 1).rev() {
        let (should_add, should_break) = Game::is_legal_in_loop(self.board[i][position.1], piece);
        if should_add {
          moves.push(Position(i, position.1));
        }
        if should_break {
          break;
        }
      }
    }
    // every tile to right
    if position.1 < 7 {
      for i in (position.1 + 1)..=7 {
        let (should_add, should_break) = Game::is_legal_in_loop(self.board[position.0][i], piece);
        if should_add {
          moves.push(Position(position.0, i));
        }
        if should_break {
          break;
        }
      }
    }
    // every tile to left
    if position.1 > 0 {
      for i in (0..=(position.1 - 1)).rev() {
        let (should_add, should_break) = Game::is_legal_in_loop(self.board[position.0][i], piece);
        if should_add {
          moves.push(Position(position.0, i));
        }
        if should_break {
          break;
        }
      }
    }

    moves
  }

  fn is_legal(attacked_position: Option<Piece>, piece: Piece) -> bool {
    match attacked_position {
      Some(attacked_piece) => {
        if attacked_piece.get_colour() != piece.get_colour() {
          true
        } else {
          false
        }
      }
      None => true,
    }
  }

  fn is_legal_in_loop(attacked_position: Option<Piece>, piece: Piece) -> (bool, bool) {
    let mut should_break = false;
    let mut should_add = false;

    match attacked_position {
      Some(attacked_piece) => {
        if attacked_piece.get_colour() != piece.get_colour() {
          should_add = true;
        }
        should_break = true;
      }
      None => should_add = true,
    }

    (should_add, should_break)
  }
}
