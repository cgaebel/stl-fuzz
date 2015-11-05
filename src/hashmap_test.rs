use sexp::Sexp;
use commands::{command_0, command_1, command_2, command_vec};

use std::collections::HashMap;

//  { initial }
//  (new)
//  (default)
//  (with_capacity cap)
fn run_initial_command(first_command: &Sexp) -> Result<HashMap<i64, i64>, ()> {
  match command_0(first_command) {
    Ok("new")     => { println!("new"); Ok(HashMap::new()) },
    Ok("default") => { println!("default"); Ok(Default::default()) },
    Ok(_) => Err(()),
    Err(_) => {
      match try!(command_1(first_command)) {
        ("with_capacity", capacity) => {
          println!("with_capacity {}", capacity);
          Ok(HashMap::with_capacity((capacity & 0xFFFF) as usize))
        },
        _ => Err(())
      }
    }
  }
}

//  { 0 arg }
//  (capacity)
//  (shrink_to_fit)
//  (print_keys)
//  (print_values)
//  (print)
//  (fmt)
//  (len)
//  (clear)
//  (clone)
fn run_0_command(command: &Sexp, hm: &mut HashMap<i64, i64>) -> Result<(), ()> {
  match try!(command_0(command)) {
    "capacity" => println!("capacity: {}", hm.capacity()),
    "shrink_to_fit" => { println!("shrink_to_fit"); hm.shrink_to_fit(); },
    "print_keys" => {
      print!("keys: [ ");
      for (i, k) in hm.keys().enumerate() {
        if i == 0 { print!(  "{}", k) }
        else      { print!(", {}", k) }
      }
      println!(" ]");
    },
    "print_values" => {
      print!("values: [ ");
      for (i, v) in hm.values().enumerate() {
        if i == 0 { print!(  "{}", v) }
        else      { print!(", {}", v) }
      }
      println!(" ]");
    },
    "print" => {
      print!("tuples: [ ");
      for (i, kv) in hm.iter().enumerate() {
        if i == 0 { print!(  "{:?}", kv) }
        else      { print!(", {:?}", kv) }
      }
      println!(" ]");
    },
    "len" => println!("len: {}", hm.len()),
    "clear" => { println!("clear"); hm.clear(); },
    "fmt" => println!("fmt: {:?}", hm),
    "clone" => { println!("clone"); let _ = hm.clone(); }
    _ => return Err(())
  };

  Ok(())
}

//  { 1 arg }
//  (reserve n)
//  (get k)
//  (get_mut k)
//  (drain_and_take n)
//  (contains_key k)
//  (remove k)
fn run_1_command(command: &Sexp, hm: &mut HashMap<i64, i64>) -> Result<(), ()> {
  match try!(command_1(command)) {
    ("reserve", n) => { println!("reserve {}", n & 0xFFFF); hm.reserve((n & 0xFFFF) as usize); },
    ("get", k) => { println!("get {}: {:?}", k, hm.get(&k)) },
    ("get_mut", k) => println!("get_mut {}: {:?}", k, hm.get_mut(&k)),
    ("drain_and_take", n) => {
      println!("drain_and_take {}", n);
      for _ in hm.drain().take((n & 0xFFFF) as usize) {
        // do nothing.
      }
    },
    ("contains_key", k) => println!("contains_key {}: {}", k, hm.contains_key(&k)),
    ("remove", k) => println!("remove {}: {:?}", k, hm.remove(&k)),
    _ => return Err(()),
  }

  Ok(())
}

//  { 2 arg }
//  (insert k v)
fn run_2_command(command: &Sexp, hm: &mut HashMap<i64, i64>) -> Result<(), ()> {
  match try!(command_2(command)) {
    ("insert", k, v) => println!("insert {} {}: {:?}", k, v, hm.insert(k, v)),
    _ => return Err(()),
  }

  Ok(())
}

//  { vec arg }
//  (extend ((k v) (k v) ...))
fn run_vec_command(command: &Sexp, hm: &mut HashMap<i64, i64>) -> Result<(), ()> {
  match try!(command_vec(command)) {
    ("extend", with) => {
      print!("extend {:?}: ", with);
      hm.extend(with.into_iter());
      println!("{:?}", *hm);
    },
    _ => return Err(())
  }

  Ok(())
}

// Try commands of different argument lengths.
fn run_command(command: &Sexp, hm: &mut HashMap<i64, i64>) -> Result<(), ()> {
  match run_0_command(command, hm) {
    Ok(()) => Ok(()),
    Err(()) =>
      match run_1_command(command, hm) {
        Ok(()) => Ok(()),
        Err(()) =>
          match run_2_command(command, hm) {
            Ok(()) => Ok(()),
            Err(()) =>
              run_vec_command(command, hm)
          }
      }
  }
}

pub fn go(commands: Sexp) -> Result<HashMap<i64, i64>, ()> {
  match commands {
    Sexp::Atom(_) => Err(()),
    Sexp::List(ref commands) => {
      if commands.is_empty() { return Err(()) }
      let mut hm = try!(run_initial_command(&commands[0]));
      for c in &commands[1..] {
        try!(run_command(c, &mut hm));
      }
      Ok(hm)
    }
  }
}
