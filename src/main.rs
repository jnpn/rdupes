// learning rust

use std::fs;
use std::path::Path;
use std::ffi::OsStr;

use walkdir::WalkDir;
use std::collections::HashMap;
use md5::{Md5, Digest};
use glob::glob;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

fn add(u:Vec3, v:Vec3) -> Vec3 {
    Vec3 {
	x: u.x + v.x,
	y: u.y + v.y,
	z: u.z + v.z,
    }

}

use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "<Person {} {}>", self.name, self.age)
    }
}

impl Drop for Person {
    fn drop(&mut self) {
	println!("{}--", self);
    }
}

fn gen_hm(c:u32) -> HashMap<u32,String> {
    let mut m = HashMap::new();
    for i in 0..c {
	m.insert(i,format!("({})", i));
    }
    return m;
}
fn gen_person (age:u32) {
    let _p:Person = Person {
	name: "Jack".to_string(),
	age: age
    };
}


// fn main() {
//     let d = "/tmp/fdt/";
//     let files = WalkDir::new(d)
// 	.into_iter()
// 	.filter_map(|e| e.ok())
// 	.map(|e| {
// 	    let f = e.clone();
// 	    let p = e.path().clone();
// 	    let m = fs::metadata(p);
// 	    let d = Utc::now();
// 	    let h = hash(p);
// 	    return (f, m, d, h)
// 	})
// 	.collect::<Vec<_>>();
//     // .collect::<HashMap<WalkDir::DirEntry, &str>>();
//     for (e,m,d,h) in &files {
// 	let me = match m {
// 	    Err(_) => "?",
// 	    Ok(m) => if m.is_dir() { "directory" } else { "file" }
// 	};
// 	println!("{} : {}\t{}\t{:x}", e.path().display(), me, d, h);
//     }
//     println!("found {} file(s)", files.len());
// }

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
	// Ok(c) => m.update(String::from_utf8_lossy(c)),
	Ok(b) => m.update(b.as_bytes()),
	Err(_) => ()
    }
    // let d = m.finalize().as_slice().iter().collect::<Vec<&u8>>();
    // for u in m.finalize().as_slice().iter() {
    // 	println!("{}", u);
    // }
    // let s = m.finalize().as_slice(); // .iter().collect::<Vec<&u8>>();
    // let t = String::from(s);

    // let s = m.finalize().as_slice().iter()
    // 	.collect::<Vec<&u8>>();

    // let hs = String::from_utf8_lossy(d);
    // return (String::from("0x01393092"), f.to_string());
    return (format!("{:?}", m.finalize().as_slice().iter()
	.collect::<Vec<&u8>>()), f.to_string());
}



// fn walkmap(root:String, keyer:fn(String) -> String) -> HashMap<String, Vec<String>> {
//     // [(str, path)]
//     let files = WalkDir::new(root)
// 	.into_iter()
// 	.filter_map(|e| e.ok())
// 	.map(md5)
// 	.collect::<Vec<_>>();
//     let mut m = HashMap::new();
//     m.insert(root, vec!["foo".to_string(), "bar".to_string()]);
//     return m;
// }

fn main() {
    let a = Vec3 { x:1.0, y:1.0, z:1.0 };
    println!("[vec]\t{:?}", a);
    let b = a;
    println!("[vadd]\t{:?}", add(a,b));

    let p = Person {
	name: String::from("Alan"),
	age: 107
    };
    println!("[person]\t{:?}", p);
    for i in 50..100 {
	gen_person(i);
    }
    println!("[nop]\tyolo {}", "!".to_string());

    let m = gen_hm(12);
    println!("[gen_hm]\t{:?}", m);

    let mut m:HashMap<String, Vec<String>> = HashMap::new();
    m.insert("0x2943083".to_string(), vec!["/a/b/c".to_string(), "/b/c/d".to_string()]);
    m.insert("0x2940003".to_string(), vec!["/a/b".to_string(), "/b/c".to_string()]);
    m.insert("0x2000083".to_string(), vec!["/a/c".to_string(), "/d".to_string()]);

    let k = "0x2000083";
    let s = match m.get(k) {
	Some(v) => format!("{} -> {:?}", k, v),
	None => format!("nil")
    };
    println!("[map]\t{:?}", m);
    println!("[match]\t{}", s);

	// let (a,b) = md5(std::path::Path::new("/home/noob/.aliases"));
	// println!("{} # {}", b, a);

    // for e in glob("/tmp/fdt/*").expect("glob failed") {
    for e in glob("/home/noob/bin/*").expect("glob failed") {
	match e {
	    Ok(f) => {
		let (a,b) = md5(std::path::Path::new(&f.display().to_string()));
		println!("{} # {}", b, a);
	    },
	    Err(e) => println!("{:?}", e)
	}
    }

    //walkmap("Hello".to_string(), |s| s);
}
