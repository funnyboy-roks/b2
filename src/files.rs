use std::{
    collections::BTreeMap,
    path::{Component, PathBuf},
};

use colored::Colorize;
use humanize_bytes::humanize_bytes_decimal;

use crate::api::{self, File};

#[derive(Debug)]
pub enum FileTree {
    Directory {
        name: String,
        children: BTreeMap<String, FileTree>,
    },
    File {
        name: String,
        file: File,
    },
    Root {
        children: BTreeMap<String, FileTree>,
    },
}

pub fn files_to_tree(files: Vec<File>) -> FileTree {
    let mut tree = FileTree::Root {
        children: Default::default(),
    };

    for file in files {
        match file.action {
            api::Action::Start => todo!(),
            api::Action::Hide => todo!(),
            api::Action::Upload => {
                let path = PathBuf::from_iter(file.file_name.split('/'));
                let mut curr = &mut tree;
                let comps: Vec<_> = path.components().collect();
                for comp in &comps[..comps.len() - 1] {
                    let Component::Normal(comp) = comp else {
                        unreachable!()
                    };
                    let comp = comp.to_str().unwrap();

                    match curr {
                        FileTree::Directory { name: _, children } => {
                            curr =
                                children
                                    .entry(comp.to_string())
                                    .or_insert(FileTree::Directory {
                                        name: comp.to_string(),
                                        children: Default::default(),
                                    });
                        }
                        FileTree::File { .. } => unreachable!(),
                        FileTree::Root { children } => {
                            curr = children
                                .entry(comp.to_string())
                                .or_insert(FileTree::Directory {
                                    name: comp.to_string(),
                                    children: Default::default(),
                                })
                        }
                    }
                }

                let last = comps.last().unwrap();
                let Component::Normal(last) = last else {
                    unreachable!()
                };
                let last = last.to_str().unwrap();

                match curr {
                    FileTree::Directory { name: _, children } => children.insert(
                        last.to_string(),
                        FileTree::File {
                            file,
                            name: last.to_string(),
                        },
                    ),
                    FileTree::File { .. } => unreachable!(),
                    FileTree::Root { children } => children.insert(
                        last.to_string(),
                        FileTree::File {
                            file,
                            name: last.to_string(),
                        },
                    ),
                };
            }
            api::Action::Folder => {
                unimplemented!("{:?}", file);
            }
        }
    }

    tree
}

pub fn print_tree(tree: FileTree, long: bool) {
    if long {
        println!(
            "  {}   {}   {}",
            "Size".underline(),
            "Date Uploaded".underline(),
            "Name".underline()
        );
    }
    print_tree_recur(tree, long, 0);
}

fn print_indent(indent: usize) {
    if indent > 0 {
        print!("{:>ind$}", "", ind = indent * 2);
    }
}

fn print_tree_recur(tree: FileTree, long: bool, indent: usize) {
    match tree {
        FileTree::Root { children } => {
            for (_, child) in children {
                print_tree_recur(child, long, indent);
            }
        }
        FileTree::Directory { name, children } => {
            if long {
                print!("                         ");
            }
            print_indent(indent);
            println!("{}/", name.blue());
            for (_, child) in children {
                print_tree_recur(child, long, indent + 1);
            }
        }
        FileTree::File { name, file } => {
            if long {
                print!(
                    "{:>6}   {:>13}   ",
                    humanize_bytes_decimal!(file.content_length)
                        .strip_suffix('B')
                        .unwrap()
                        .replace(' ', "")
                        .green(),
                    file.upload_timestamp.format("%e %h %Y").to_string().blue(),
                );
            }
            print_indent(indent);
            println!("{}", name.yellow());
        }
    }
}
