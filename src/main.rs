extern crate image;
extern crate rand;
extern crate itertools;

use self::rand::Rng;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::Error;

use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::fs;

//
//use itertools::Itertools;

fn main() {

//    test_write();
//    test_read();
//    test_zip();
    let geno_len = 2000;
//
//    let genotype: Vec<u8> = (0..geno_len).map(|_| rand::thread_rng().gen::<u8>()).collect();
//
//    test_get_geno(16);
    make_gray_pics(geno_len,&test_map2, 600, 600,40);
//    get_pic_nums("./imgs/");
//    reprod();
}



pub fn get_pic_nums(loc: &str) -> Vec<usize> {

    let mut nums: Vec<usize> = Vec::new();
    let paths = fs::read_dir(loc).unwrap();


    for path in paths {
        let p =  path.unwrap().file_name().into_string().unwrap();
//            let p1 = p[3..];
        println!("Name: {:?}",p);

        let mut f_name_parts = p.split(".").next().unwrap().split("-");
        f_name_parts.next(); //throw out 'pic'

        let f_num = f_name_parts.next();

        match f_num {
            None => (),
            Some(num) => nums.push(num.parse::<usize>().unwrap() ),
        }

//            println!("{:?} fnum", f_num);
    }
    println!("nums: {:?}", nums);
    nums

}


pub fn reprod() {
    let pics_nums = get_pic_nums("./imgs/");

    let max_num = pics_nums.iter().max().unwrap();
    let mut new_pic_num = max_num + 1;

    for current_pic_num in pics_nums.iter() {
        let mut genotype = test_get_geno(*current_pic_num);
        match genotype {
            None => println!("Couldnt find genotype for number {}",current_pic_num),
            Some(mut genotype) => {{simple_point_mut(&mut genotype)}
                make_a_gray_pic(&genotype, &test_map2, 600, 600, new_pic_num);
                new_pic_num += 1}
        }

    }


}

pub fn simple_point_mut(genotype: &mut Vec<u8>) {
    let mut_point = rand::thread_rng().gen_range::<usize>(0,genotype.len());
    genotype[mut_point] = rand::thread_rng().gen::<u8>();
}


pub fn test_get_geno(num: usize) -> Option<Vec<u8>> {
    let mut test_file = File::open("./imgs/genos.txt").unwrap();

    let f = BufReader::new(test_file);
// read the whole file
    for (i, line) in f.lines().enumerate(){
        println!("line {} is {:?}",i, line);
        if i == num {
            let geno: Vec<u8> = line.unwrap().split("\t").map(|x: &str| x.parse::<u8>().unwrap()).collect();
            println!("geno is {:?}", geno);
            return Some(geno);
//            make_a_gray_pic(&geno, &test_map2, 600, 600);
        }
    }
    return None;

//    test_file.read_to_end(&mut l).unwrap();
//    println!("read l is {:?}", l);
}

pub fn test_write(){
    let mut l: Vec<u8> = vec![234, 66, 90, 56];

//    let l_str = l.join("\t");

//    let l_str = ["234", "66", "90", "56"].join("\t");

    let mut l_str: Vec<String> = l.iter().map(|x| x.to_string()).collect();
    let mut l_str = l_str.join("-");
    l_str.push_str("\n");


    let mut test_file = File::create("./test.txt").unwrap();
    test_file.write(l_str.as_bytes());

    println!("l is {:?}, l_str is {:?}", l, l_str);
}

pub fn write_to_file(f: &mut File, genotype: &Vec<u8>) {
    let mut string_vec: Vec<String> = genotype.iter().map(|x| x.to_string()).collect();
    let mut string_vec = string_vec.join("\t");
    string_vec.push_str("\n");
    f.write(string_vec.as_bytes());
}

pub fn test_zip(){
    let l1: Vec<u8> = vec![234, 66, 90, 56];
    let l2: Vec<u8> = vec![4, 696, 990, 806];
    let l3: Vec<(&u8,u8)> = l1.iter().zip(l2).collect();
    println!("{:?}", l3);
}

pub fn test_read(){
    let mut test_file = File::open("./imgs/genos.txt").unwrap();
    let mut l = Vec::new();

//    l.push(5);
//    let f = BufReader::new(test_file);
//// read the whole file
//    for line in f.lines(){
//        println!("line is {:?}", line);
//    }
    test_file.read_to_end(&mut l);
//    test_file.read_to_end(&mut l).unwrap();
    println!("read l is {:?}", l);
}

pub fn make_gray_pics(geno_len: usize, gp_map: &Fn(&Vec<u8>, u32, u32)->u8, width: u32, height: u32, count: usize) {
    let mut geno_file = File::create("./imgs/genos.txt").unwrap();
    for pic_id in 0..count {
        let genotype: Vec<u8> = (0..geno_len).map(|_| rand::thread_rng().gen::<u8>()).collect();
        write_to_file(&mut geno_file, &genotype);
        let mut imgbuf = image::ImageBuffer::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let i = gp_map(&genotype, x, y);

            // Create an 8bit pixel of type Luma and value i
            // and assign in to the pixel at position (x, y)
            *pixel = image::Luma([i]);
        }


        let filename = format!("./imgs/pic-{}.png",pic_id);
        // Save the image as “fractal.png”
        let ref mut fout = File::create(filename).unwrap();


        // We must indicate the image's color type and what format to save as
        image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();
    }
}

