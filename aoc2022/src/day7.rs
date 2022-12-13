use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, rest, value},
    sequence::tuple,
    IResult,
};
use qu::ick_use::*;

const INPUT: &str = include_str!("../input/7");

pub fn first() -> Result<usize> {
    let fs = FsEntry::from_input(INPUT)?;
    Ok(fs
        .walk()
        .filter_map(|entry| match entry {
            FsEntry::File { .. } => None,
            FsEntry::Dir { size, .. } => {
                if *size <= 100_000 {
                    Some(*size)
                } else {
                    None
                }
            }
        })
        .sum())
}

pub fn second() -> Result<usize> {
    const TOTAL_SPACE: usize = 70_000_000;
    const SPACE_NEEDED: usize = 30_000_000;
    let fs = FsEntry::from_input(INPUT)?;
    let total_used = fs.size();
    assert!(TOTAL_SPACE > total_used);
    let free_available = TOTAL_SPACE - total_used;
    assert!(SPACE_NEEDED > free_available);
    let space_to_free = SPACE_NEEDED - free_available;

    Ok(fs
        .walk()
        .filter_map(|entry| match entry {
            FsEntry::File { .. } => None,
            FsEntry::Dir { size, .. } => {
                if *size >= space_to_free {
                    Some(*size)
                } else {
                    None
                }
            }
        })
        .min()
        .unwrap())
}

// Filesystem builder

#[derive(Debug, Clone)]
struct FsBuilder<'a> {
    root: FsEntry<'a>,
    dir_stack: Vec<&'a str>,
}

impl<'a> FsBuilder<'a> {
    fn new() -> Self {
        FsBuilder {
            root: FsEntry::new_dir("/"),
            dir_stack: vec![],
        }
    }

    fn process(&mut self, line: Line<'a>) {
        match line {
            Line::CmdCdRoot => {
                assert!(self.dir_stack.is_empty())
            }
            Line::CmdCdDir { name } => {
                self.dir_stack.push(name);
            }
            Line::CmdCdUp => {
                let dir = self.dir_stack.pop();
                assert!(dir.is_some());
            }
            Line::CmdLs => (),
            Line::File { name, size } => self.add_file(name, size),
            Line::Dir { name } => self.add_directory(name),
        }
    }

    fn add_directory(&mut self, name: &'a str) {
        let mut entry = &mut self.root;
        for dir in &self.dir_stack {
            entry = entry.child_mut(dir)
        }
        entry.children_mut().push(FsEntry::new_dir(name));
    }

    fn add_file(&mut self, name: &'a str, size: usize) {
        let mut entry = &mut self.root;
        entry.set_size(entry.size() + size);
        for dir in &self.dir_stack {
            entry = entry.child_mut(dir);
            entry.set_size(entry.size() + size);
        }
        entry.children_mut().push(FsEntry::new_file(name, size));
    }
}

// Would probably be better to use `petgraph` because it has features allowing you to modify a
// graph you have pointers into.
#[derive(Debug, Clone)]
enum FsEntry<'a> {
    Dir {
        name: &'a str,
        children: Vec<FsEntry<'a>>,
        size: usize,
    },
    File {
        name: &'a str,
        size: usize,
    },
}

impl<'a> FsEntry<'a> {
    fn from_input(input: &'a str) -> Result<Self> {
        let mut fs = FsBuilder::new();
        for line in input.lines() {
            let line = Line::parse(line)?;
            fs.process(line);
        }
        Ok(fs.root)
    }

    fn new_dir(name: &'a str) -> Self {
        FsEntry::Dir {
            name,
            children: vec![],
            size: 0,
        }
    }

    fn new_file(name: &'a str, size: usize) -> Self {
        FsEntry::File { name, size }
    }

