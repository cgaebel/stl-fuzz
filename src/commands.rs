use sexp::{Sexp, Atom};

pub fn command_0(command: &Sexp) -> Result<&str, ()> {
  match *command {
    Sexp::Atom(_) => Err(()),
    Sexp::List(ref xs) => {
      if xs.len() != 1 { return Err(()) }
      match xs[0] {
        Sexp::Atom(Atom::S(ref s)) => Ok(&*s),
        _ => Err(())
      }
    }
  }
}

pub fn command_1(command: &Sexp) -> Result<(&str, i64), ()> {
  match *command {
    Sexp::Atom(_) => Err(()),
    Sexp::List(ref xs) => {
      if xs.len() != 2 { return Err(()) }
      let command_name =
        match xs[0] {
          Sexp::Atom(Atom::S(ref s)) => &*s,
          _ => return Err(())
        };
      let arg1 =
        match xs[1] {
          Sexp::Atom(Atom::I(i)) => i,
          _ => return Err(()),
        };
      Ok((command_name, arg1))
    }
  }
}

pub fn command_2(command: &Sexp) -> Result<(&str, i64, i64), ()> {
  match *command {
    Sexp::Atom(_) => Err(()),
    Sexp::List(ref xs) => {
      if xs.len() != 3 { return Err(()) }
      let command_name =
        match xs[0] {
          Sexp::Atom(Atom::S(ref s)) => &*s,
          _ => return Err(())
        };
      let arg1 =
        match xs[1] {
          Sexp::Atom(Atom::I(i)) => i,
          _ => return Err(()),
        };
      let arg2 =
        match xs[2] {
          Sexp::Atom(Atom::I(i)) => i,
          _ => return Err(()),
        };
      Ok((command_name, arg1, arg2))
    }
  }
}

#[allow(type_complexity)]
pub fn command_vec(command: &Sexp) -> Result<(&str, Vec<(i64, i64)>), ()> {
  match *command {
    Sexp::Atom(_) => Err(()),
    Sexp::List(ref xs) => {
      if xs.len() != 2 { return Err(()) }
      let command_name =
        match xs[0] {
          Sexp::Atom(Atom::S(ref s)) => &*s,
          _ => return Err(()),
        };
      let vec =
        match xs[1] {
          Sexp::Atom(_) => return Err(()),
          Sexp::List(ref vs) => {
            let mut r = Vec::new();
            for e in vs {
              match *e {
                Sexp::Atom(_) => return Err(()),
                Sexp::List(ref kv) => {
                  if kv.len() != 2 { return Err(()) }
                  let k =
                    match kv[0] {
                      Sexp::Atom(Atom::I(i)) => i,
                      _ => { return Err(()) },
                    };
                  let v =
                    match kv[1] {
                      Sexp::Atom(Atom::I(i)) => i,
                      _ => { return Err(()) },
                    };
                  r.push((k, v));
                }
              }
            }
            r
          }
        };
      Ok((command_name, vec))
    }
  }
}
