extern crate getopts;
extern crate time;
extern crate byteorder;

use getopts::Options;
use std::env;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const OUTPUT_TIMES : bool = true;
const MIN_ENTRY : u64 = 9325100404908321;
const MAX_ENTRY : u64 = 35604928818740863;
const LINEAR_DISTRIBUTION : bool = false;
// Generated like that:
/*	let MIN_ENTRY : u64 = bytearr_to_int64(&[0x21u8, 0x21u8, 0x21u8, 0x21u8, 0x21u8, 0x21u8, 0x21u8]);
	let MAX_ENTRY : u64 = bytearr_to_int64(&[0x7Eu8, 0x7Eu8, 0x7Eu8, 0x7Eu8, 0x7Eu8, 0x7Eu8, 0x7Eu8])+1;
	
	println!("{}", MIN_ENTRY);
	println!("{}", MAX_ENTRY);*/

fn choose_bucket(threads: usize, line: u64) -> usize {
	if line < MIN_ENTRY {return 0;}
	if line >= MAX_ENTRY {return threads;}

	// linear distribution
	// TODO take some load off the first buckets so they can write early
	if LINEAR_DISTRIBUTION {
		return (((line - MIN_ENTRY) * (threads as u64)) / (MAX_ENTRY - MIN_ENTRY)) as usize;
	} else {
		let resolution = 500;
		let tmp: u64  = ((line - MIN_ENTRY) * (resolution as u64)) / (MAX_ENTRY - MIN_ENTRY);
		let mut bucket: f64 = (tmp as f64) / (resolution as f64);
		bucket = bucket.powf(0.7) * (threads as f64);
		if bucket < 0.0 {bucket=0.0;}
		let mut bucket = bucket as usize;
		if bucket >= threads {bucket = threads - 1;}
		return bucket;
	}
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn bytearr_to_int64(slice: &[u8]) -> u64 {
use std::cmp;
use byteorder::ByteOrder;

	
	// If lengths mismatch, add leading zeros or omit parts of input
	if slice.len() != 8 {
		let mut arr : [u8; 8] = [0; 8];
		let offset = cmp::max(8-slice.len(), 0);
		for i in 0..cmp::min(8, slice.len()) {
			arr[i+offset] = slice[i];
		}
		return byteorder::BigEndian::read_u64(&arr);
	}
	
	// Transmute to int according to Bigendian byteorder (for comparison)
	byteorder::BigEndian::read_u64(&slice)

}

fn int64_to_bytearr(input : u64) -> [u8; 8] {
use byteorder::ByteOrder;
	let mut arr : [u8; 8] = [0; 8];
	
	// Transmute to bytes 
	byteorder::BigEndian::write_u64(&mut arr, input);
	arr
}


fn main() {
	let start = time::now();

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("n", "", "set number of threads", "NUMBER");
    opts.optopt("o", "", "set output file", "FILE");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    
    let mut nthreads : usize = 1;
	let param = matches.opt_str("n");
    if let Some(param) = param {
		if let Ok(n) = param.parse::<usize>() {
			nthreads = n;
		}
    }
    let nthreads = nthreads;
    
    let param = matches.opt_str("o");
    let output = if let Some(param) = param {
		param.clone()
    } else {String::from("output")};
    
    
    if OUTPUT_TIMES {println!("{} - Start reading files", time::now() - start);}
   
    // Read the file input
    let path = Path::new(&input);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };
    

	// Read File and send to buckets
	let mut contents = String::new();
	if let Err(why) = file.read_to_string(&mut contents) {
		panic!("could not read file: {}", Error::description(&why));
	}
	
	
	if OUTPUT_TIMES {println!("{} - Converting to u64", time::now() - start);}
	let mut buckets  = vec![Vec::<u64>::with_capacity(contents.len()/6/nthreads); nthreads];  // Only divide by 6 to have some extra capacity to avoid full vector copies

	
	let contents = contents;
	
	// Idea: do this parallel?
	// Buckets would have to be seperated and joined then.
	for line in contents.lines() {
		if line.is_empty() {continue;}
		let line = bytearr_to_int64(line.as_bytes());
		let bucket = choose_bucket(nthreads, line);

		if /*bucket >= 0 &&*/ bucket < nthreads {
			buckets[bucket].push(line);
		} else {
			panic!("Index out of bound: {}", bucket);
		}
	}

	    
	if OUTPUT_TIMES {println!("{} - Start processing", time::now() - start);}

	// Communicate through channels
	let mut threads = Vec::new();
	let (restx, resrx) = mpsc::channel();
	
    for i in (0..nthreads).rev() {
		let (tx, rx) : (Sender<Arc<Mutex<File>>>, Receiver<Arc<Mutex<File>>>) = mpsc::channel();
		threads.push(tx);
		
		// Clone the result channel
		let cloned_restx = restx.clone();
		let mut threadbucket = buckets.pop().unwrap();
		thread::spawn(move || {
			let mut output = String::with_capacity(threadbucket.len()*8);
			
			let sortstart = time::now();
			threadbucket.sort();
			let parsestart = time::now();
			for item in &threadbucket {
				let bytes = &int64_to_bytearr(*item)[1..];
				
				output.push_str(String::from_utf8_lossy(bytes).to_mut());
				output.push_str("\n");
			}
			let parseend = time::now();

			let output = output;
			
			let pointer = rx.recv().unwrap();
			let writestart = time::now();

			let mut file = pointer.lock().unwrap();
			file.write(output.as_bytes()).expect("could not write to file");
			if !LINEAR_DISTRIBUTION {file.flush().expect("could not write to file");}
			let now = time::now();
			if OUTPUT_TIMES {println!("Thread {} load: {} items (sort - {}, parse - {}, write - {})", i, threadbucket.len(), parsestart-sortstart, parseend-parsestart, now-writestart);}
			
			cloned_restx.send(()).expect("sending to main thread failed");
		});
    }
    threads.reverse();
	if OUTPUT_TIMES {println!("{} - Start gathering results", time::now() - start);}

	let path = Path::new(&output);
    let display = path.display();
	// Open the path in read-only mode, returns `io::Result<File>`
    let file = Arc::new(Mutex::new(match File::create(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
		Ok(file) => file,
    }));
    
    // Gather results
    for item in &threads {
		item.send(file.clone()).expect("send to thread failed");
		resrx.recv().expect("receive from thread failed");
    }
    drop(file);
	if OUTPUT_TIMES {println!("{} - Finished", time::now() - start);}
	
}
