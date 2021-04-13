// learning rust

use std::fs;
use std::path::Path;

use walkdir::WalkDir;
use std::collections::HashMap;
use md5::{Md5, Digest};

use data_encoding::HEXUPPER;

use ansi_term::Colour;

fn md5(p:&Path) -> (String,String) {
    let mut m = Md5::new();
    let c = fs::read_to_string(p);
    let f = match p.file_name() {
	Some(n) => match n.to_str() {
	    Some(s) => s,
	    None => "??"
	},
	None => "?",
    };
    match c {
	Ok(b) => m.update(b.as_bytes()),
	Err(_) => ()
    }

    let a = &m.finalize()[..];
    let b = a.into_iter().collect::<Vec<&u8>>();
    // let c = b.iter().map(|r| *r).collect::<[u8]>();
    let mut r:[u8; 16] = [0 ; 16];
    let mut i = 0;
    for u in b {
	r[i] = *u;
	i += 1;
    }
    let s = String::from(HEXUPPER.encode(&r));
    return (f.to_string(), s);
}

fn walkmap(root:String, keyer:fn(walkdir::DirEntry) -> (String, String)) -> HashMap<String, Vec<String>> {

    println!("{}", Colour::Green.paint(format!("\n-- scanning {} ...\n", &root)));
    let mut total = 0;

    // https://stackoverflow.com/questions/7733808/vt100-escape-sequence-to-remove-already-printed-newline
    // for println ansi line clear

    let files: Vec<(String, String)> = WalkDir::new(root)
	.into_iter()
	.filter_map(Result::ok)
	.map(keyer)
	.map(|o| { total += 1; println!("\x1b[A\x1b[K{} file(s)", total); return o; })
	.collect::<Vec<_>>();

    println!("{}", Colour::Green.paint("\n-- building hash dup\n..\x08!."));

    let mut m:HashMap<String, Vec<String>> = HashMap::new();

    for (f,h) in files {
	match m.get_mut(&h) {
	    Some(v) => v.push(f),
	    None => { &m.entry(h).or_insert(vec![f]); println!("\x1b[A\x1b[K{} entries", &m.len())},
	}
    }

    println!("{}", Colour::Green.paint("\n-- done\n"));

    return m;
}

fn main() {
    for a in std::env::args().skip(1) {
	println!("deduping {:?}", a);
	for (k,v) in walkmap(a, |e| md5(e.path())) {
	    println!("{} -> {:?}", k, v);
	}
    }
}
