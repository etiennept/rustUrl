use std::env::args;

fn main() {
    let args= args() ;
    args.for_each(|item| println!("{}" , item))

}
