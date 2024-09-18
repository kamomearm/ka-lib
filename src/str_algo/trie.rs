// use std::collections::HashMap;
use cargo_snippet::snippet;

#[snippet("Trie")]
#[derive(Debug, Clone)]
struct TrieNode {
    vertex: Option<char>,
    is_finished: bool,
    common: usize,
    next_verts: std::collections::HashMap<char, TrieNode>,
}

#[snippet("Trie")]
impl TrieNode {
    fn new(c: char, is_finished: bool) -> TrieNode {
        TrieNode {
            vertex: Some(c),
            is_finished,
            common: 0,
            next_verts: std::collections::HashMap::new(),
        }
    }

    fn new_root() -> TrieNode {
        TrieNode {
            vertex: None,
            is_finished: false,
            common: 0,
            next_verts: std::collections::HashMap::new(),
        }
    }
}

#[snippet("Trie")]
#[derive(Debug, Clone)]
pub struct Trie {
    root: TrieNode,
}

#[snippet("Trie")]
impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode::new_root(),
        }
    }
    // trie木の中にある文字列達と、与えられた文字列の最長共通接頭辞の長さの総和を返す
    pub fn abc353e(&self, vc: &[char]) -> usize {
        let mut ret = 0;
        let mut current_node = &self.root;
        for c in vc {
            if current_node.next_verts.contains_key(c) {
                current_node = current_node.next_verts.get(c).unwrap();
                eprintln!("{:?}: {}", current_node.vertex, current_node.common);
                ret += current_node.common;
            } else {
                break;
            }
        }
        ret
    }
    pub fn insert(&mut self, vc: &[char]) {
        let mut current_node = &mut self.root;

        for c in vc {
            current_node.common += 1;
            if current_node.next_verts.contains_key(c) {
                current_node = current_node.next_verts.get_mut(c).unwrap();
            } 
            else {
                current_node
                    .next_verts
                    .insert(*c, TrieNode::new(*c, false));
                current_node = current_node.next_verts.get_mut(c).unwrap();
            }
        }
        current_node.common += 1;
        current_node.is_finished = true;
    }

    pub fn contains(&mut self, vc: &[char]) -> bool {
        let mut current_node = &mut self.root;
        for c in vc {
            if current_node.next_verts.contains_key(c) {
                current_node = current_node.next_verts.get_mut(c).unwrap();
            } else {
                return false;
            }
        }
        true
    }
    pub fn lcp(&mut self, vc: &[char]) -> usize {
        let mut current_node = &mut self.root;
        let mut ret = 0;
        for c in vc {
            if current_node.next_verts.contains_key(c) {
                current_node = current_node.next_verts.get_mut(c).unwrap();
                if current_node.common >= 2 {
                    ret += 1
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }
        ret
    }
}
