use ascent::ascent_par;
use ascent::rayon::ThreadPoolBuilder;
use clap::{Arg, arg,Command};

pub type Str = &'static str;

ascent_par! {
    relation isType(Str);
    relation p(i32, i32);
    relation isReferenceType(Str);
    relation isArrayType(Str);
    relation isClassType(Str);
    relation isInterfaceType(Str);
    relation DirectSuperclass(Str, Str);

}

fn main() {
    let matches = Command::new("My Test Program")
        .version("1.0")
        .author("Tianqi Shen")
        .about("Does awesome things")
        .arg(arg!(--database <VALUE>).required(true))
        .get_matches();

    let _pool = ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();
    println!("Hello, world!");
    // let mut prog = AscentProgram::default();

    // .input DirectSuperclass(IO="file", filename="DirectSuperclass.facts", delimiter="\t")
}