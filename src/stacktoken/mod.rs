use std::collections::LinkedList;
use std::fmt::Debug;
use std::ops::Index;
use crate::stacktoken::TestCustomTokens::Group;
use crate::utils::read_lines;

#[derive(Debug, PartialEq, Clone)]
pub struct TokenArgs<HOLDER: TokenHolder> {
    pub tokens: Vec<Token<HOLDER>>
}

impl<HOLDER: TokenHolder> TokenArgs<HOLDER> {

    pub fn new() -> Self {
        Self {
            tokens: Vec::new()
        }
    }

    pub fn from_list(list: LinkedList<Token<HOLDER>>) -> Self {
        let mut tokens = Vec::new();
        for tok in list {
            tokens.push(tok);
        }
        Self {
            tokens
        }
    }

    pub fn remove_backs(&mut self) {
        let tokens = self.tokens[1..self.tokens.len() - 1].to_vec();

        self.tokens = tokens;
    }

    pub fn scan_string(&self) -> String {
        let mut a: String = self.tokens.iter().map(|x| x.to_text().to_string()).collect();
        a = a[1..a.len() - 1].to_string();
        a
    }

}

impl<HOLDER: TokenHolder> Index<usize> for TokenArgs<HOLDER> {
    type Output = Token<HOLDER>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
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

    pub fn from_string(&mut self, str: &str) {
        str.chars().for_each(|ch| {
            self.tokens.push_back(Token::Char(ch));
        });
        self.tokens.push_back(Token::NewLine);
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

    pub fn found(&self, pattern: impl Tokens<HOLDER>) -> Vec<(usize, usize)> {
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

    pub fn found_range(&self, start: Token<HOLDER>, end: Token<HOLDER>) -> Vec<(usize, usize)> {
        let mut indexes: Vec<(usize, usize)> = Vec::new();
        let mut start_idx:Option<usize> = None;
        let mut cur = 0;
        for token in &self.tokens {
            match start_idx {
                None => {
                    if token == &start {
                        start_idx = Some(cur);
                    }
                }
                Some(start) => {
                    if token == &end {
                        start_idx = None;
                        indexes.push((start, cur));
                    }
                }
            }
            cur += 1;
        }
        indexes
    }

    pub fn replace(&mut self, slices: &[(usize, usize)], eq: fn (TokenArgs<HOLDER>) -> Token<HOLDER>) {
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
            self.tokens.push_back(eq(TokenArgs::from_list(slice)));
            self.tokens.append(&mut right);
        });
    }

    pub fn debug(&self) {
        self.tokens.iter().for_each(|tk| {
            println!("{:?}", tk);
        });
    }
}

#[derive(Debug, Clone)]
pub enum TestCustomTokens {
    StartParen,
    EndParen,
    CompileOnly,
    CompileOnlyCompound(String, String, String),
    FailedCompound,
    Group(TokenArgs<TestCustomTokens>)
}

impl PartialEq for TestCustomTokens {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TestCustomTokens::StartParen => {
                if let TestCustomTokens::StartParen = other {
                    true
                } else {
                    false
                }
            }
            TestCustomTokens::EndParen => {
                if let TestCustomTokens::EndParen = other {
                    true
                } else {
                    false
                }
            }
            TestCustomTokens::CompileOnly => {
                if let TestCustomTokens::CompileOnly = other {
                    true
                } else {
                    false
                }
            }
            TestCustomTokens::FailedCompound => {
                if let TestCustomTokens::FailedCompound = other {
                    true
                } else {
                    false
                }
            }
            TestCustomTokens::CompileOnlyCompound(_, _, _) => {
                false
            }
            TestCustomTokens::Group(_) => {
                if let Group(_) = other {
                    true
                } else {
                    false
                }
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other)
    }
}

impl TokenHolder for TestCustomTokens {}

#[derive(Debug, Clone)]
pub enum Token<T: TokenHolder> {
    Empty,
    Char(char),
    StrLit(String),
    String(String),
    IntLit(i32),
    EOF,
    NewLine,
    Custom(T),
}

impl<HOLDER: TokenHolder> PartialEq for Token<HOLDER> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Empty, Token::Empty) => true,
            (Token::Char(a), Token::Char(b)) => a == b,
            (Token::StrLit(a), Token::StrLit(b)) => a == b,
            (Token::IntLit(a), Token::IntLit(b)) => a == b,
            (Token::EOF, Token::EOF) => true,
            (Token::NewLine, Token::NewLine) => true,
            (Token::Custom(a), Token::Custom(b)) => a == b,
            (Token::String(_), Token::String(_)) => true,
            _ => false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T: TokenHolder> Token<T> {
    pub fn to_text(&self) -> String {
        match self {
            Token::Empty => { String::from("") }
            Token::Char(c) => { format!("{}", c) }
            Token::StrLit(a) => { String::from(a) }
            Token::IntLit(i) => { format!("{}", i) }
            Token::EOF => { String::from("<eof>") }
            Token::NewLine => { String::from("\n") }
            Token::Custom(o) => { format!("{:?}", o) }
            Token::String(a) => { format!("{:?}", a) }
        }
    }
}
