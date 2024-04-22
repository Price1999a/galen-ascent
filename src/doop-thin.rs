use std::time::Instant;
use ascent::ascent_par;
use ascent::rayon::ThreadPoolBuilder;
use clap::{arg, Command};
use zip::read::ZipFile;

mod utils;

pub type Str = &'static str;

ascent_par! {
    relation isType(String);
    relation isReferenceType(String);
    relation isArrayType(String);
    relation isClassType(String);
    relation isInterfaceType(String);
    relation DirectSuperclass(String, String);
    relation DirectSuperinterface(String, String);
    relation ApplicationClass(String);
    relation MainClass(String);
    relation Field_DeclaringType(String, String);
    relation Method_DeclaringType(String, String);
    relation Method_ReturnType(String, String);
    relation Method_SimpleName(String, String);
    relation Method_Params(String, String);
    relation Method_Descriptor(String, String);
    relation Method_Modifier(String, String);

}


fn input_process(prog: &mut AscentProgram, zip_file_name: &String) {
    //.input DirectSuperclass(IO="file", filename="DirectSuperclass.facts", delimiter="\t")
    let tmp = utils::utils::read_file_from_zip_to_vec_doop_thin(zip_file_name,"database/DirectSuperclass.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.DirectSuperclass.extend(tmp);
    // println!("[debug]: {:?}",prog.DirectSuperclass.get(0));
    //.input DirectSuperinterface(IO="file", filename="DirectSuperinterface.facts", delimiter="\t")
    let tmp = utils::utils::read_file_from_zip_to_vec_doop_thin(zip_file_name,"database/DirectSuperinterface.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.DirectSuperinterface.extend(tmp);
}

fn main() {
    let matches = Command::new("Doop-thin Program")
        .version("1.0")
        .author("Tianqi Shen")
        .arg(arg!(--database <VALUE>).required(true))
        .get_matches();
    let zip_file_name = matches.get_one::<String>("database").expect("required");
    println!("zip file name: {}", zip_file_name);
    // utils::utils::list_zip_contents(zip_file_name)?;

    let _pool = ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();
    println!("Hello, world!");
    let mut prog = AscentProgram::default();
    // input & facts area below
    input_process(&mut prog, zip_file_name);


    let start = Instant::now();
    prog.run();
    let duration = start.elapsed();
    println!("方法运行时间: {:?}", duration);
    // println!("{}",prog.DirectSuperinterface.len())
    // for (i, j) in prog.p {
    //     println!("{}, {}", i, j);
    // }
}
