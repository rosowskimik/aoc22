use camino::Utf8Path;
use id_tree::{Node, Tree};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{space1, u64 as parse_u64},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Command<'a> {
    Cd(&'a Utf8Path),
    Ls,
}

fn parse_path(i: &str) -> IResult<&str, &Utf8Path> {
    map(
        take_while1(|c: char| c.is_alphabetic() || "./".contains(c)),
        Into::into,
    )(i)
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;

    let parse_ls = map(tag("ls"), |_| Command::Ls);
    let parse_cd = map(preceded(tag("cd "), parse_path), Command::Cd);
    alt((parse_ls, parse_cd))(i)
}

#[derive(Debug)]
enum Entry<'a> {
    Directory(&'a Utf8Path),
    File(u64, &'a Utf8Path),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Directory);
    let parse_file = map(separated_pair(parse_u64, space1, parse_path), |(s, p)| {
        Entry::File(s, p)
    });

    alt((parse_dir, parse_file))(i)
}

#[derive(Debug)]
enum Line<'a> {
    Command(Command<'a>),
    Entry(Entry<'a>),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[derive(Debug)]
struct FsEntry<'a> {
    _name: &'a Utf8Path,
    size: u64,
}

fn parse_input(i: &str) -> Tree<FsEntry> {
    use id_tree::InsertBehavior::*;

    let mut tree = Tree::<FsEntry>::new();
    let root = tree
        .insert(
            Node::new(FsEntry {
                _name: "/".into(),
                size: 0,
            }),
            AsRoot,
        )
        .unwrap();

    let mut current = root;

    i.lines()
        .map(|line| all_consuming(parse_line)(line).unwrap().1)
        .for_each(|line| match line {
            Line::Command(c) => match c {
                Command::Ls => {}
                Command::Cd(path) => match path.as_str() {
                    "/" => current = tree.root_node_id().unwrap().clone(),
                    ".." => current = tree.get(&current).unwrap().parent().unwrap().clone(),
                    _ => {
                        let dir = tree
                            .insert(
                                Node::new(FsEntry {
                                    _name: path,
                                    size: 0,
                                }),
                                UnderNode(&current),
                            )
                            .unwrap();
                        current = dir;
                    }
                },
            },
            Line::Entry(e) => match e {
                Entry::Directory(_) => {}
                Entry::File(size, name) => {
                    tree.insert(
                        Node::new(FsEntry { _name: name, size }),
                        UnderNode(&current),
                    )
                    .unwrap();
                }
            },
        });

    tree
}

fn entries_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> u64 {
    node.data().size
        + node
            .children()
            .iter()
            .fold(0, |acc, e| acc + entries_size(tree, tree.get(e).unwrap()))
}

fn part1(tree: &Tree<FsEntry>) -> u64 {
    tree.traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|node| entries_size(tree, node))
        .filter(|&s| s < 100000)
        .sum()
}

fn part2(tree: &Tree<FsEntry>) -> u64 {
    let root_node = tree.root_node_id().unwrap();
    let free_space = 70000000 - entries_size(tree, tree.get(root_node).unwrap());
    let to_free = 30000000 - free_space;

    tree.traverse_pre_order(root_node)
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|node| entries_size(tree, node))
        .filter(|&s| s > to_free)
        .min()
        .unwrap()
}

fn main() {
    let tree = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part1(&tree));
    println!("Part 2: {}", part2(&tree));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Tree<FsEntry<'static>> {
        parse_input(include_str!("../test.txt"))
    }

    #[test]
    fn p1() {
        let tree = get_input();

        let expected = 95437;
        let result = part1(&tree);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let tree = get_input();

        let expected = 24933642;
        let result = part2(&tree);

        assert_eq!(expected, result);
    }
}
