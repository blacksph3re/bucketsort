// Necessary for insertion sort variant
//#![feature(linked_list_extras)]

extern crate getopts;
extern crate time;
extern crate byteorder;

use getopts::Options;
use std::env;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::collections::LinkedList;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

enum ToWorkers<'a> {
	Return,
	NewData(&'a str),
	Process,
}

enum FromWorkers {
	Finished,
}

fn chooseBucketSimple(threads: usize, firstchar: u8) -> usize {
	if firstchar < 33 {return 0;}
	let mut firstchar = firstchar as f32;
	firstchar -= 33.0;
	let retval = (firstchar / 94.0 * (threads as f32)) as usize;
	if retval >= threads {return threads-1;}
	retval
}

fn chooseBucket(threads: usize, line: String) -> usize {
0
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
	return byteorder::BigEndian::read_u64(&slice);

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
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = matches.opt_str("n");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    
    let mut NTHREADS = 1;
    
    if let Some(output) = output {
		if let Ok(n) = output.parse::<i32>() {
			NTHREADS = n;
		}
    }

    let NTHREADS = NTHREADS;

    // Create communication channels to and from the buckets
    let mut threads : Vec<Sender<ToWorkers>> = Vec::new();
    let (restx, resrx) = mpsc::channel();

    // Create NTHREAD buckets
    for id in 0..NTHREADS {
		// Create a channel for this bucket to send data and instructions
        let (tx, rx) = mpsc::channel();
		threads.push(tx);
		
		// Clone the result channel
		let cloned_restx = restx.clone();
		
		// Sort after finished variant
		// Fucking damn lot faster O.o
		thread::spawn(move || {
			let mut stored : Vec<&str> = Vec::new();
			let mut sorted = true;
			while let Ok(msg) = rx.recv() {
				match msg {
					ToWorkers::NewData(data) => {stored.push(data); sorted=false;},
					ToWorkers::Process => {stored.sort(); sorted=true;},
					ToWorkers::Return => {
						if(!sorted) {stored.sort();}

						//println!("Thread {} stored {} items", id, stored.len());
						
						for item in &stored {
							//println!("{}", item);
						}
		
						break;
					},
				}
			}
			cloned_restx.send(FromWorkers::Finished);

		});

		// Insertion sort variant
		/*
        thread::spawn(move || {
			let mut stored : LinkedList<String> = LinkedList::new();
					
			// Implement insertion sort for new data
			// Unfortunately Rust does not provide a insert method for their linked lists
			// Rendering linked lists absolutely unusable.
			
			while let Ok(ToWorkers::NewData(data)) = rx.recv() {
	
				let mut iter = stored.iter_mut();
				let mut count = 0;
				
				// Loop until the first element is greater
				loop {
					{
						let curitem = match iter.peek_next() {
							Some(curitem) => curitem,
							_ => break,
						};
	
						if data < *curitem {
							break;
						}
					}
					iter.next();
					count+=1;
				}
				iter.insert_next(data);
			}
			
			//println!("Thread {} stored {:?}", id, stored);
			
			for item in &stored {
				println!("{}", item);
			}
			
			cloned_restx.send(FromWorkers::Finished);
        });*/
    }
    
    println!("{} - Start reading files", time::now() - start);
   
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
    let file = BufReader::new(file).lines();
    for line in file {
		let line = line.unwrap();
		if(line.is_empty()) {continue;}
		
		let byteline = line.as_bytes();
		
		let bucket = chooseBucketSimple(threads.len(), byteline[0]);
		if bucket >= 0 && bucket < threads.len() {
			//threads[bucket].send(ToWorkers::NewData(line.as_ref()));
		} else {
			panic!("Index out of bound: {}", bucket);
		}
	}

	    
	println!("{} - Start processing", time::now() - start);

    
    // Start Processing
    for item in &threads {
		item.send(ToWorkers::Process);
    }
    
	println!("{} - Start gathering results", time::now() - start);

    
    // Gather results
    for item in &threads {
		item.send(ToWorkers::Return);
		resrx.recv();
    }
    
	println!("{} - Finished", time::now() - start);

}
