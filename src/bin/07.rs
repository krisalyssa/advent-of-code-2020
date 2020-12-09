use common::{Day, Part};
use petgraph::algo::bellman_ford;
use petgraph::graphmap::GraphMap;
use petgraph::prelude::*;
use regex::Regex;

pub fn main() {
  let mut data: Vec<String> = vec![];

  if common::load_data("data/day-07-input.txt", &mut data).is_ok() {
    let part_1 = Part::new(part_1);
    let part_2 = Part::new(part_2);

    let mut day = Day::new(part_1, part_2);

    day.run(&data);

    assert_eq!(287, day.part_1.result);
    assert_eq!(48160, day.part_2.result);

    println!("{}", day.to_string());
  } else {
    eprintln!("cannot open data/day-07-input.txt");
    std::process::exit(1);
  }
}

pub fn part_1(data: &[&str]) -> u64 {
  let graph = build_graph(&data, false);

  if let Ok((paths, _)) = bellman_ford(&graph, "shiny gold") {
    paths.iter().filter(|w| w.is_normal()).count() as u64
  } else {
    panic!("couldn't find paths to 'shiny gold'");
  }
}

pub fn part_2(data: &[&str]) -> u64 {
  let graph = build_graph(&data, true);

  // we have to subtract one here because count_contents() counts the outer bag as well
  unsafe { (count_contents(&graph, "shiny gold") - 1.0).to_int_unchecked::<u64>() }
}

fn build_dependencies<'a>(rule: &'a &'a str) -> (&'a str, Vec<(u8, &'a str)>) {
  let parts = split_rule(*rule);
  let outer: &str = parts.get(0).unwrap();
  let inner: Vec<(u8, &str)> = split_contents(parts.get(1).unwrap())
    .iter()
    .map(|bag| extract_bag_descriptor(bag))
    .filter(std::option::Option::is_some)
    .map(std::option::Option::unwrap)
    .collect();
  (outer, inner)
}

