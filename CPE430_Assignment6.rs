struct numC {
  num : i32
}

struct ifC<T> {
  cond: T,
  first: T,
  second: T,
}

struct idC {
  symbol: String,
}

struct appC<T> {
  func: T,
  args: Vec<T>,
}

struct binOp<T> {
  operator: String,
  first: T,
  second: T,
}

struct boolC {
  b: bool,
}

struct lamC<T> {
  args: Vec<String>,
  body: T,
}

struct with<T> {
  vars: Vec<Vec<String>>,
  body: T
}

fn main () {
  return
}
