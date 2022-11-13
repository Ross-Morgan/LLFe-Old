use llfe::prelude::*;

fn main() {
    let source = load_file("sample/source.llfe", true);

    if let Err(e) = source {
        panic!("{e:?}");
    }

    let lexer = Lexer(source.unwrap());

    let mut contents = vec![];

    lexer.find_section_contents(&vec![], &mut contents);

    println!("---");
    println!("---");
}
