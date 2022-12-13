use nom::{bytes::complete::tag, character::complete::alpha1, IResult};
use petgraph::graph::{DefaultIx, NodeIndex, UnGraph as Graph};
use qu::ick_use::*;
use std::{collections::HashMap, fmt};

const INPUT: &str = include_str!("../input/12");

pub fn first() -> Result<u32> {
    let input = Input::parse()?;
    todo!()
}

pub fn second() -> Result<u32> {
    let input = Input::parse()?;
    todo!()
}

struct Input {
    graph: Graph<Cave<'static>, ()>,
}

impl Input {
    fn parse() -> Result<Self> {
        let mut graph = Graph::new_undirected();
        let mut caves = HashMap::new();
        for line in INPUT.lines() {
            let (_, (cave1, cave2)) = parse_line(line).unwrap();
            let cave1 = *caves.entry(cave1).or_insert_with(|| graph.add_node(cave1));
            let cave2 = *caves.entry(cave2).or_insert_with(|| graph.add_node(cave2));
            graph.add_edge(cave1, cave2, ());
        }
        Ok(Self { graph })
    }

    fn print(&self) {
        println!(
            "{:?}",
            petgraph::dot::Dot::with_config(&self.graph, &[petgraph::dot::Config::EdgeNoLabel])
        );
    }
}

fn parse_line(i: &str) -> IResult<&str, (Cave<'_>, Cave<'_>)> {
    let (i, first) = Cave::parse(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, second) = Cave::parse(i)?;
    Ok((i, (first, second)))
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum Cave<'input> {
    Start,
    End,
    Named(&'input str),
}

impl<'input> Cave<'input> {
    fn parse(i: &'input str) -> IResult<&'input str, Self> {
        let (i, name) = alpha1(i)?;
        Ok((i, Self::from_str(name)))
    }

    fn from_str(i: &'input str) -> Self {
        match i {
            "start" => Cave::Start,
            "end" => Cave::End,
            other => Cave::Named(other),
        }
    }

    fn is_large(&self) -> bool {
        match self {
            Self::Named(name) => name.chars().next().unwrap().is_uppercase(),
            _ => false,
        }
    }
}

impl fmt::Debug for Cave<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cave::Start => f.write_str("start"),
            Cave::End => f.write_str("end"),
            Cave::Named(name) => f.write_str(name),
        }
    }
}
