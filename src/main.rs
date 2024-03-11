use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;
use ascent::ascent_par;
use ascent::rayon::ThreadPoolBuilder;
use zip::ZipArchive;
ascent_par! {
    relation p(i32, i32);
    relation q(i32, i32, i32);
    relation r(i32, i32, i32);
    relation c(i32, i32, i32);
    relation u(i32, i32, i32);
    relation s(i32, i32);

    p(x, z) < - - p(x, y), p(y, z);
    q(x, r, z) < - - p(x, y), q(y, r, z);
    p(x, z) < - - p(y, w), u(w, r, z), q(x, r, y);
    p(x, z) < - - c(y, w, z), p(x, w), p(x, y);
    q(x, q, z) < - - q(x, r, z), s(r, q);
    q(x, e, o) < - - q(x, y, z), r(y, u, e), q(z, u, o);
}

fn read_file_to_vec_3(file_path: &str) -> Vec<(i32, i32, i32)> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let values: Vec<&str> = line.split(',').collect();
            if values.len() == 3 {
                if let (Ok(val1), Ok(val2), Ok(val3)) = (
                    i32::from_str(values[0]),
                    i32::from_str(values[1]),
                    i32::from_str(values[2]),
                ) {
                    vec.push((val1, val2, val3));
                }
            }
        }
    }

    vec
}

fn read_file_to_vec_2(file_path: &str) -> Vec<(i32, i32)> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let values: Vec<&str> = line.split(',').collect();
            if values.len() == 2 {
                if let (Ok(val1), Ok(val2)) = (
                    i32::from_str(values[0]),
                    i32::from_str(values[1]),
                ) {
                    vec.push((val1, val2));
                }
            }
        }
    }

    vec
}

fn read_file_from_zip_to_vec_3(zip_path: &str, file_path: &str) -> Vec<(i32, i32, i32)> {
    let file = File::open(zip_path).expect("Failed to open ZIP file");
    let mut archive = ZipArchive::new(file).expect("Failed to create ZIP archive");

    let mut vec = Vec::new();

    if let Ok(zip_file) = archive.by_name(file_path) {
        let reader = BufReader::new(zip_file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let values: Vec<&str> = line.split(',').collect();
                if values.len() == 3 {
                    if let (Ok(val1), Ok(val2), Ok(val3)) = (
                        i32::from_str(values[0]),
                        i32::from_str(values[1]),
                        i32::from_str(values[2]),
                    ) {
                        vec.push((val1, val2, val3));
                    }
                }
            }
        }
    }

    vec
}

fn read_file_from_zip_to_vec_2(zip_path: &str, file_path: &str) -> Vec<(i32, i32)> {
    let file = File::open(zip_path).expect("Failed to open ZIP file");
    let mut archive = ZipArchive::new(file).expect("Failed to create ZIP archive");

    let mut vec = Vec::new();

    if let Ok(zip_file) = archive.by_name(file_path) {
        let reader = BufReader::new(zip_file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let values: Vec<&str> = line.split(',').collect();
                if values.len() == 2 {
                    if let (Ok(val1), Ok(val2)) = (
                        i32::from_str(values[0]),
                        i32::from_str(values[1]),
                    ) {
                        vec.push((val1, val2));
                    }
                }
            }
        }
    }

    vec
}

