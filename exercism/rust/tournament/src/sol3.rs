use std::collections::HashMap;

struct Game<'a> {
    team1: &'a str,

    team2: &'a str,

    result: &'a str,
}

struct Table {
    statistic: HashMap<String, Statistic>,
}

impl ToString for Table {
    fn to_string(&self) -> String {
        format!("{}\n{}", self.header(), self.body())
    }
}

impl Table {
    fn new() -> Table {
        Table {
            statistic: HashMap::new(),
        }
    }

    fn scores(&mut self, game: &Game) {
        let team1 = self
            .statistic
            .entry(game.team1.to_string())
            .or_insert(Statistic::new());

        let team2 = self
            .statistic
            .entry(game.team2.to_string())
            .or_insert(Statistic::new());

        match game.result {
            "won" => {
                team1.won += 1;

                team2.lost += 1;
            }

            "lost" => {
                team1.lost += 1;

                team2.won += 1;
            }

            "drawn" => {
                team1.drawn += 1;

                team2.drawn += 1;
            }

            _ => panic!("Wrong game result"),
        }
    }

    fn header(&self) -> String {
        format!(
            "{:30} | {} | {} | {} | {} | {}",
            "Team", "MP", "W", "D", "L", "P"
        )
    }

    fn body(&self) -> String {
        self.statistic
            .iter()
            .map(|(team, statistic)| {
                format!(
                    "{:30} | {} | {} | {} | {} | {}",
                    team,
                    statistic.played(),
                    statistic.won,
                    statistic.drawn,
                    statistic.lost,
                    statistic.points()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

struct Statistic {
    won: usize,

    drawn: usize,

    lost: usize,
}

impl Statistic {
    pub fn new() -> Statistic {
        Statistic {
            won: 0,

            drawn: 0,

            lost: 0,
        }
    }

    pub fn played(&self) -> usize {
        self.won + self.lost + self.drawn
    }

    pub fn points(&self) -> usize {
        3 * self.won + self.drawn
    }
}

impl<'a> Game<'a> {
    fn from(string: &str) -> Option<Game> {
        let v: Vec<&str> = string.split(';').collect();

        match v.len() {
            3 => Some(Game {
                team1: v[0],

                team2: v[1],

                result: v[2],
            }),

            _ => None,
        }
    }
}

pub fn tally(input: &str) -> String {
    let mut table = Table::new();

    for line in input.split("\n") {
        match Game::from(line) {
            Some(game) => table.scores(&game),

            None => println!("Wrong game found..."),
        }
    }

    table.to_string()
}
