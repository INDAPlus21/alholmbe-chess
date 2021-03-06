#[cfg(test)]
mod tests {
  mod init {
    use crate::Colour;
    use crate::Game;
    use crate::GameState;
    use crate::Piece;

    #[test]
    fn initializes_correctly() {
      //let game = Game::new();
      let game = Game::new_from_fen(String::from(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      ));
      assert_eq!(game.turn, Colour::White);
      assert_eq!(game.board[0][0].unwrap(), Piece::Rook(Colour::White));
      assert_eq!(game.board[0][1].unwrap(), Piece::Knight(Colour::White));
      assert_eq!(game.board[0][2].unwrap(), Piece::Bishop(Colour::White));
      assert_eq!(game.board[0][3].unwrap(), Piece::Queen(Colour::White));
      assert_eq!(game.board[0][4].unwrap(), Piece::King(Colour::White));
      assert_eq!(game.board[0][5].unwrap(), Piece::Bishop(Colour::White));
      assert_eq!(game.board[0][6].unwrap(), Piece::Knight(Colour::White));
      assert_eq!(game.board[0][7].unwrap(), Piece::Rook(Colour::White));

      for i in 0..=7 {
        assert_eq!(game.board[1][i].unwrap(), Piece::Pawn(Colour::White));
      }

      for i in 0..=7 {
        assert_eq!(game.board[2][i].is_none(), true);
      }

      for i in 0..=7 {
        assert_eq!(game.board[6][i].unwrap(), Piece::Pawn(Colour::Black));
      }

      assert_eq!(game.board[7][0].unwrap(), Piece::Rook(Colour::Black));
      assert_eq!(game.board[7][1].unwrap(), Piece::Knight(Colour::Black));
      assert_eq!(game.board[7][2].unwrap(), Piece::Bishop(Colour::Black));
      assert_eq!(game.board[7][3].unwrap(), Piece::Queen(Colour::Black));
      assert_eq!(game.board[7][4].unwrap(), Piece::King(Colour::Black));
      assert_eq!(game.board[7][5].unwrap(), Piece::Bishop(Colour::Black));
      assert_eq!(game.board[7][6].unwrap(), Piece::Knight(Colour::Black));
      assert_eq!(game.board[7][7].unwrap(), Piece::Rook(Colour::Black));

      let game = Game::new_from_fen(String::from("4k2r/6r1/8/8/8/8/3R4/R3K3 b Qk - 0 1"));
      assert_eq!(game.turn, Colour::Black);
    }

    #[test]
    fn random_fen_inits() {
      let game = Game::new_from_fen(String::from("4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1"));
      assert_eq!(game.turn, Colour::White);

      assert_eq!(game.board[0][0].unwrap(), Piece::Rook(Colour::White));
      assert_eq!(game.board[1][3].unwrap(), Piece::Rook(Colour::White));
      assert_eq!(game.board[6][6].unwrap(), Piece::Rook(Colour::Black));
      assert_eq!(game.board[7][7].unwrap(), Piece::Rook(Colour::Black));
      assert_eq!(game.board[7][6].is_none(), true);
      assert_eq!(game.board[0][4].unwrap(), Piece::King(Colour::White));
      assert_eq!(game.board[7][4].unwrap(), Piece::King(Colour::Black));
      assert_eq!(game.board[0][5].is_none(), true);

      let game = Game::new_from_fen(String::from(
        "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50",
      ));
      assert_eq!(game.turn, Colour::Black);

      assert_eq!(game.board[2][0].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[4][1].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[3][1].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[3][0].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[5][3].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[4][4].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[3][5].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[4][3].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[3][4].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[2][5].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[3][7].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[4][7].unwrap(), Piece::Pawn(Colour::Black));

      assert_eq!(game.board[2][7].unwrap(), Piece::King(Colour::White));
      assert_eq!(game.board[6][5].unwrap(), Piece::King(Colour::Black));

      assert_eq!(game.board[0][0].is_none(), true);
      assert_eq!(game.board[7][7].is_none(), true);
      assert_eq!(game.board[5][7].is_none(), true);
    }