fn main() {
    let _pool = ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();
    println!("Hello, world!");
    let mut program = AscentProgram::default();
    let mut program_inc1 = AscentProgram::default();
    let mut program_inc2 = AscentProgram::default();
    let mut program_inc3 = AscentProgram::default();
    let mut program_inc4 = AscentProgram::default();
    let mut program_inc5 = AscentProgram::default();
    let mut program_inc6 = AscentProgram::default();

    // let vec_p = read_file_to_vec_2("./data/input-1000/p.txt");
    // let vec_q = read_file_to_vec_3("./data/input-1000/q.txt");
    // let vec_r = read_file_to_vec_3("./data/input-1000/r.txt");
    // let vec_c = read_file_to_vec_3("./data/input-1000/c.txt");
    // let vec_u = read_file_to_vec_3("./data/input-1000/u.txt");
    // let vec_s = read_file_to_vec_2("./data/input-1000/s.txt");

    let vec_p = read_file_from_zip_to_vec_2("./data/input.zip", "input/p.txt");
    let vec_q = read_file_from_zip_to_vec_3("./data/input.zip", "input/q.txt");
    let vec_r = read_file_from_zip_to_vec_3("./data/input.zip", "input/r.txt");
    let vec_c = read_file_from_zip_to_vec_3("./data/input.zip", "input/c.txt");
    let vec_u = read_file_from_zip_to_vec_3("./data/input.zip", "input/u.txt");
    let vec_s = read_file_from_zip_to_vec_2("./data/input.zip", "input/s.txt");


    println!("file read info:");
    println!("p{}", vec_p.len());
    println!("q{}", vec_q.len());
    println!("r{}", vec_r.len());
    println!("c{}", vec_c.len());
    println!("u{}", vec_u.len());
    println!("s{}", vec_s.len());
    println!();

    let (vec_p_first, _vec_p_last) = vec_p.split_at(vec_p.len() - 10);
    let (vec_q_first, _vec_q_last) = vec_q.split_at(vec_q.len() - 10);
    let (vec_r_first, _vec_r_last) = vec_r.split_at(vec_r.len() - 10);
    let (vec_c_first, _vec_c_last) = vec_c.split_at(vec_c.len() - 10);
    let (vec_u_first, _vec_u_last) = vec_u.split_at(vec_u.len() - 10);
    let (vec_s_first, _vec_s_last) = vec_s.split_at(vec_s.len() - 10);

    let vec_p_first_owned = vec_p_first.to_vec();
    let vec_q_first_owned = vec_q_first.to_vec();
    let vec_r_first_owned = vec_r_first.to_vec();
    let vec_c_first_owned = vec_c_first.to_vec();
    let vec_u_first_owned = vec_u_first.to_vec();
    let vec_s_first_owned = vec_s_first.to_vec();

    // let vec_p_last_owned = vec_p_last.to_vec();
    // let vec_q_last_owned = vec_q_last.to_vec();
    // let vec_r_last_owned = vec_r_last.to_vec();
    // let vec_c_last_owned = vec_c_last.to_vec();
    // let vec_u_last_owned = vec_u_last.to_vec();
    // let vec_s_last_owned = vec_s_last.to_vec();

    println!("initial: ");
    let start = Instant::now();
    program.p.extend(vec_p_first_owned.clone());
    program.q.extend(vec_q_first_owned.clone());
    program.r.extend(vec_r_first_owned.clone());
    program.c.extend(vec_c_first_owned.clone());
    program.u.extend(vec_u_first_owned.clone());
    program.s.extend(vec_s_first_owned.clone());
    program.run();
    let duration = start.elapsed();
    println!("方法运行时间: {:?}", duration);


    println!("inc 1");
    let start = Instant::now();
    program_inc1.p.extend(vec_p.clone());
    program_inc1.q.extend(vec_q_first_owned.clone());
    program_inc1.r.extend(vec_r_first_owned.clone());
    program_inc1.c.extend(vec_c_first_owned.clone());
    program_inc1.u.extend(vec_u_first_owned.clone());
    program_inc1.s.extend(vec_s_first_owned.clone());
    program_inc1.run();
    let duration = start.elapsed();
    println!("方法运行时间: {:?}", duration);

    println!("inc 2");
    let start = Instant::now();
    program_inc2.p.extend(vec_p.clone());
    program_inc2.q.extend(vec_q.clone());
    program_inc2.r.extend(vec_r_first_owned.clone());
    program_inc2.c.extend(vec_c_first_owned.clone());
    program_inc2.u.extend(vec_u_first_owned.clone());
    program_inc2.s.extend(vec_s_first_owned.clone());
    program_inc2.run();
    let duration = start.elapsed();
    println!("方法运行时间: {:?}", duration);

    println!("inc 3");
    let start = Instant::now();
    program_inc3.p.extend(vec_p.clone());
    program_inc3.q.extend(vec_q.clone());
    program_inc3.r.extend(vec_r.clone());
    program_inc3.c.extend(vec_c_first_owned.clone());
    program_inc3.u.extend(vec_u_first_owned.clone());
    program_inc3.s.extend(vec_s_first_owned.clone());
    program_inc3.run();
    let duration = start.elapsed();
    println!("方法运行时间: {:?}", duration);

    println!("inc 4");
    let start = Instant::now();
    program_inc4.p.extend(vec_p.clone());
    program_inc4.q.extend(vec_q.clone());
    program_inc4.r.extend(vec_r.clone());
    program_inc4.c.extend(vec_c.clone());
    program_inc4.u.extend(vec_u_first_owned.clone());
    program_inc4.s.extend(vec_s_first_owned.clone());
    program_inc4.run();
    let duration = start.elapsed();
    println!("方法运行时间: {:?}", duration);

    println!("inc 5");
    let start = Instant::now();
    program_inc5.p.extend(vec_p.clone());
    program_inc5.q.extend(vec_q.clone());
    program_inc5.r.extend(vec_r.clone());
    program_inc5.c.extend(vec_c.clone());
    program_inc5.u.extend(vec_u.clone());
    program_inc5.s.extend(vec_s_first_owned.clone());
    program_inc5.run();
    let duration = start.elapsed();
    println!("方法运行时间: {:?}", duration);

    println!("inc 6");
    let start = Instant::now();
    program_inc6.p.extend(vec_p.clone());
    program_inc6.q.extend(vec_q.clone());
    program_inc6.r.extend(vec_r.clone());
    program_inc6.c.extend(vec_c.clone());
    program_inc6.u.extend(vec_u.clone());
    program_inc6.s.extend(vec_s.clone());
    program_inc6.run();
    let duration = start.elapsed();
    println!("方法运行时间: {:?}", duration);

    println!("galen run res:");
    println!("p{}", program_inc6.p.len());
    println!("q{}", program_inc6.q.len());
    println!("r{}", program_inc6.r.len());
    println!("c{}", program_inc6.c.len());
    println!("u{}", program_inc6.u.len());
    println!("s{}", program_inc6.s.len());
}
