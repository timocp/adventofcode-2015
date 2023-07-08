use crate::Part;
use std::collections::HashSet;

pub fn run(input: &str, part: Part) -> String {
    let (replacements, molecule) = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&replacements, &molecule),
            Part::Two => 0,
        }
    )
}

fn part1(replacements: &Vec<Replacement>, molecule: &str) -> usize {
    let mut molecules = HashSet::new();
    for replacement in replacements {
        for (pos, _) in molecule.match_indices(&replacement.from) {
            let new_molecule = format!(
                "{}{}{}",
                &molecule[..pos],
                replacement.to,
                &molecule[(pos + replacement.from.len())..]
            );
            molecules.insert(new_molecule);
        }
    }
    molecules.len()
}

#[derive(Debug)]
struct Replacement {
    from: String,
    to: String,
}

fn parse_input(input: &str) -> (Vec<Replacement>, String) {
    let mut lines = input.lines();
    let mut replacements = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" => ");
        replacements.push(Replacement {
            from: parts.next().unwrap().to_string(),
            to: parts.next().unwrap().to_string(),
        });
    }
    let molecule = lines.next().unwrap().to_string();
    (replacements, molecule)
}

#[test]
fn test_parse_input() {
    let test_input = "H => HO\nH => OH\nO => HH\n\nHOH\n";
    let (replacements, molecule) = parse_input(test_input);
    assert_eq!(replacements.len(), 3);
    assert_eq!(molecule, "HOH");
    assert_eq!(replacements[0].from, "H");
    assert_eq!(replacements[0].to, "HO");
    assert_eq!(replacements[1].from, "H");
    assert_eq!(replacements[1].to, "OH");
    assert_eq!(replacements[2].from, "O");
    assert_eq!(replacements[2].to, "HH");
}

#[test]
fn test_part1() {
    let test_input = "H => HO\nH => OH\nO => HH\n\nHOH\n";
    let (replacements, molecule) = parse_input(test_input);
    assert_eq!(4, part1(&replacements, &molecule));

    let test_input = "H => HO\nH => OH\nO => HH\n\nHOHOHO\n";
    let (replacements, molecule) = parse_input(test_input);
    assert_eq!(7, part1(&replacements, &molecule));
}
