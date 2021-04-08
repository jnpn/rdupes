// learning rust

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

// impl Copy for Vec3 {
//     fn (&self) -> Vec3 {
// 	Vec3 { x, y, z };
//     }
// }

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
	// ...
	write!(f, "<Person {} {}>", self.name, self.age)
    }
}

impl Drop for Person {
    fn drop(&mut self) {
	// println!("Person.drop()");
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
}
