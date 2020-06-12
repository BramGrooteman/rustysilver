use failure::Error;
use regex::Regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub struct Query<'a> {
    source: &'a str,
    path: RPQTree<'a>,
    dest: &'a str,
}

pub struct RPQTree<'a> {
    left: Option<Box<RPQTree<'a>>>,
    right: Option<Box<RPQTree<'a>>>,
    data: &'a str,
}

impl<'a> Query<'a> {
    pub fn parse_queries(raw_queries: &'a str) -> Result<Vec<Self>, Error> {
        let mut queries: Vec<Query> = vec![];
        for l in raw_queries.lines() {
            queries.push(Query::parse_query(l));
        }

        Ok(queries)
    }

    fn parse_query(single_query: &'a str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.+), (.+), (.+)").unwrap();
        }

        let groups = RE.captures(single_query).unwrap();
        println!("{:?}", groups);

        let source = groups.get(1).unwrap().as_str();
        let dest = groups.get(3).unwrap().as_str();
        let path = RPQTree::from_str(groups.get(2).unwrap().as_str());

        Query { source, path, dest }
    }

    pub fn print(&self) -> () {
        print!("{}, ", self.source);
        self.path.print();
        print!(", {}", self.dest);
    }
}

impl<'a> RPQTree<'a> {
    pub fn from_str(inp: &'a str) -> Self {
        let mut level = 0;
        for (i, c) in inp.char_indices() {
            // println!("{}", c);
            if c == '(' {
                level += 1;
                continue;
            }
            if c == ')' {
                level -= 1;
                continue;
            }
            if level > 0 {
                continue;
            }
            if c == '/' {
                let l = &inp[0..i];
                let r = &inp[i + 1..];
                println!("L: '{}' R: '{}' data: '{}' ", l, r, &inp[i..i + 1]);
                return RPQTree {
                    left: Some(Box::new(RPQTree::from_str(l))),
                    right: Some(Box::new(RPQTree::from_str(r))),
                    data: &inp[i..i + 1],
                };
            }
        }

        if inp.chars().nth(0).unwrap() == '(' {
            for c in inp.chars() {
                if c == '(' {
                    level += 1;
                    continue;
                }
                if c == ')' {
                    level -= 1;
                    if level == 0 {
                        // Recurse
                        println!("{:?}", &inp[1..inp.len() - 1]);
                        return RPQTree::from_str(&inp[1..inp.len() - 1]);
                    }
                    continue;
                }
            }
        } else {
            // case value
            return RPQTree {
                left: None,
                right: None,
                data: inp,
            };
        }

        // println!("Error: parsing RPQ failed. {}", inp);

        RPQTree {
            left: None,
            right: None,
            data: inp,
        }
    }

    fn print(&self) -> () {
        if let Some(l) = &self.left {
            l.print();
        }

        print!("{}", self.data);

        if let Some(r) = &self.right {
            r.print();
        }
    }
}
