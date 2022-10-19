#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentResponse {
    Cooperate,
    Cheat,
}

pub trait Agent {
    fn play(&self) -> AgentResponse;
    fn respond(&mut self, other: AgentResponse);
}

pub struct Game {
    left: Box<dyn Agent>,
    right: Box<dyn Agent>,
    left_score: i32,
    right_score: i32,
}

use AgentResponse::*;
use RoundOutcome::*;
impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Game {
            left_score: 0,
            right_score: 0,
            left,
            right,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let l = self.left.play();
        let r = self.right.play();
        let (res, left_score, right_score) = match (l, r) {
            (Cooperate, Cooperate) => (BothCooperated, 2, 2),
            (Cooperate, Cheat) => (RightCheated, -1, 3),
            (Cheat, Cooperate) => (LeftCheated, 3, -1),
            (Cheat, Cheat) => (BothCheated, 0, 0),
        };
        self.left_score += left_score;
        self.right_score += right_score;
        self.left.respond(r);
        self.right.respond(l);
        res
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

impl Agent for CheatingAgent {
    fn play(&self) -> AgentResponse {
        AgentResponse::Cheat
    }

    fn respond(&mut self, _: AgentResponse) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Agent for CooperatingAgent {
    fn play(&self) -> AgentResponse {
        AgentResponse::Cooperate
    }

    fn respond(&mut self, _: AgentResponse) {}
}

////////////////////////////////////////////////////////////////////////////////

pub struct GrudgerAgent {
    answer: AgentResponse,
}

impl Default for GrudgerAgent {
    fn default() -> Self {
        Self {
            answer: AgentResponse::Cooperate,
        }
    }
}

impl Agent for GrudgerAgent {
    fn play(&self) -> AgentResponse {
        self.answer
    }

    fn respond(&mut self, other: AgentResponse) {
        if other == AgentResponse::Cheat {
            self.answer = other
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {
    answer: AgentResponse,
}

impl Default for CopycatAgent {
    fn default() -> Self {
        Self {
            answer: AgentResponse::Cooperate,
        }
    }
}

impl Agent for CopycatAgent {
    fn play(&self) -> AgentResponse {
        self.answer
    }

    fn respond(&mut self, other: AgentResponse) {
        self.answer = other;
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    initial: u8,
    copycat_mode: bool,
    last_response: AgentResponse,
}
impl Default for DetectiveAgent {
    fn default() -> Self {
        Self {
            initial: 0,
            copycat_mode: false,
            last_response: AgentResponse::Cooperate,
        }
    }
}

impl Agent for DetectiveAgent {
    fn play(&self) -> AgentResponse {
        match self.initial {
            4.. => {
                if self.copycat_mode {
                    self.last_response
                } else {
                    Cheat
                }
            }
            1 => Cheat,
            _ => Cooperate,
        }
    }

    fn respond(&mut self, other: AgentResponse) {
        self.last_response = other;
        if self.initial <= 3 {
            self.copycat_mode = self.copycat_mode || self.last_response == Cheat;
            self.initial += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CheatingAgent, CooperatingAgent, CopycatAgent, DetectiveAgent, Game, GrudgerAgent,
        RoundOutcome,
    };

    fn test_game<'a>(
        mut game: Game,
        expected_outcomes: impl IntoIterator<Item = &'a RoundOutcome>,
    ) {
        let mut left_score = 0;
        let mut right_score = 0;

        for (i, expected) in expected_outcomes.into_iter().enumerate() {
            let outcome = game.play_round();
            let expected = expected.to_owned();
            assert_eq!(
                expected,
                outcome,
                "move #{}: expected {:?}, got {:?}",
                i + 1,
                expected,
                outcome,
            );

            match outcome {
                RoundOutcome::BothCooperated => {
                    left_score += 2;
                    right_score += 2;
                }
                RoundOutcome::LeftCheated => {
                    left_score += 3;
                    right_score -= 1;
                }
                RoundOutcome::RightCheated => {
                    left_score -= 1;
                    right_score += 3;
                }
                RoundOutcome::BothCheated => (),
            }

            assert_eq!(left_score, game.left_score());
            assert_eq!(right_score, game.right_score());
        }
    }

    #[test]
    fn cooperators() {
        let game = Game::new(
            Box::new(CooperatingAgent::default()),
            Box::new(CooperatingAgent::default()),
        );
        test_game(game, &[RoundOutcome::BothCooperated; 12]);
    }

    #[test]
    fn cheaters() {
        let game = Game::new(
            Box::new(CheatingAgent::default()),
            Box::new(CheatingAgent::default()),
        );
        test_game(game, &[RoundOutcome::BothCheated; 8]);
    }

    #[test]
    fn grudgers() {
        let game = Game::new(
            Box::new(GrudgerAgent::default()),
            Box::new(GrudgerAgent::default()),
        );
        test_game(game, &[RoundOutcome::BothCooperated; 15]);
    }

    #[test]
    fn copycats() {
        let game = Game::new(
            Box::new(CopycatAgent::default()),
            Box::new(CopycatAgent::default()),
        );
        test_game(game, &[RoundOutcome::BothCooperated; 14]);
    }

    #[test]
    fn detectives() {
        let game = Game::new(
            Box::new(DetectiveAgent::default()),
            Box::new(DetectiveAgent::default()),
        );
        test_game(
            game,
            [RoundOutcome::BothCooperated; 1]
                .iter()
                .chain([RoundOutcome::BothCheated; 1].iter())
                .chain([RoundOutcome::BothCooperated; 12].iter()),
        );
    }

    #[test]
    fn cooperator_cheater() {
        let game = Game::new(
            Box::new(CooperatingAgent::default()),
            Box::new(CheatingAgent::default()),
        );
        test_game(game, &[RoundOutcome::RightCheated; 18]);
    }

    #[test]
    fn cooperator_grudger() {
        let game = Game::new(
            Box::new(CooperatingAgent::default()),
            Box::new(GrudgerAgent::default()),
        );
        test_game(game, &[RoundOutcome::BothCooperated; 16]);
    }

    #[test]
    fn cooperator_copycat() {
        let game = Game::new(
            Box::new(CooperatingAgent::default()),
            Box::new(CopycatAgent::default()),
        );
        test_game(game, &[RoundOutcome::BothCooperated; 11]);
    }

    #[test]
    fn cooperator_detective() {
        let game = Game::new(
            Box::new(CooperatingAgent::default()),
            Box::new(DetectiveAgent::default()),
        );
        test_game(
            game,
            [RoundOutcome::BothCooperated; 1]
                .iter()
                .chain([RoundOutcome::RightCheated; 1].iter())
                .chain([RoundOutcome::BothCooperated; 2].iter())
                .chain([RoundOutcome::RightCheated; 8].iter()),
        );
    }

    #[test]
    fn cheater_grudger() {
        let game = Game::new(
            Box::new(CheatingAgent::default()),
            Box::new(GrudgerAgent::default()),
        );
        test_game(
            game,
            [RoundOutcome::LeftCheated; 1]
                .iter()
                .chain([RoundOutcome::BothCheated; 10].iter()),
        );
    }

    #[test]
    fn cheater_copycat() {
        let game = Game::new(
            Box::new(CheatingAgent::default()),
            Box::new(CopycatAgent::default()),
        );
        test_game(
            game,
            [RoundOutcome::LeftCheated; 1]
                .iter()
                .chain([RoundOutcome::BothCheated; 7].iter()),
        );
    }

    #[test]
    fn cheater_detective() {
        let game = Game::new(
            Box::new(CheatingAgent::default()),
            Box::new(DetectiveAgent::default()),
        );
        test_game(
            game,
            [RoundOutcome::LeftCheated; 1]
                .iter()
                .chain([RoundOutcome::BothCheated; 1].iter())
                .chain([RoundOutcome::LeftCheated; 2].iter())
                .chain([RoundOutcome::BothCheated; 8].iter()),
        );
    }

    #[test]
    fn grudger_copycat() {
        let game = Game::new(
            Box::new(GrudgerAgent::default()),
            Box::new(CopycatAgent::default()),
        );
        test_game(game, &[RoundOutcome::BothCooperated; 17]);
    }

    #[test]
    fn grudger_detective() {
        let game = Game::new(
            Box::new(GrudgerAgent::default()),
            Box::new(DetectiveAgent::default()),
        );
        test_game(
            game,
            [RoundOutcome::BothCooperated; 1]
                .iter()
                .chain([RoundOutcome::RightCheated; 1].iter())
                .chain([RoundOutcome::LeftCheated; 2].iter())
                .chain([RoundOutcome::BothCheated; 8].iter()),
        );
    }

    #[test]
    fn copycat_detective() {
        let game = Game::new(
            Box::new(CopycatAgent::default()),
            Box::new(DetectiveAgent::default()),
        );
        test_game(
            game,
            [RoundOutcome::BothCooperated; 1]
                .iter()
                .chain([RoundOutcome::RightCheated; 1].iter())
                .chain([RoundOutcome::LeftCheated; 1].iter())
                .chain([RoundOutcome::BothCooperated; 11].iter()),
        );
    }
}
