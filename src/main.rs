use llfe::prelude::*;

fn main() {
    let source = load_file("sample/source.llfe", true);

    let lexer = source.unwrap();

    // let e = lexer.find_section_contents(&vec![], &mut contents);

    let e = lexer.find_section_names();

    println!("---");
    println!("{:?}", e);
    println!("---");
}
