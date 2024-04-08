// fn main() {
//     let search_term = "picture";
//     let quote = "Every face, every shop, bedroom window, public-house, anddark squareis a picture feverishly turned--in search of what?It is the same with books. What do we seekthrough millions of pages?";
//     for line in quote.lines() {
//         if line.contains(search_term) {
//             println!("{}", line);    
//         }  
//     }
// }

// use regex::Regex;

// fn main() {
//     let re = Regex::new("picture").unwrap();
//     let quote = "Every face, every shop, bedroom window, public-house, anddark square is a picture feverishly turned--in search of what?It is the same with books. What do we seek through millions of pages?";
//     for line in quote.lines() {
//         let contains_substring = re.find(line);
//         match contains_substring {
//             Some(_) => println!("{}", line),
//             None => (),
//         }
//     }
// }

// use regex::Regex;
// use clap::{App,Arg};

// fn main() {

//     let args = App::new("grep-lite").version("1.0").about("searches for patterns")
//     .arg(Arg::with_name("pattern").help("The pattern to search for").takes_value(true).required(true))
//     .get_matches();

//     let pattern = args.value_of("pattern").unwrap();
//     let re = Regex::new(pattern).unwrap();

//     let quote = "Every face, every shop, bedroom window, public-house, and18 dark square is a picture feverishly turned--in search of what?19 It is the same with books. What do we seek through millions of pages?";

//     for line in quote.lines() {
//         match re.find(line) {
//             Some(_) => println!("{}", line),
//             None => (),
//         }
//     }
// }

use regex::Regex;
use clap::{App,Arg};

fn main() {

    let args = App::new("grep-lite").version("1.0").about("searches for patterns")
    .arg(Arg::with_name("pattern").help("The pattern to search for").takes_value(true).required(true))
    .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f)

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}