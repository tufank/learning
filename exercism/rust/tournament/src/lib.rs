use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Team<'a> {
    name: &'a str,
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}
type TeamMap<'a> = HashMap<&'a str, Team<'a>>;

pub fn tally(match_results: &str) -> String {
    let mut tm: TeamMap = HashMap::new();

    for l in match_results.lines() {
        let v: Vec<&str> = l.split(';').collect();
        let team1 = v[0];
        let team2 = v[1];
        let result = v[2];

        tm.entry(team1).or_insert(Team {
            name: team1,
            mp: 0,
            w: 0,
            d: 0,
            l: 0,
            p: 0,
        });
        tm.entry(team2).or_insert(Team {
            name: team2,
            mp: 0,
            w: 0,
            d: 0,
            l: 0,
            p: 0,
        });

        let mut scores1 = tm[team1].clone();
        let mut scores2 = tm[team2].clone();

        scores1.mp += 1;
        scores2.mp += 1;

        if result == "win" {
            scores1.p += 3;
            scores1.w += 1;
            scores2.l += 1;
        } else if result == "loss" {
            scores2.p += 3;
            scores2.w += 1;
            scores1.l += 1;
        } else if result == "draw" {
            scores1.p += 1;
            scores2.p += 1;
            scores1.d += 1;
            scores2.d += 1;
        }
        tm.insert(team1, scores1);
        tm.insert(team2, scores2);
    }

    println!("================== {:?}", tm);
    order(tm);

    String::from("")
}

fn order(tm: TeamMap) {
    let mut ot: Vec<Team> = vec![];

    for (_, v) in tm.iter() {
        ot.push(v.clone())
    }

    for i in ot.iter() {}
}