    fn name(&self) -> &'a str {
        match self {
            FsEntry::Dir { name, .. } => name,
            FsEntry::File { name, .. } => name,
        }
    }

    fn set_size(&mut self, new_size: usize) {
        match self {
            FsEntry::Dir { size, .. } => *size = new_size,
            FsEntry::File { size, .. } => *size = new_size,
        }
    }

    fn size(&self) -> usize {
        match self {
            FsEntry::Dir { size, .. } => *size,
            FsEntry::File { size, .. } => *size,
        }
    }

    fn children(&self) -> &[FsEntry<'a>] {
        let FsEntry::Dir { children, .. } = self else {
            panic!("not a directory");
        };
        children
    }

    fn children_mut(&mut self) -> &mut Vec<FsEntry<'a>> {
        let FsEntry::Dir { children, .. } = self else {
            panic!("not a directory");
        };
        children
    }

    fn child_mut(&mut self, name: &str) -> &mut FsEntry<'a> {
        self.children_mut()
            .iter_mut()
            .find(|child| child.name() == name)
            .expect("cannot find entry with given name")
    }

    fn child_idx(&self, idx: usize) -> Option<&FsEntry<'a>> {
        self.children().get(idx)
    }

    fn descendant_idx(&self, idxs: &[usize]) -> &FsEntry<'a> {
        let mut entry = self;
        for idx in idxs {
            entry = entry.child_idx(*idx).unwrap();
        }
        entry
    }

    fn walk(&self) -> impl Iterator<Item = &'_ FsEntry<'a>> + '_ {
        let index_stack = if self.children().is_empty() {
            vec![]
        } else {
            vec![0]
        };
        FsIter {
            fs: self,
            index_stack,
        }
    }
}

struct FsIter<'iter, 'a> {
    fs: &'iter FsEntry<'a>,
    index_stack: Vec<usize>,
}

impl<'iter, 'a> Iterator for FsIter<'iter, 'a> {
    type Item = &'iter FsEntry<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index_stack.is_empty() {
            return None;
        }

        let mut entry = self.fs;
        // walk down the indices until we get to the child we are interested in
        for idx in self.index_stack.iter().copied() {
            entry = entry.child_idx(idx).unwrap();
        }
        // now work out what the next entry is
        if let FsEntry::Dir { children, .. } = entry {
            if !children.is_empty() {
                self.index_stack.push(0);
                return Some(entry);
            }
        }
        // For each index on the stack, see if we can increment it, and if not pop it and move on
        // to the next one.
        loop {
            if self.index_stack.is_empty() {
                break;
            }
            let last_idx_idx = self.index_stack.len() - 1;
            let descend_idxs = &self.index_stack[..last_idx_idx];
            let last_idx = self.index_stack[last_idx_idx];
            let entry = self.fs.descendant_idx(descend_idxs);
            if entry.children().get(last_idx + 1).is_some() {
                self.index_stack[last_idx_idx] += 1;
                break;
            } else {
                self.index_stack.pop();
            }
        }
        Some(entry)
    }
}

// Input parser

#[derive(Debug, Copy, Clone)]
enum Line<'a> {
    CmdCdRoot,
    CmdCdDir { name: &'a str },
    CmdCdUp,
    CmdLs,
    File { name: &'a str, size: usize },
    Dir { name: &'a str },
}

impl<'a> Line<'a> {
    /// Parse a single line.
    fn parse(i: &'a str) -> Result<Self> {
        let (_, line) = parse_line(i).map_err(|_| format_err!("failed to parse line"))?;
        Ok(line)
    }
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    all_consuming(alt((
        value(Line::CmdCdRoot, tag("$ cd /")),
        value(Line::CmdCdUp, tag("$ cd ..")),
        value(Line::CmdLs, tag("$ ls")),
        map(tuple((tag("$ cd "), rest)), |(_, name)| Line::CmdCdDir {
            name,
        }),
        map(tuple((tag("dir "), rest)), |(_, name)| Line::Dir { name }),
        map(
            tuple((map_res(digit1, str::parse), tag(" "), rest)),
            |(size, _, name)| Line::File { size, name },
        ),
    )))(i)
}
