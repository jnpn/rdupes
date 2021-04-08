use std::fs;
use std::path;
// use std::fs::File;
use std::collections::HashMap;
// use std::fmt;

use md5::{Md5, Digest};
use chrono::{Utc};
use walkdir::WalkDir;

// fn hfill<A,B>(items: Vec<(A,B)>) -> HashMap<A,B> {
//     let mut  h = HashMap::new();
//     for (a,b) in items {
// 	h.insert(a,b);
//     }
//     return h;
// }

fn h<'a>(i:u32, s:str) -> HashMap<u32,&'a str> {
    let mut h = HashMap::new();
    h.insert(s, i);
    return h;
}

// struct Entries(pub Vec<walkdir::DirEntry>);

// impl std::fmt::Display for Entries {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 	writeln!(f, "<>");
// 	f
//     }
// }

fn hash(p: path::Path) {
    // let mut fi = File::open(p)?; // .expect("unable to open the file");
    // let mut c = String::new();
    // fi.read_to_string(&mut c).expect("Unable to read the file");

    let c = fs::read_to_string(p).expect("read?");
    let mut md = Md5::new();
    md.update(c);
    return md.finalize();
}

fn main() {
    let d = "/tmp/fdt/";
    let files = WalkDir::new(d)
	.into_iter()
	.filter_map(|e| e.ok())
	.map(|e| {
	    let f = e.clone();
	    let p = e.path().clone();
	    let m = fs::metadata(p);
	    let d = Utc::now();
	    let h = hash(p);
	    return (f, m, d, h)
	})
	.collect::<Vec<_>>();
    // .collect::<HashMap<WalkDir::DirEntry, &str>>();
    for (e,m,d,h) in &files {
	let me = match m {
	    Err(_) => "?",
	    Ok(m) => if m.is_dir() { "directory" } else { "file" }
	};
	println!("{} : {}\t{}\t{:x}", e.path().display(), me, d, h);
    }
    println!("found {} file(s)", files.len());
}