    #[test]
    fn kings_are_in_place() {
      let game = Game::new();
      assert_eq!(
        game.board[7][4].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::King(Colour::Black)
      );
      assert_eq!(
        game.board[0][4].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::King(Colour::White)
      );
    }
    #[test]
    fn queens_are_in_place() {
      let game = Game::new();
      assert_eq!(
        game.board[7][3].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::Queen(Colour::Black)
      );
      assert_eq!(
        game.board[0][3].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::Queen(Colour::White)
      );
    }

    #[test]
    fn rooks_are_in_place() {
      let game = Game::new();
      assert_eq!(
        game.board[0][0].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::Rook(Colour::White)
      );
      assert_eq!(
        game.board[7][7].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::Rook(Colour::Black)
      );
    }

    #[test]
    fn nones_in_place() {
      let game = Game::new();
      assert_eq!(game.board[3][5].is_none(), true);
      assert_eq!(game.board[4][2].is_none(), true);
    }

    #[test]
    fn gets_game_state() {
      let mut game = Game::new();
      assert_eq!(*game.get_game_state(), GameState::InProgress);
      game.state = GameState::GameOver;
      assert_eq!(*game.get_game_state(), GameState::GameOver);
    }
  }

  mod movement {
    use crate::Colour;
    use crate::Game;
    use crate::Piece;
    use crate::Position;
    #[test]
    fn parsing_positions() {
      //let game = Game::new();
      assert_eq!(Game::parse_string(&String::from("a5")), Position(4, 0));
      assert_eq!(Game::parse_string(&String::from("d7")), Position(6, 3));
      assert_eq!(Game::parse_string(&String::from("h7")), Position(6, 7));
    }

    #[test]
    fn random_moves() {}

    #[test]
    fn getting_possible_moves() {
      let game = Game::new();
      assert_eq!(game.get_possible_moves(String::from("a2")).is_some(), true);
      let mut m = game.get_possible_moves(String::from("a2")).unwrap();
      let mut m2 = vec![String::from("a3"), String::from("a4")];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      assert_eq!(game.get_possible_moves(String::from("c3")).is_none(), true);
      assert_eq!(game.get_possible_moves(String::from("c6")).is_none(), true);
      assert_eq!(game.get_possible_moves(String::from("d5")).is_none(), true);
      assert_eq!(game.get_possible_moves(String::from("e4")).is_none(), true);
    }
    #[test]
    fn move_twice_in_row() {
      let mut game = Game::new();
      // cant move white twice in a row
      assert_eq!(
        game
          .make_move(String::from("a2"), String::from("a4"))
          .is_some(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("b2"), String::from("b4"))
          .is_some(),
        false
      );

      assert_eq!(
        game
          .make_move(String::from("a7"), String::from("a5"))
          .is_some(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("f7"), String::from("f5"))
          .is_some(),
        false
      );
    }

    #[test]
    fn moving_none() {
      let mut game = Game::new();
      // cant move white twice in a row
      assert_eq!(
        game
          .make_move(String::from("a3"), String::from("a4"))
          .is_some(),
        false
      );
      assert_eq!(
        game
          .make_move(String::from("b4"), String::from("b5"))
          .is_some(),
        false
      );
      assert_eq!(
        game
          .make_move(String::from("b6"), String::from("b7"))
          .is_some(),
        false
      );
    }

    #[test]
    fn validate_input() {
      assert_eq!(
        Game::check_input(String::from("daggbfdlgh"), String::from("bfdnlgbvfdhb")),
        false
      );
      assert_eq!(
        Game::check_input(String::from("f2"), String::from("f9")),
        false
      );
      assert_eq!(
        Game::check_input(String::from("f0"), String::from("e0")),
        false
      );
      assert_eq!(
        Game::check_input(String::from("a2"), String::from("a4")),
        true
      );
      assert_eq!(
        Game::check_input(String::from("b7"), String::from("b5")),
        true
      );
      assert_eq!(
        Game::check_input(String::from("g2"), String::from("g4")),
        true
      );
    }

