use std::collections::HashSet;

use anyhow::bail;
use itertools::Itertools;
use tracing::debug;

const ADJ_SIZE: usize = 26 * 26;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Clique {
    verticies: Vec<usize>,
}

impl Clique {
    fn grow_with(&mut self, candidate: usize, adjacency: &[Vec<usize>]) -> Option<Self> {
        if self.verticies.iter().all(|v| adjacency[candidate][*v] == 1) {
            let mut next_verts = self.verticies.clone();
            next_verts.insert(next_verts.binary_search(&candidate).unwrap_err(), candidate);
            Some(Self {
                verticies: next_verts,
            })
        } else {
            None
        }
    }
    fn from_tuple(tuple: (usize, usize)) -> Self {
        Self {
            verticies: vec![tuple.0.min(tuple.1), tuple.0.max(tuple.1)],
        }
    }
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.verticies.len()
    }
    fn format_members(&self) -> String {
        self.verticies.iter().map(index_to_str).sorted().join(",")
    }
    fn contains_vertex(&self, vertex: usize) -> bool {
        self.verticies.contains(&vertex)
    }
}

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<usize> {
    let (adjacency, verticies) = construct_adjacency(&input);
    let t_pairs = t_desktops();
    let mut t_cliques: HashSet<Clique> = clique_pairs(&input)
        .into_iter()
        .filter(|c| t_pairs.iter().any(|&vertex| c.contains_vertex(vertex)))
        .collect();
    t_cliques = grow_cliques(t_cliques, &adjacency, &verticies);
    Ok(t_cliques.len())
}

fn part2(input: Vec<String>) -> anyhow::Result<String> {
    let (adjacency, verticies) = construct_adjacency(&input);
    let mut cliques = clique_pairs(&input);
    let mut clique_size = 3;
    while cliques.len() > 1 {
        cliques = grow_cliques(cliques, &adjacency, &verticies);
        debug!("There are {} cliques of size {clique_size}", cliques.len());
        clique_size += 1;
    }
    if let Some(clique) = cliques.into_iter().next() {
        Ok(clique.format_members())
    } else {
        bail!("No unique maximum clique found")
    }
}

fn grow_cliques(
    cliques: HashSet<Clique>,
    adjacency: &[Vec<usize>],
    verticies: &[usize],
) -> HashSet<Clique> {
    let mut new_cliques = HashSet::new();
    for mut clique in cliques {
        for &vertex in verticies {
            if let Some(new_clique) = clique.grow_with(vertex, adjacency) {
                new_cliques.insert(new_clique);
            }
        }
    }
    new_cliques
}

fn clique_pairs(input: &[String]) -> HashSet<Clique> {
    input
        .iter()
        .map(|line| line.split('-').map(str_to_index).next_tuple().unwrap())
        .map(Clique::from_tuple)
        .collect()
}

fn index_to_str(i: &usize) -> String {
    let first_char = (((i / 26) as u8) + b'a') as char;
    let second_char = (((i % 26) as u8) + b'a') as char;
    format!("{first_char}{second_char}")
}

fn str_to_index(s: &str) -> usize {
    let sb = s.as_bytes();
    ((sb[0] - b'a') as usize * 26) + ((sb[1] - b'a') as usize)
}

fn construct_adjacency(input: &[String]) -> (Vec<Vec<usize>>, Vec<usize>) {
    let mut adjacency = vec![vec![0; ADJ_SIZE]; ADJ_SIZE];
    let mut verticies = HashSet::new();
    for (a, b) in input
        .iter()
        .map(|line| line.split('-').map(str_to_index).next_tuple().unwrap())
    {
        verticies.insert(a);
        verticies.insert(b);
        adjacency[a][b] = 1;
        adjacency[b][a] = 1;
    }
    (adjacency, verticies.into_iter().collect_vec())
}

fn t_desktops() -> Vec<usize> {
    [
        "ta", "tb", "tc", "td", "te", "tf", "tg", "th", "ti", "tj", "tk", "tl", "tm", "tn", "to",
        "tp", "tq", "tr", "ts", "tt", "tu", "tv", "tw", "tx", "ty", "tz",
    ]
    .into_iter()
    .map(str_to_index)
    .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = [7];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part1(input).unwrap(), output);
        }
    }

    #[test]
    fn test_part2() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = ["co,de,ka,ta"];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part2(input).unwrap(), output);
        }
    }
}
