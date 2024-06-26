// #[derive(Debug)]

// struct File {
//     name: String,
//     data: Vec<u8>,
// }

// fn main() {
//     let f1 = File {
//         name: String::from("f1.txt"),
//         data: Vec::new(),
//     };

//     let f1_name = &f1.name;
//     let f1_length = &f1.data.len();

//     println!("{:?}", f1);
//     println!("{} is {} bytes long", f1_name, f1_length);
// }


// #![allow(unused_variables)]
// type File = String;

// fn open(f: &mut File) -> bool {
//     true
// }

// fn close(f: &mut File) -> bool {
//     true
// }

// #[allow(dead_code)]
// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!()
// }

// fn main() {
//     let mut f1 = File::from("f1.txt");
//     open(&mut f1);
//     // read(f1, vec![]);
//     close(&mut f1);
// }

// #![allow(unused_variables)]
// #[derive(Debug)]

// struct File {
//     name: String,
//     data: Vec<u8>,
// }

// fn open(f: &mut File) -> bool {
//     true
// }

// fn close(f: &mut File) -> bool {
//     true
// }

// fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
//     let mut tmp = f.data.clone();
//     let read_length = tmp.len();
//     save_to.reserve(read_length);
//     save_to.append(&mut tmp);
//     read_length
// }

// fn main() {
//     let mut f2 = File {
//         name: String::from("2.txt"),
//         data: vec![114, 117, 115, 116, 33],
//     };
//     let mut buffer: Vec<u8> = vec![];

//     open(&mut f2);
//     let f2_length = read(&f2, &mut buffer);
//     close(&mut f2);

//     let text = String::from_utf8_lossy(&buffer);

//     println!("{:?}", f2);
//     println!("{} is {} bytes long", &f2.name, f2_length);
//     println!("{}", text)
// }

// #[derive(Debug)]

// struct File {
//     name: String,
//     data: Vec<u8>,
// }

// impl File {
//     fn new(name: &str) -> File {
//         File {
//             name: String::from(name),
//             data: Vec::new(),
//         }
//     }
// }

// fn main() {
//     let f3 = File::new("f3.txt");
//     let f3_name = &f3.name;
//     let f3_length = f3.data.len();
//     println!("{:?}", f3);
//     println!("{} is {} bytes long", f3_name, f3_length);
// }


// #![allow(unused_variables)]
// #[derive(Debug)]

// struct File {
//     name: String,
//     data: Vec<u8>,
// }

// impl File {
//     fn new(name: &str) -> File {
//         File {
//             name: String::from(name),
//             data: Vec::new(),
//         }
//     }

//     fn new_with_data(name: &str, data: &Vec<u8>) -> File {
//         let mut f = File::new(name);
//         f.data = data.clone();
//         f
//     }

//     fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
//         let mut tmp = self.data.clone();
//         let read_length = tmp.len();
//         save_to.reserve(read_length);
//         save_to.append(&mut tmp);
//         read_length
//     }
// }

// fn open(f: &mut File) -> bool {
//     true
// }

// fn close(f: &mut File) -> bool {
//     true
// }

// fn main() {
//     let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33];
//     let mut f3 = File::new_with_data("2.txt", &f3_data);
//     let mut buffer: Vec<u8> =vec![];
//     open(&mut f3);
//     let f3_length = f3.read(&mut buffer);
//     close(&mut f3);

//     let text = String::from_utf8_lossy(&buffer);
//     println!("{:?}", f3);
//     println!("{} is {} bytes long", &f3.name, f3_length);
//     println!("{}", text);
// }

#![allow(unused_variables)]

#[derive(Debug)]
struct File;

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec!();
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}