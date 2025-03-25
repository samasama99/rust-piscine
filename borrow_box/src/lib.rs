// Instructions
//
// Game time.
//
// You will implement some CRUD functionality for a game session. You will need to implement the GameSession structure with the following associated functions:
//
// new: which initializes a game session state with player names and some other information. This function returns the structure wrapped in a Box.
//
// read_winner: which returns a tuple with the name and score of the player who is currently winning.
//  In the case that no player is winning, it should return the same tuple with the string "Same score! tied" and the tied score.
//
// update_score: which receives the name of a player, and increments their score.
//      This function should do nothing if the the game session is already finished or if the name received doesn't match any player.
//
// delete: which takes ownership of the boxed game session and returns a string: "game deleted: id -> 0", where 0 is the id of the GameSession.
//
// Examples for nb_games:
//
// When nb_games is 5, the game is best out of 5, and if some player has a score of 3, the game is finished (there aren't enough games for the other player to draw). When nb_games is 11, the game is best out of 11, and if some player has a score of 6, the game is finished (there aren't enough games for the other player to draw).
//
// Expected Functions

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }

    pub fn read_winner(&self) -> (String, u16) {
        // "Same score! tied"
        let score_1 = self.p1.1;
        let score_2 = self.p2.1;
        if score_1 == score_2 {
            ("Same score! tied".to_owned(), score_1)
        } else if score_1 > score_2 {
            self.p1.clone()
        } else {
            self.p2.clone()
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        let score_1 = self.p1.1;
        let score_2 = self.p2.1;

        if score_1 == self.nb_games - 2 || score_2 == self.nb_games - 2 {
            return;
        }

        if self.p1.0 == user_name {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name {
            self.p2.1 += 1;
        }
    }
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
        assert_eq!(("Same score! tied".to_owned(), 0), game.read_winner());
        // output : ("Same score! tied", 0)

        game.update_score(String::from("Joao"));
        game.update_score(String::from("Joao"));
        game.update_score(String::from("Susana"));
        game.update_score(String::from("Susana"));
        assert_eq!(("Same score! tied".to_owned(), 2), game.read_winner());
        // output : ("Same score! tied", 2)

        game.update_score(String::from("Joao"));
        // this one will not count because it already 5 games played, the nb_games
        game.update_score(String::from("Susana"));

        assert_eq!(("Joao".to_owned(), 3), game.read_winner());
        // output : ("Joao", 3)

        assert_eq!("game deleted: id -> 0".to_owned(), game.delete());
        // output : "game deleted: id -> 0"

        // game.read_winner();
        // this will give error
        // because the game was dropped, no longer exists on the heap
    }
}
