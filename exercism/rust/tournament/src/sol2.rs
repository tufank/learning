use std::collections::HashMap;

struct TeamResults {
    pub name: String,
    pub play_count: u32,
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
    pub points: u32,
}

impl TeamResults {
    pub fn new(name: &str) -> Self {
        TeamResults {
            name: String::from(name),
            play_count: 0,
            wins: 0,
            losses: 0,
            draws: 0,
            points: 0,
        }
    }

    pub fn win(&mut self) {
        self.wins = self.wins + 1;
        self.points = self.points + 3;
        self.play_count = self.play_count + 1;
    }

    pub fn loss(&mut self) {
        self.losses = self.losses + 1;
        self.play_count = self.play_count + 1;
    }

    pub fn draw(&mut self) {
        self.draws = self.draws + 1;
        self.points = self.points + 1;
        self.play_count = self.play_count + 1;
    }
}

pub fn tally(input: &str) -> String {
    let mut team_dict: HashMap<String, TeamResults> = HashMap::new();

    for line in input.lines() {
        let tokens = line.split(';').collect::<Vec<&str>>();

        if tokens.len() != 3 {
            continue;
        }

        let result = tokens[2];

        let outcome = match result {
            "win" => (1, -1),

            "loss" => (-1, 1),

            "draw" => (0, 0),

            _ => continue,
        };

        record(&mut team_dict, String::from(tokens[0]), outcome.0);
        record(&mut team_dict, String::from(tokens[1]), outcome.1);
    }

    let mut tally = team_dict.values().collect::<Vec<&TeamResults>>();

    tally.sort_by(|a, b| {
        if a.points == b.points {
            a.name.partial_cmp(&b.name).unwrap()
        } else {
            b.points.partial_cmp(&a.points).unwrap()
        }
    });

    let mut output = table_line("Team", "MP", "W", "D", "L", "P");

    for team in tally {
        output = output
            + "\n"
            + &*table_line(
                &*team.name,
                &*team.play_count.to_string(),
                &*team.wins.to_string(),
                &*team.draws.to_string(),
                &*team.losses.to_string(),
                &*team.points.to_string(),
            );
    }

    output
}

fn table_line(title: &str, mp: &str, w: &str, d: &str, l: &str, points: &str) -> String {
    format!(
        "{:31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        title, mp, w, d, l, points
    )
}

fn record(team_dict: &mut HashMap<String, TeamResults>, name: String, outcome: i8) {
    let mut team = team_dict
        .entry(name.clone())
        .or_insert(TeamResults::new(&*name));

    match outcome {
        1 => team.win(),
        -1 => team.loss(),
        0 => team.draw(),
        _ => {}
    }
}