pub fn make_a_gray_pic(genotype: &Vec<u8>, gp_map: &Fn(&Vec<u8>, u32, u32)->u8, width: u32, height: u32, file_num: usize) {

    let mut imgbuf = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = gp_map(&genotype, x, y);

        // Create an 8bit pixel of type Luma and value i
        // and assign in to the pixel at position (x, y)
        *pixel = image::Luma([i]);
    }


    let filename = format!("./imgs2/pic{}.png",file_num);
    // Save the image as “fractal.png”
    let ref mut fout = File::create(filename).unwrap();


    // We must indicate the image's color type and what format to save as
    image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();

}



pub fn test_map1(genotype: &Vec<u8>, x: u32, y: u32) -> u8 {
    let mut res = 0;

    for num in genotype {
        let temp = res as u32+ *num as u32;
        res = (temp%255) as u8;
    }

    if x > y {return ( ( ((x as f32).sin() + 1.0)/2.0)*(res as f32)) as u8  }
    return ( ( ((y as f32).cos() + 1.0)/2.0)*(res as f32)) as u8
}



pub fn test_map2(genotype: &Vec<u8>, x: u32, y: u32) -> u8 {
    let op_count = 10;
    const REGS: usize = 16;

    let reg_count = REGS as u8;

    let mut regs = [0.0; REGS];

    for i in 2..REGS {
        regs[i] = -256.0 + i as f32*512.0/REGS as f32;
    }

    regs[0] = x as f32;
    regs[1] = y as f32;

    let instr_len = 4;
    let instr_count = genotype.len() / instr_len;

    let mut skip_next = false;


    for index in 0..instr_count {

        let target_i = 2 + (genotype[index] % (reg_count-2)) as usize; //dont select first 2

        let source1_val = regs[(genotype[index + REGS] % 6) as usize];
        let source2_val = regs[(genotype[index + REGS] % 6) as usize];

        if skip_next {
            skip_next = false;
            continue
        }


        match genotype[index + 2] % op_count { // 0..3
            0 => { regs[target_i] = source1_val.sin() },

            1 => { regs[target_i] = source1_val.cos() },

            2 => { regs[target_i] = source1_val*source2_val  },

            3 => { regs[target_i] = source1_val + source2_val },

            4 => { regs[target_i] = if source2_val==0.0 {0.0} else {source1_val/source2_val}  },

            5 => { regs[target_i] = source1_val - source2_val },

            6 => { regs[target_i] = (-source1_val).exp(); if regs[target_i] > 256.0 {regs[target_i] = 256.0}},

            7 => { let mut v = -(source1_val-300.0).powf(2.0);
                    regs[target_i] = v.exp() },
            8 => {skip_next = source1_val > source2_val },
            9 => {skip_next = source1_val < source2_val },
            _ => panic!("Invalid op code! {}", genotype[index + 2] % op_count)
        }


    }

//    if regs[2] < 0.0 {regs[2] = 0.0}
    println!("REgs are {:?}", regs);
    let ret_val = ( (regs[2]*1.0) % 255.0).abs() as u8;
    println!("REt val: {}", ret_val);

    ret_val


}

pub fn test_map3(genotype: &Vec<u8>, x: u32, y: u32) -> u8 {
    let op_count = 10;
    const REGS: usize = 16;

    let reg_count = REGS as u8;

    let mut regs = [0.0; REGS];

    regs[0] = x as f32;
    regs[1] = y as f32;

    let instr_len = 4;
    let instr_count = genotype.len() / instr_len;


    for index in 0..instr_count {

        let target_i = 2 + (genotype[index] % (reg_count-2)) as usize; //dont select first 2

        let source1_val = regs[(genotype[index + REGS] % 6) as usize];
        let source2_val = regs[(genotype[index + REGS] % 6) as usize];


        match genotype[index + 2] % op_count { // 0..3
            0 => { regs[target_i] = source1_val.sin() },

            1 => { regs[target_i] = source1_val.cos() },

            2 => { regs[target_i] = source1_val*source2_val  },

            3 => { regs[target_i] = source1_val + source2_val },

            4 => { regs[target_i] = if source2_val==0.0 {0.0} else {source1_val/source2_val}  },

            5 => { regs[target_i] = source1_val - source2_val },

            6 => { regs[target_i] = source1_val.exp(); if regs[target_i] > 256.0 {regs[target_i] = 256.0}},

            7 => { let mut v = (source1_val-source2_val).powf(2.0);
                if v > 6.0 {v=6.0}
                regs[target_i] = v.exp() },
            8 => { let mut v = (source1_val-source2_val).powf(2.0);
                if v > 6.0 {v=6.0}
                regs[target_i] = v.exp() },
            9 => { let mut v = (source1_val-source2_val).powf(2.0);
                if v > 6.0 {v=6.0}
                regs[target_i] = v.exp() },

            _ => panic!("Invalid op code! {}", genotype[index + 2] % op_count)
        }


    }

//    if regs[2] < 0.0 {regs[2] = 0.0}
    println!("REgs are {:?}", regs);

    ( (regs[2]*1.0).abs()  % 255.0)as u8
}