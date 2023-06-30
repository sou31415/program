use rand::Rng;
use std::fmt;
use std::fmt::Write;

const H: usize = 3; // 迷路の高さ
const W: usize = 3; // 迷路の幅
const END_TURN: i32 = 4; // ゲーム終了ターン

#[derive(Clone, Copy, PartialEq)]
enum WinningStatus {
    WIN,
    LOSE,
    DRAW,
    NONE,
}

struct AlternateMazeState {
    points: [[i32; W]; H], // 床のポイントを1~9で表現する
    turn: i32,             // 現在のターン
    characters: [Character; 2],
}

struct Character {
    y: usize,
    x: usize,
    game_score: i32,
}

impl AlternateMazeState {
    fn new(seed: u64) -> Self {
        let mut rng: u64 = rand::SeedableRng::seed_from_u64(seed);
        let mut points = [[0; W]; H];
        let characters = [
            Character {
                y: H / 2,
                x: (W / 2) - 1,
                game_score: 0,
            },
            Character {
                y: H / 2,
                x: (W / 2) + 1,
                game_score: 0,
            },
        ];

        for y in 0..H {
            for x in 0..W {
                let point: u64 = rng.gen_range(0..=9);
                if characters[0].y == y && characters[0].x == x {
                    continue;
                }
                if characters[1].y == y && characters[1].x == x {
                    continue;
                }

                points[y][x] = point;
            }
        }

        AlternateMazeState {
            points,
            turn: 0,
            characters,
        }
    }

    fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    fn advance(&mut self, action: usize) {
        let character = &mut self.characters[0];
        character.x = (character.x as isize + dx[action]) as usize;
        character.y = (character.y as isize + dy[action]) as usize;
        let point = &mut self.points[character.y][character.x];
        if *point > 0 {
            character.game_score += *point;
            *point = 0;
        }
        self.turn += 1;
        self.characters.swap(0, 1);
    }

    fn legal_actions(&self) -> Vec<usize> {
        let character = &self.characters[0];
        let mut actions = Vec::new();
        for action in 0..4 {
            let ty = character.y as isize + dy[action];
            let tx = character.x as isize + dx[action];
            if ty >= 0 && ty < H as isize && tx >= 0 && tx < W as isize {
                actions.push(action);
            }
        }
        actions
    }

    fn get_winning_status(&self) -> WinningStatus {
        if self.is_done() {
            if self.characters[0].game_score > self.characters[1].game_score {
                WinningStatus::WIN
            } else if self.characters[0].game_score < self.characters[1].game_score {
                WinningStatus::LOSE
            } else {
                WinningStatus::DRAW
            }
        } else {
            WinningStatus::NONE
        }
    }
}

impl fmt::Display for AlternateMazeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "turn:\t{}\n", self.turn)?;
        for (player_id, character) in self.characters.iter().enumerate() {
            let actual_player_id = if self.turn % 2 == 1 {
                (player_id + 1) % 2
            } else {
                player_id
            };
            write!(
                f,
                "score({}):\t{}\ty: {} x: {}\n",
                player_id, character.game_score, character.y, character.x
            )?;
        }
        for h in 0..H {
            for w in 0..W {
                let mut is_written = false;
                for (player_id, character) in self.characters.iter().enumerate() {
                    let actual_player_id = if self.turn % 2 == 1 {
                        (player_id + 1) % 2
                    } else {
                        player_id
                    };

                    if character.y == h && character.x == w {
                        let c = if actual_player_id == 0 { 'A' } else { 'B' };
                        write!(f, "{}", c)?;
                        is_written = true;
                    }
                }
                if !is_written {
                    let point = self.points[h][w];
                    if point > 0 {
                        write!(f, "{}", point)?;
                    } else {
                        write!(f, ".")?;
                    }
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

const dx: [isize; 4] = [1, -1, 0, 0];
const dy: [isize; 4] = [0, 0, 1, -1];

fn random_action(state: &AlternateMazeState) -> usize {
    let mut rng = rand::thread_rng();
    let legal_actions = state.legal_actions();
    legal_actions[rng.gen_range(0..legal_actions.len())]
}

fn play_game(seed: u64) {
    let mut state = AlternateMazeState::new(seed);
    println!("{}", state);
    while !state.is_done() {
        // 1p
        {
            println!("1p ------------------------------------");
            let action = random_action(&state);
            println!("action {}", action);
            state.advance(action);
            println!("{}", state);
            if state.is_done() {
                match state.get_winning_status() {
                    WinningStatus::WIN => println!("winner: 2p"),
                    WinningStatus::LOSE => println!("winner: 1p"),
                    _ => println!("DRAW"),
                }
                break;
            }
        }
        // 2p
        {
            println!("2p ------------------------------------");
            let action = random_action(&state);
            println!("action {}", action);
            state.advance(action);
            println!("{}", state);
            if state.is_done() {
                match state.get_winning_status() {
                    WinningStatus::WIN => println!("winner: 1p"),
                    WinningStatus::LOSE => println!("winner: 2p"),
                    _ => println!("DRAW"),
                }
                break;
            }
        }
    }
}

fn main() {
    play_game(4121859904);
}
