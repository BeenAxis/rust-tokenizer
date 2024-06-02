use std::fs::File;
use std::collections::LinkedList;
use std::fmt::Debug;
use std::time::Instant;

use utils::read_lines;

use crate::Token::StrLit;
use crate::utils::linkedlist::LinkedHashList;

mod utils;

fn main() {
    let mut hl2:LinkedList<String> = LinkedList::new();
    hl2.push_back(String::from("A"));
    hl2.push_back(String::from("B"));
    println!("{hl2:?}");
    let mut hl:LinkedHashList<String> = LinkedHashList::new();
    for i in 0..100000 {
        hl.push(format!("A with {}", i));
    }
    let el = Instant::now();
    hl.replace(1000..5000, String::from("NEW"));
    hl.replace(950..1050, String::from("NEW"));
    hl.replace(10000..95000, String::from("NEW"));
    //hl.reindex();
    //0..100000
    //0..[500 1000]..100000
    let el = el.elapsed();
    println!("{hl:?}");
    println!("Replace: {el:?} {}", hl.len());
    /*let mut tkb: TokenBuilder<TestCustomToken> = TokenBuilder::new();
    tkb.from_file("test.mc");

    let t = Instant::now();

    let founded = tkb.found("t");
    let del_f = t.elapsed();
    println!("{founded:?}");
    tkb.replace(founded.as_slice(), Token::Custom(TestCustomToken::MethodCall("test".into())));
    //tkb.replace(a.as_slice(), Token::Custom(TestCustomToken::MethodCall("test".into())));
    let del = t.elapsed();
    tkb.debug();
    println!("Elapsed: {del:?}");
    println!("Elapsed del f: {del_f:?}");*/
}

pub trait TokenHolder: Clone + Debug + PartialEq {}

pub trait Tokens<HOLDER: TokenHolder> {
    fn convert_to_tokens(&self) -> Vec<Token<HOLDER>>;
}

impl<HOLDER: TokenHolder, const T: usize> Tokens<HOLDER> for &[Token<HOLDER>; T] {
    fn convert_to_tokens(&self) -> Vec<Token<HOLDER>> {
        self.to_vec()
    }
}

impl<HOLDER: TokenHolder> Tokens<HOLDER> for &str {
    fn convert_to_tokens(&self) -> Vec<Token<HOLDER>> {
        let mut tokens: Vec<Token<HOLDER>> = Vec::new();
        self.chars().for_each(|char| {
            tokens.push(Token::Char(char));
        });
        tokens
    }
}


impl<HOLDER: TokenHolder> Tokens<HOLDER> for String {
    fn convert_to_tokens(&self) -> Vec<Token<HOLDER>> {
        let mut tokens: Vec<Token<HOLDER>> = Vec::new();
        self.chars().for_each(|char| {
            tokens.push(Token::Char(char));
        });
        tokens
    }
}

#[derive(Default)]
pub struct TokenBuilder<HOLDER: TokenHolder> {
    pub tokens: LinkedList<Token<HOLDER>>,
}

impl<HOLDER: TokenHolder> TokenBuilder<HOLDER> {
    pub fn new() -> TokenBuilder<HOLDER> {
        TokenBuilder {
            tokens: LinkedList::default()
        }
    }

    pub fn push_custom(&mut self, tok: HOLDER) {
        self.tokens.push_back(Token::Custom(tok))
    }

    pub fn from_file(&mut self, path: impl Into<String>) {
        let lines = read_lines(path);
        for line in lines {
            line.chars().for_each(|ch| {
                self.tokens.push_back(Token::Char(ch));
            });
            self.tokens.push_back(Token::NewLine);
        }
        self.tokens.push_back(Token::EOF);
    }

    pub fn found(&mut self, pattern: impl Tokens<HOLDER>) -> Vec<(usize, usize)> {
        let mut start = None;
        let pattern = pattern.convert_to_tokens();
        let pattern: Vec<&Token<HOLDER>> = pattern.iter().collect();
        let mut temp = 0usize;
        let mut indexes: Vec<(usize, usize)> = Vec::new();
        self.tokens.iter().enumerate().for_each(|(idx, x)| {
            if start.is_none() {
                if pattern[0] == x {
                    if pattern.len() == 1 {
                        indexes.push((idx, idx));
                        start = None;
                        temp = 0;
                    }else {
                        start = Some(idx);
                        temp += 1;
                    }
                }
            } else {
                if pattern[temp] == x {
                    temp += 1;
                } else {
                    start = None;
                    temp = 0;
                }
                if pattern.len() - 1 < temp {
                    indexes.push((start.unwrap(), idx));
                    start = None;
                    temp = 0;
                }
            }
        });
        indexes
    }

    pub fn replace(&mut self, slices: &[(usize, usize)], eq: Token<HOLDER>) {
        //a -> b -> c -> d -> e -> f -> g = (2, 4) , (6, 7)
        //a -> b -> || c -> d -> e -> f -> g
        //a -> b -> MAP -> f -> g = (2, 4), (dx: 3, dy: 4) -3!
        let mut negative_offset = 0usize;
        slices.iter().for_each(|x| {
            if x.1 + 1 - negative_offset >= self.tokens.len() {
                return;
            }
            let mut right = self.tokens.split_off(x.1 + 1 - negative_offset);
            let slice = self.tokens.split_off(x.0 - negative_offset);
            negative_offset += slice.len() - 1;
            self.tokens.push_back(eq.clone());
            self.tokens.append(&mut right);
        });
    }

    pub fn debug(&self) {
        self.tokens.iter().for_each(|tk| {
            println!("{:?}", tk);
        });
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TestCustomToken {
    Def,
    MethodCall(String),
}

impl TokenHolder for TestCustomToken {}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<T: TokenHolder> {
    Empty,
    Char(char),
    StrLit(String),
    IntLit(i32),
    EOF,
    NewLine,
    Custom(T),
}

impl<T: TokenHolder> Token<T> {
    pub fn to_text(&self) -> String {
        match self {
            Token::Empty => { String::from("") }
            Token::Char(c) => { format!("{}", c) }
            StrLit(a) => { String::from(a) }
            Token::IntLit(i) => { format!("{}", i) }
            Token::EOF => { String::from("<eof>") }
            Token::NewLine => { String::from("\n") }
            Token::Custom(o) => { format!("{:?}", o) }
        }
    }
}
