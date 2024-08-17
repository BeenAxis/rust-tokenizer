use std::fs::File;
use std::ops::Index;
use std::time::Instant;
use crate::stacktoken::{TestCustomTokens, Token, TokenArgs, TokenBuilder};

mod utils;
mod stacktoken;

fn main() {
    /*let mut hl2:LinkedList<String> = LinkedList::new();
    hl2.push_back(String::from("A"));
    hl2.push_back(String::from("B"));
    println!("{hl2:?}");
    let mut hl:LinkedHashList<String> = LinkedHashList::new();
    for i in 0..100000 {
        hl.push(format!("A with {}", i));
    }
    let el = Instant::now();
    hl.replace(1000..10000, String::from("NEW"));
    hl.replace(1000..50000, String::from("NEW"));
    hl.replace(1000..20000, String::from("NEW"));*/
    
    /*let el = el.elapsed();
    println!("{hl:?}");
    println!("Replace: {el:?} {}", hl.len());*/

    let mut tkb: TokenBuilder<TestCustomTokens> = TokenBuilder::new();
    tkb.from_file("test.mc");
    let els = Instant::now();
    /*let founded = tkb.found("(\"");
    tkb.replace(founded.as_slice(), |_| Token::Custom(TestCustomTokens::StartParen));
    let founded = tkb.found("\")");
    tkb.replace(founded.as_slice(), |_| Token::Custom(TestCustomTokens::EndParen));*/
    let range = tkb.found_range(Token::Char('"'), Token::Char('"'));

    tkb.replace(range.as_slice(), |list| {
        Token::StrLit(list.scan_string())
    });

    let founded = tkb.found("compileOnly");
    tkb.replace(founded.as_slice(), |_| Token::Custom(TestCustomTokens::CompileOnly));

    let range = tkb.found_range(Token::Char('('), Token::Char(')'));
    //println!("Range: {range:#?}");
    tkb.replace(range.as_slice(), |mut list| {
        list.remove_backs();
        Token::Custom(TestCustomTokens::Group(list))
    });

    let range = tkb.found_range(Token::Custom(TestCustomTokens::CompileOnly),
                                Token::Custom(TestCustomTokens::Group(TokenArgs::new()))
    );
    tkb.replace(range.as_slice(), |mut list| {
        if let Token::Custom(TestCustomTokens::Group(a)) = &list[1] {
            let a = a[0].to_text();
            let args: Vec<&str> = a.split(":").collect();
            if args.len() == 3 {
                Token::Custom(TestCustomTokens::CompileOnlyCompound(
                    args[0].to_string(),
                    args[1].to_string(),
                    args[2].to_string(),
                ))
            } else {
                Token::Custom(TestCustomTokens::FailedCompound)
            }
        } else {
            Token::Custom(TestCustomTokens::FailedCompound)
        }
    });
    let aaa = els.elapsed();

    //tkb.debug();
    println!("Elapsed: {:?}", aaa);

    tkb.tokens.iter().for_each(|x| {
        if let Token::Custom(TestCustomTokens::CompileOnlyCompound
                             (group, artifact, ver)) = x {
            println!(" DEP {group} {artifact} {ver}");
        }
        if let Token::Custom(TestCustomTokens::FailedCompound) = x {
            println!(" FAILED");
        }
    });
}