    #[test]
    fn pawn_moves() {
      let game = Game::new();

      let mut m = game.get_all_moves(Position(1, 0), Piece::Pawn(Colour::White));
      let mut m2 = vec![Position(2, 0), Position(3, 0)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(1, 6), Piece::Pawn(Colour::White));
      let mut m2 = vec![Position(2, 6), Position(3, 6)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game2 = Game::new_from_fen(String::from(
        "8/5k2/4p3/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50",
      ));
      let mut m = game2.get_all_moves(Position(4, 3), Piece::Pawn(Colour::White));
      let mut m2 = vec![Position(5, 3), Position(5, 4)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      let mut m = game2.get_all_moves(Position(3, 0), Piece::Pawn(Colour::Black));
      let mut m2 = vec![];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }

    #[test]
    fn rook_moves() {
      let game = Game::new();
      let m = game.get_all_moves(Position(0, 0), Piece::Rook(Colour::White));
      let m2 = vec![];
      assert_eq!(m, m2);

      let m = game.get_all_moves(Position(7, 7), Piece::Rook(Colour::Black));
      let m2 = vec![];
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "8/3q1k2/3r4/1p1Pp2p/pP2Pp1P/P4P2/7K/8 b - - 99 50",
      ));
      let mut m = game.get_all_moves(Position(5, 3), Piece::Rook(Colour::Black));
      let mut m2 = vec![
        Position(4, 3),
        Position(5, 4),
        Position(5, 5),
        Position(5, 6),
        Position(5, 7),
        Position(5, 2),
        Position(5, 1),
        Position(5, 0),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      // let mut m = game.get_possible_moves(String::from("d6")).unwrap();
      // let mut m2 = vec![
      //   String::from("a6"),
      //   String::from("b6"),
      //   String::from("c6"),
      //   String::from("e6"),
      //   String::from("f6"),
      //   String::from("g6"),
      //   String::from("h6"),
      //   String::from("d5"),
      // ];
      // m.sort();
      // m2.sort();
      // assert_eq!(m, m2);
    }

    #[test]
    fn knight_moves() {
      let game = Game::new();
      let mut m = game.get_all_moves(Position(3, 3), Piece::Knight(Colour::White));
      let mut m2 = vec![
        Position(5, 2),
        Position(5, 4),
        Position(4, 1),
        Position(2, 1),
        Position(2, 5),
        Position(4, 5),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      let mut m = game.get_all_moves(Position(7, 6), Piece::Knight(Colour::Black));
      let mut m2 = vec![Position(5, 7), Position(5, 5)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/1n1B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(4, 3), Piece::Knight(Colour::White));
      let mut m2 = vec![
        Position(5, 1),
        Position(3, 1),
        Position(6, 2),
        Position(6, 4),
        Position(5, 5),
        Position(3, 5),
        Position(2, 4),
        Position(2, 2),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(5, 1), Piece::Knight(Colour::Black));
      let mut m2 = vec![Position(3, 0), Position(3, 2), Position(4, 3)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/3B4/1p1NP2P/6P1/3P1Q2/PnP1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(1, 1), Piece::Knight(Colour::Black));
      let mut m2 = vec![
        Position(3, 0),
        Position(3, 2),
        Position(2, 3),
        Position(0, 3),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }

    #[test]
    fn bishop_moves() {
      let game = Game::new();
      let mut m = game.get_all_moves(Position(0, 2), Piece::Bishop(Colour::White));
      let mut m2 = vec![];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(5, 3), Piece::Bishop(Colour::White));
      let mut m2 = vec![
        Position(4, 2),
        Position(3, 1),
        Position(2, 0),
        Position(6, 2),
        Position(7, 1),
        Position(6, 4),
        Position(7, 5),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(0, 6), Piece::Bishop(Colour::Black));
      let mut m2 = vec![
        Position(1, 5),
        Position(2, 4),
        Position(3, 3),
        Position(4, 2),
        Position(5, 1),
        Position(1, 7),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3PbQ2/P1P1K3/q7 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(2, 4), Piece::Bishop(Colour::Black));
      let mut m2 = vec![
        Position(1, 5),
        Position(0, 6),
        Position(3, 3),
        Position(4, 2),
        Position(5, 1),
        Position(3, 5),
        Position(4, 6),
        Position(5, 7),
        Position(1, 3),
        Position(0, 2),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/b5q1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(0, 0), Piece::Bishop(Colour::Black));
      let mut m2 = [
        Position(1, 1),
        Position(2, 2),
        Position(3, 3),
        Position(4, 4),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }
    #[test]
    fn queen_moves() {
      let game = Game::new();
      let mut m = game.get_all_moves(Position(0, 3), Piece::Queen(Colour::White));
      let mut m2 = vec![];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(2, 5), Piece::Queen(Colour::White));
      let mut m2 = vec![
        Position(2, 4),
        Position(3, 4),
        Position(3, 5),
        Position(4, 5),
        Position(5, 5),
        Position(6, 5),
        Position(2, 6),
        Position(2, 7),
        Position(1, 6),
        Position(0, 7),
        Position(1, 5),
        Position(0, 5),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(0, 0), Piece::Queen(Colour::Black));
      let mut m2 = vec![
        Position(1, 0),
        Position(1, 1),
        Position(2, 2),
        Position(3, 3),
        Position(4, 4),
        Position(0, 1),
        Position(0, 2),
        Position(0, 3),
        Position(0, 4),
        Position(0, 5),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }
    #[test]
    fn king_moves() {
      let game = Game::new();
      let mut m = game.get_all_moves(Position(0, 4), Piece::King(Colour::White));
      let mut m2 = vec![];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(1, 4), Piece::King(Colour::White));
      let mut m2 = vec![
        Position(1, 3),
        Position(2, 4),
        Position(1, 5),
        Position(0, 5),
        Position(0, 4),
        Position(0, 3),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      let mut m = game.get_all_moves(Position(7, 4), Piece::King(Colour::Black));
      let mut m2 = vec![Position(7, 3), Position(7, 5), Position(6, 4)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }

    #[test]
    fn getting_king() {
      let game = Game::new();
      assert_eq!(game.get_king(String::from("white")), Position(0, 4));
      assert_eq!(game.get_king(String::from("black")), Position(7, 4));
      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      assert_eq!(game.get_king(String::from("white")), Position(1, 4));
      assert_eq!(game.get_king(String::from("black")), Position(7, 4));
      let game = Game::new_from_fen(String::from(
        "8/3q1k2/3r4/1p1Pp2p/pP2Pp1P/P4P2/7K/8 b - - 99 50",
      ));
      assert_eq!(game.get_king(String::from("white")), Position(1, 7));
      assert_eq!(game.get_king(String::from("black")), Position(6, 5));
    }
  }

  mod special_rules {
    use crate::Castling;
    use crate::Colour;
    use crate::Game;
    use crate::GameState;
    use crate::Piece;
    #[test]
    fn castling() {
      let game = Game::new();
      let castling = Castling {
        white_queen: true,
        white_king: true,
        black_queen: true,
        black_king: true,
      };
      assert_eq!(game.castling, castling);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let castling = Castling {
        white_queen: false,
        white_king: false,
        black_queen: false,
        black_king: false,
      };
      assert_eq!(game.castling, castling);
      let game = Game::new_from_fen(String::from("4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1"));
      let castling = Castling {
        white_queen: true,
        white_king: false,
        black_queen: false,
        black_king: true,
      };
      assert_eq!(game.castling, castling);
    }

    #[test]
    fn black_in_check() {
      let mut game = Game::new();
      game.make_move(String::from("a2"), String::from("a4"));
      assert_eq!(game.state, GameState::InProgress);

      let mut game = Game::new_from_fen(String::from(
        "rnbqkbnr/ppp2ppp/8/3pp3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 3",
      ));
      game.make_move(String::from("f1"), String::from("b5"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("e8"), String::from("e7"));
      assert_eq!(game.state, GameState::InProgress);
      let mut game = Game::new_from_fen(String::from(
        "rnbq1bnr/ppp2ppp/3k4/1B1P4/5p2/8/PPPPQ1PP/RNB1K1NR w KQ - 2 6",
      ));
      game.make_move(String::from("e2"), String::from("e6"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("d6"), String::from("c5"));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("e6"), String::from("c6"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("c5"), String::from("b4"));
      assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn white_in_check() {
      let mut game = Game::new_from_fen(String::from(
        "rnbq1bnr/ppp2ppp/3k4/1B1P4/5p2/8/PPPP1QPP/RNB1K1NR b KQ - 3 6",
      ));
      game.make_move(String::from("d8"), String::from("e8"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("e1"), String::from("f1"));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("e8"), String::from("e1"));
      assert_eq!(game.state, GameState::Check);
      game.print_board();
      game.make_move(String::from("f1"), String::from("e1"));
      assert_eq!(game.state, GameState::InProgress);

      let mut game = Game::new_from_fen(String::from(
        "rnb1kbnr/ppp2ppp/8/6N1/5B2/3q2p1/PPP4P/RN2KB1R b KQkq - 1 8",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("d3"), String::from("e2"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("f1"), String::from("e2"));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("f8"), String::from("b4"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("e1"), String::from("f1"));
      game.make_move(String::from("b4"), String::from("e1"));
      game.make_move(String::from("f1"), String::from("e1"));
      assert_eq!(game.state, GameState::InProgress);
      let mut game = Game::new_from_fen(String::from(
        "rnbk4/ppp2ppp/2P4n/bP4N1/P7/4B1p1/R3K2P/3r1B1R b - - 2 19",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("d1"), String::from("e1"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("e2"), String::from("d3"));
      assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn black_in_check_by_pawn() {
      let mut game = Game::new_from_fen(String::from(
        "rnbqk1nr/1p3ppp/2pPP3/p7/8/4p3/PP3PPP/RNBQKBNR w KQkq - 0 8",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("d6"), String::from("d7"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("d8"), String::from("d7"));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("e6"), String::from("f7"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("e8"), String::from("f7"));
      assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn white_in_check_by_pawn() {
      let mut game = Game::new_from_fen(String::from(
        "rnbqkbnr/ppp2ppp/2P5/5P2/8/3pp3/PP4PP/RNBQKBNR b KQkq - 0 7",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("d3"), String::from("d2"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("c1"), String::from("d2"));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("e3"), String::from("d2"));
      assert_eq!(game.state, GameState::Check);
      let mut game = Game::new_from_fen(String::from(
        "rnbqkbnr/ppppp2p/4P3/2P5/3P4/5pP1/PP4PP/RNBQKBNR b KQkq - 0 7",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("f3"), String::from("f2"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("e1"), String::from("f2"));
      assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn black_in_check_by_knight() {
      let mut game = Game::new_from_fen(String::from(
        "rnbqkbnr/5ppp/8/pNp1N3/3Pp3/8/PPP2PPP/R1BQKB1R w KQkq - 0 7",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("b5"), String::from("d6"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("e8"), String::from("e7"));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("d6"), String::from("f5"));
      assert_eq!(game.state, GameState::Check);
      let mut game = Game::new_from_fen(String::from(
        "rnbq1Nnr/5ppp/3b1k2/p1p5/PP1P4/4p3/2P2PPP/R1BQKB1R w KQ - 4 13",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("f8"), String::from("h7"));
      assert_eq!(game.state, GameState::Check);
    }

    #[test]
    fn white_in_check_by_knight() {
      let mut game = Game::new_from_fen(String::from(
        "r1bq2nr/4kppN/3b4/pPp5/P1Pn4/4p3/5PPP/R1BQKB1R b KQ - 0 16",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("d4"), String::from("c2"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("d1"), String::from("c2"));
      game.print_board();
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("g8"), String::from("h6"));
      assert_eq!(game.state, GameState::InProgress);
      let mut game = Game::new_from_fen(String::from(
        "r1bq3r/4kppN/3b4/pPp5/P1PQ4/3Bp3/5PPn/R1B1K2R b KQ - 1 20",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("h2"), String::from("f3"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("g2"), String::from("f3"));
      assert_eq!(game.state, GameState::InProgress);
      let mut game = Game::new_from_fen(String::from(
        "rnbqkbnr/pppp2pp/4pp2/8/8/4P3/PPPPKPPP/RNBQ1BNR w kq - 0 3",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("b2"), String::from("b4"));
      game.make_move(String::from("a7"), String::from("a5"));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("b1"), String::from("c3"));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("a4"), String::from("a4"));
      assert_eq!(game.state, GameState::InProgress);
      let mut game = Game::new_from_fen(String::from(
        "r1bqkb1r/Nppp2pp/2n1pp2/8/pP4n1/4P3/P1PPKPPP/R1BQ1BNR b kq - 7 8",
      ));
      assert_eq!(game.state, GameState::InProgress);
      game.make_move(String::from("c6"), String::from("d4"));
      assert_eq!(game.state, GameState::Check);
    }

    #[test]
    fn moves_in_check() {
      let game = Game::new_from_fen(String::from(
        "rnbq1bnr/ppp2ppp/2Q5/1BkP4/5p2/8/PPPP2PP/RNB1K1NR b KQ - 5 7",
      ));
      assert_eq!(game.state, GameState::Check);
      let mut m = game.get_possible_moves(String::from("c5")).unwrap();
      let mut m2 = vec![String::from("b4"), String::from("d4")];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      let game = Game::new_from_fen(String::from(
        "rnNq1bnr/p1pP1ppp/k3Q3/8/8/PP3p2/2PPN1PP/R1B1K2R b KQ - 0 17",
      ));
      assert_eq!(game.state, GameState::Check);
      let mut m = game.get_possible_moves(String::from("a6")).unwrap();
      let mut m2 = vec![String::from("b7"), String::from("a5"), String::from("b5")];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }

    #[test]
    fn cant_move_into_check() {
      let game = Game::new_from_fen(String::from(
        "r1bq2nr/4kppN/3b4/pnpQ4/P1P5/4p3/5PPP/R1B1KB1R b KQ - 1 17",
      ));
      assert_eq!(game.state, GameState::InProgress);
      let mut m = game.get_possible_moves(String::from("e7")).unwrap();
      let mut m2 = vec![String::from("d7"), String::from("e8")];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "Q1b1k1nr/5ppN/8/p1p3q1/P1P5/R2R4/4pPP1/2B1KB2 b - - 2 22",
      ));
      let mut m = game.get_possible_moves(String::from("e8")).unwrap();
      let mut m2 = vec![String::from("e7")];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "Q1b1kN1r/3R1pp1/8/p1p3q1/P1P5/R1n5/4pPP1/2B1KB2 w - - 7 25",
      ));
      let mut m = game.get_possible_moves(String::from("e1")).unwrap();
      let mut m2: Vec<String> = vec![];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }

    #[test]
    fn get_out_of_check() {
      let mut game = Game::new_from_fen(String::from(
        "r1bq2nr/4kppN/3b4/p1pQ4/P1P5/n3p3/5PPP/R1B1KB1R w KQ - 2 18",
      ));
      game.make_move(String::from("d5"), String::from("e5"));
      assert_eq!(game.state, GameState::Check);
      assert_eq!(
        game
          .make_move(String::from("e7"), String::from("e6"))
          .is_none(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("e7"), String::from("e8"))
          .is_none(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("e7"), String::from("f6"))
          .is_none(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("e7"), String::from("f8"))
          .is_none(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("e7"), String::from("d7"))
          .is_some(),
        true
      );

      let mut game = Game::new_from_fen(String::from(
        "r1b1q1nr/3k1ppN/3b4/p1p5/P1n5/B5Q1/5pPP/R2K1B1R b - - 1 22",
      ));
      game.make_move(String::from("e8"), String::from("e1"));
      assert_eq!(game.state, GameState::Check);
      assert_eq!(
        game
          .make_move(String::from("d1"), String::from("c1"))
          .is_none(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("d1"), String::from("d2"))
          .is_none(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("d1"), String::from("e1"))
          .is_none(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("d1"), String::from("e2"))
          .is_none(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("d1"), String::from("c2"))
          .is_some(),
        true
      );
    }

    #[test]
    fn checkmate_works() {
      let mut game = Game::new_from_fen(String::from(
        "1r1qkb1r/p3p2p/p4n1p/2P2P2/P5bP/Np5N/5PP1/R3KR2 b Qk - 1 14",
      ));
      game.make_move(String::from("d8"), String::from("a5"));
      assert_eq!(game.state, GameState::Checkmate);
      let mut game = Game::new_from_fen(String::from(
        "rnb1kbnr/pp2pppp/8/2pp4/2PPP3/8/PP1q1PPP/RNB1KBNR w KQkq - 0 5",
      ));
      assert_eq!(game.state, GameState::Check);
      let mut game = Game::new_from_fen(String::from(
        "rnb2b2/3Bkp2/1Q6/2P1N2r/2P2R1p/2N5/PP3PPP/R2K4 w - - 1 20",
      ));
      game.make_move(String::from("f4"), String::from("f7"));
      assert_eq!(game.state, GameState::Checkmate);
      // checkmate from the start
      let game = Game::new_from_fen(String::from(
        "rnb2b1r/3Bkp2/1Q3p2/2PNN2p/2P1p3/8/PP3PPP/R3K2R b KQ - 1 16",
      ));
      assert_eq!(game.state, GameState::Checkmate);
    }

    #[test]
    fn stalemate_works() {
      let mut game = Game::new_from_fen(String::from(
        "k7/1R1RN3/p3p3/P3P2p/1PP4P/3K1PP1/8/8 b - - 1 2",
      ));
      game.make_move(String::from("a7"), String::from("a8"));
      assert_eq!(game.state, GameState::Stalemate);
    }

    #[test]
    fn promotion_works() {
      let mut game = Game::new_from_fen(String::from(
        "rnbqkbnr/5ppp/8/pNp1N3/3Pp3/8/PPP2PPP/R1BQKB1R w KQkq - 0 7",
      ));
      assert_eq!(game.promote.clone(), (false, String::new()));
      game.set_promotion(game.promote.1.clone(), 'q');

      let mut game = Game::new_from_fen(String::from(
        "r1bqkbnr/1P1ppppp/p1p5/8/1P1n4/8/2PPPPPP/RNBQKBNR w KQkq - 0 6",
      ));
      assert_eq!(
        game.get_piece_at(String::from("b7")).unwrap(),
        Piece::Pawn(Colour::White)
      );
      game.make_move(String::from("b7"), String::from("b8"));
      assert_eq!(game.promote.clone(), (true, String::from("b8")));
      assert_eq!(
        game.get_piece_at(String::from("b8")).unwrap(),
        Piece::Pawn(Colour::White)
      );
      game.set_promotion(game.promote.1.clone(), 'q');
      game.promote = (false, String::new());
      game.print_board();
      assert_eq!(
        game.get_piece_at(String::from("b8")).unwrap(),
        Piece::Queen(Colour::White)
      );
      assert_eq!(game.promote, (false, String::new()));

      let mut game = Game::new_from_fen(String::from(
        "2bqkbnr/1R1ppppp/8/8/3n4/8/p1PPPPPP/1NBQKBNR b Kk - 0 13",
      ));
      game.make_move(String::from("a2"), String::from("b1"));
      assert_eq!(game.promote, (true, String::from("b1")));
      assert_eq!(
        game.get_piece_at(String::from("b1")).unwrap(),
        Piece::Pawn(Colour::Black)
      );
      game.set_promotion(game.promote.1.clone(), 'r');
      game.promote = (false, String::new());
      assert_eq!(
        game.get_piece_at(String::from("b1")).unwrap(),
        Piece::Rook(Colour::Black)
      );
    }
  }
}