fn build_graph<'a>(data: &'a [&'a str], top_down: bool) -> GraphMap<&'a str, f32, Directed> {
  let mut graph = GraphMap::<&str, f32, Directed>::new();
  let dependencies: Vec<(&str, Vec<(u8, &str)>)> = data.iter().map(build_dependencies).collect();
  for (bag, contents) in dependencies {
    graph.add_node(bag);
    for (quantity, descriptor) in contents {
      graph.add_node(descriptor);
      if top_down {
        graph.add_edge(bag, descriptor, quantity.into());
      } else {
        graph.add_edge(descriptor, bag, quantity.into());
      }
    }
  }

  graph
}

fn count_contents(graph: &GraphMap<&str, f32, Directed>, outer: &str) -> f32 {
  graph
    .neighbors_directed(outer, Outgoing)
    .map(|inner| (count_contents(graph, inner) * *graph.edge_weight(outer, inner).unwrap_or(&0.0)))
    .sum::<f32>()
    + 1.0
}

fn extract_bag_descriptor(bag: &str) -> Option<(u8, &str)> {
  let regex = Regex::new(r"^(\d+)\s+(.+)\s+bags?.?$").unwrap();
  if let Some(captures) = regex.captures(bag) {
    let quantity: u8 = captures.get(1).unwrap().as_str().parse().ok().unwrap();
    let descriptor: &str = captures.get(2).unwrap().as_str();
    Some((quantity, descriptor))
  } else {
    None
  }
}

fn split_contents(contents: &str) -> Vec<&str> {
  if contents == "no other bags" {
    vec![]
  } else {
    contents.split(", ").map(str::trim).collect()
  }
}

fn split_rule(rule: &str) -> Vec<&str> {
  rule.splitn(2, " bags contain ").collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let data = vec![
      "light red bags contain 1 bright white bag, 2 muted yellow bags.",
      "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
      "bright white bags contain 1 shiny gold bag.",
      "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
      "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
      "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
      "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
      "faded blue bags contain no other bags.",
      "dotted black bags contain no other bags.",
    ];
    assert_eq!(part_1(&data), 4);
  }

  #[test]
  fn test_part_2() {
    let data = vec![
      "light red bags contain 1 bright white bag, 2 muted yellow bags.",
      "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
      "bright white bags contain 1 shiny gold bag.",
      "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
      "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
      "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
      "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
      "faded blue bags contain no other bags.",
      "dotted black bags contain no other bags.",
    ];
    assert_eq!(part_2(&data), 32);
  }

  #[test]
  fn test_build_dependencies() {
    assert_eq!(
      build_dependencies(&"light red bags contain 1 bright white bag, 2 muted yellow bags."),
      ("light red", vec![(1, "bright white"), (2, "muted yellow")])
    );
  }

  #[test]
  fn test_build_graph() {
    let data = vec![
      "light red bags contain 1 bright white bag, 2 muted yellow bags.",
      "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
      "bright white bags contain 1 shiny gold bag.",
      "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
      "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
      "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
      "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
      "faded blue bags contain no other bags.",
      "dotted black bags contain no other bags.",
    ];
    let top_down = build_graph(&data, true);
    assert!(top_down.contains_edge("bright white", "shiny gold"));
    assert!(top_down.contains_edge("muted yellow", "shiny gold"));
    assert!(top_down.contains_edge("dark orange", "bright white"));
    assert!(top_down.contains_edge("dark orange", "muted yellow"));

    let bottom_up = build_graph(&data, false);
    assert!(bottom_up.contains_edge("shiny gold", "bright white"));
    assert!(bottom_up.contains_edge("shiny gold", "muted yellow"));
    assert!(bottom_up.contains_edge("bright white", "dark orange"));
    assert!(bottom_up.contains_edge("muted yellow", "dark orange"));
  }

  #[test]
  fn test_count_contents() {
    {
      let data = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
      ];
      let graph = build_graph(&data, true);
      assert_eq!(count_contents(&graph, "faded blue"), 1.0);
      assert_eq!(count_contents(&graph, "dotted black"), 1.0);
      assert_eq!(count_contents(&graph, "vibrant plum"), 12.0);
      assert_eq!(count_contents(&graph, "shiny gold"), 33.0)
    }
    {
      let data = vec![
        "shiny gold bags contain 2 dark red bags.",
        "dark red bags contain 2 dark orange bags.",
        "dark orange bags contain 2 dark yellow bags.",
        "dark yellow bags contain 2 dark green bags.",
        "dark green bags contain 2 dark blue bags.",
        "dark blue bags contain 2 dark violet bags.",
        "dark violet bags contain no other bags.",
      ];
      let graph = build_graph(&data, true);
      assert_eq!(count_contents(&graph, "shiny gold"), 127.0)
    }
  }

  #[test]
  fn test_extract_bag_descriptor() {
    assert_eq!(
      extract_bag_descriptor("1 bright white bag").unwrap(),
      (1, "bright white")
    );
    assert_eq!(
      extract_bag_descriptor("2 muted yellow bags.").unwrap(),
      (2, "muted yellow")
    );
  }

  #[test]
  fn test_split_contents() {
    assert_eq!(
      split_contents("1 bright white bag, 2 muted yellow bags."),
      vec!["1 bright white bag", "2 muted yellow bags."]
    );
  }

  #[test]
  fn test_split_rule() {
    assert_eq!(
      split_rule("light red bags contain 1 bright white bag, 2 muted yellow bags."),
      vec!["light red", "1 bright white bag, 2 muted yellow bags."]
    );
    assert_eq!(
      split_rule("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
      vec!["dark orange", "3 bright white bags, 4 muted yellow bags."]
    );
    assert_eq!(
      split_rule("bright white bags contain 1 shiny gold bag."),
      vec!["bright white", "1 shiny gold bag."]
    );
    assert_eq!(
      split_rule("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
      vec!["muted yellow", "2 shiny gold bags, 9 faded blue bags."]
    );
    assert_eq!(
      split_rule("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
      vec!["shiny gold", "1 dark olive bag, 2 vibrant plum bags."]
    );
    assert_eq!(
      split_rule("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
      vec!["dark olive", "3 faded blue bags, 4 dotted black bags."]
    );
    assert_eq!(
      split_rule("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
      vec!["vibrant plum", "5 faded blue bags, 6 dotted black bags."]
    );
    assert_eq!(
      split_rule("faded blue bags contain no other bags."),
      vec!["faded blue", "no other bags."]
    );
    assert_eq!(
      split_rule("dotted black bags contain no other bags."),
      vec!["dotted black", "no other bags."]
    );
  }
}
