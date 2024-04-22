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

    relation FormalParam(String, String, String);
    relation ThisVar(String, String);
    relation Var_Type(String, String);
    relation Var_DeclaringMethod(String, String);
    relation HeapAllocation_Type(String, String);
    relation MainMethodArgArray(String);
    relation MainMethodArgArrayContent(String);
    relation Instruction_Method(String, String);

    relation isVirtualMethodInvocation_Insn(String);
    relation isStaticMethodInvocation_Insn(String);
    relation FieldInstruction_Signature(String, String);
    relation LoadInstanceField_Base(String, String);
    relation LoadInstanceField_To(String, String);
    relation StoreInstanceField_From(String, String);
    relation StoreInstanceField_Base(String, String);
    relation LoadStaticField_To(String, String);
    relation StoreStaticField_From(String, String);

    relation ComponentType(String, String);
    relation LoadArrayIndex_Base(String, String);
    relation LoadArrayIndex_To(String, String);
    relation StoreArrayIndex_From(String, String);
    relation StoreArrayIndex_Base(String, String);
    relation AssignInstruction_To(String, String);
    relation AssignCast_From(String, String);
    relation AssignCast_Type(String, String);

    relation AssignLocal_From(String, String);
    relation AssignHeapAllocation_Heap(String, String);
    relation ReturnNonvoid_Var(String, String);
    relation MethodInvocation_Method(String, String);
    relation AssignReturnValue(String, String);
    relation ActualParam(String, String, String);
    relation VirtualMethodInvocation_Base(String, String);
    relation VirtualMethodInvocation_SimpleName(String, String);
    relation VirtualMethodInvocation_Descriptor(String, String);
    relation SpecialMethodInvocation_Base(String, String);
    relation MethodInvocation_Base(String, String);
    // Fat schema
    relation LoadInstanceField(String, String, String, String);
    relation StoreInstanceField(String, String, String, String);
    relation LoadStaticField(String, String, String);
    relation StoreStaticField(String, String, String);
    relation LoadArrayIndex(String, String, String);
    relation StoreArrayIndex(String, String, String);
    relation AssignCast(String, String, String, String);
    relation AssignLocal(String, String, String);
    relation AssignHeapAllocation(String, String, String);
    relation ReturnVar(String, String);
    relation StaticMethodInvocation(String, String, String);

    relation _ClassType(String);
    isType(class) <-- _ClassType(class);
    isReferenceType(class) <-- _ClassType(class);
    isClassType(class) <-- _ClassType(class);
    relation _ArrayType(String);
    isType(arrayType) <-- _ArrayType(arrayType);
    isReferenceType(arrayType) <-- _ArrayType(arrayType);
    isArrayType(arrayType) <-- _ArrayType(arrayType);
}


fn input_process(prog: &mut AscentProgram, zip_file_name: &String) {
    //.input DirectSuperclass(IO="file", filename="DirectSuperclass.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/DirectSuperclass.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.DirectSuperclass.extend(tmp);
    //.input DirectSuperinterface(IO="file", filename="DirectSuperinterface.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/DirectSuperinterface.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.DirectSuperinterface.extend(tmp);
    // .input MainClass(IO="file", filename="MainClass.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/MainClass.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), )  // 直接移除并返回向量的前两个元素
    });
    prog.MainClass.extend(tmp);
    // .input Method_Modifier(IO="file", filename="Method-Modifier.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/Method-Modifier.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.Method_Modifier.extend(tmp);
    // .input FormalParam(IO="file", filename="FormalParam.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/FormalParam.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.FormalParam.extend(tmp);
    // .input Var_Type(IO="file", filename="Var-Type.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/Var-Type.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.Var_Type.extend(tmp);
    // .input ComponentType(IO="file", filename="ComponentType.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ComponentType.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.ComponentType.extend(tmp);
    // .input AssignReturnValue(IO="file", filename="AssignReturnValue.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/AssignReturnValue.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.AssignReturnValue.extend(tmp);
    // .input ActualParam(IO="file", filename="ActualParam.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ActualParam.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), v.remove(0), v.remove(0))  // 直接移除并返回向量的前两个元素
    });
    prog.ActualParam.extend(tmp);
    // .input _ClassType(IO="file", filename="ClassType.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ClassType.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), )  // 直接移除并返回向量的前两个元素
    });
    prog._ClassType.extend(tmp);
    // .input _ArrayType(IO="file", filename="ArrayType.facts", delimiter="\t")
    let tmp = utils::utils::
    read_file_from_zip_to_vec_doop_thin(zip_file_name, "database/ArrayType.facts")
        .into_iter().map(|mut v| {
        (v.remove(0), )  // 直接移除并返回向量的前两个元素
    });
    prog._ArrayType.extend(tmp);
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
    // println!("{}",prog.isArrayType.len())
    // for (i, j) in prog.p {
    //     println!("{}, {}", i, j);
    // }
}
