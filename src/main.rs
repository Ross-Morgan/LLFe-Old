use llfe::prelude::*;

fn main() {
    let source = load_file("sample/source.llfe", true);

    let lexer = match source {
        Ok(s) => Lexer(s),
        Err(e) => panic!("{:?}", e),
    };

    // let e = lexer.find_section_contents(&vec![], &mut contents);

    let e = lexer.find_section_names();

    println!("---");
    println!("{:?}", e);
    println!("---");
}
