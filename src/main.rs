// learning rust

use std::fs;
use std::path::Path;

use walkdir::WalkDir;
use std::collections::HashMap;
use md5::{Md5, Digest};
use glob::glob;

use data_encoding::HEXUPPER;

#[derive(Debug)]
struct Pair<A> {
    a:A,
    b:A,
}

fn g() {
    let p = Pair { a:0,b:1 }; // { a:0, b:1 };
    println!("({},{})", p.a, p.b);
}

fn f(x: &u8, f: fn(u8) -> u8) -> u8 {
    return f(*x + *x);
}

fn gen_hm(c:u32) -> HashMap<u32,String> {
    let mut m = HashMap::new();
    for i in 0..c {
	m.insert(i,format!("({})", i));
    }
    return m;
}

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
    let mut r:[u8; 16] = [0 ; 16];
    let mut i = 0;
    for u in &b {
	r[i] = **u;
	i += 1;
    }
    let s = String::from(HEXUPPER.encode(&r));
    return (f.to_string(), s);
}

fn walkmap(root:String, keyer:fn(String) -> String) -> HashMap<String, Vec<String>> {
    // Vec<(String,  String)>
    let files: Vec<(String, String)> = WalkDir::new(root)
	.into_iter()
	.filter_map(Result::ok)
	.map(|e| md5(e.path()))
	.collect::<Vec<_>>();
    let mut m:HashMap<String, Vec<String>> = HashMap::new();

    // m.insert(keyer("@@@".to_string()), vec!["foo".to_string(), "bar".to_string()]);
    println!("{}", keyer("@@@".to_string()));
    // for (n,v) in &files {
    // 	println!("{}:-{:?}", n, v);
    // }

    // let mut n:HashMap<String, u32> = HashMap::new();
    // for (_,h) in files {
    // 	if n.contains_key(&h) {
    // 	    n.entry(h).or_insert(100);
    // 	} else {
    // 	    n.insert(h, 1);
    // 	}
    // }
    
    for (f,h) in files {
	match m.get_mut(&h) {
	    Some(v) => v.push(f),
	    None => { m.entry(h).or_insert(vec![f]); },
	}
    }

    return m;
}

fn main() {
    let m = gen_hm(12);
    println!("[gen_hm]\t{:?}", m);
    println!("----");

    let (a,b) = md5(std::path::Path::new("/home/noob/.aliases"));
    println!("[1hash] {} # {}", a, b);
    println!("----");

    let root = "/home/noob/bin/*";

    for e in glob(root).expect("glob failed") {
	match e {
	    Ok(p) => {
		let (a,b) = md5(p.as_path());
		println!("{} # {}", a, b);
	    },
	    Err(e) => println!("{:?}", e)
	}
    }

    // match glob(root) {
    // 	Ok(paths) => {
    // 	    for p in paths {
    // 		match p {
    // 		    Ok(pb) => {
    // 		    	let (a,b) = md5(pb.as_path());
    // 			println!("{} {}", a, b);
    // 		    },
    // 		    Err(f) => println!("{:?}", f)
    // 		}
    // 	    };
    // 	}
    // 	Err(e) => println!("{:?}", e),
    // };
    
    println!("----");
    println!("[WALK]");
    for (k,v) in walkmap("/home/noob/bin/".to_string(), |s| s) {
	println!("{} -> {:?}", k, v);
    }

    g();

    let x:u8 = 23;
    let y = f(&x, |x| x+x);
    println!("{} -> {}", x, y);

    println!("----");
    println!("bye.");
}
