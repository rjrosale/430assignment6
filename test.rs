pub enum ExprC
{
  NumC(i32),
  BoolC(bool),
  IdC(String),

  IfC(Box<ExprC>, Box<ExprC>, Box<ExprC>),
  AppC(Box<ExprC>, Vec<Box<ExprC>>),
  BinOpC(String, Box<ExprC>, Box<ExprC>),
  LamC(Vec<String>, Box<ExprC>),
  WithC(Vec<Vec<String>>, Box<ExprC>),
  Missing,
}

pub enum Value
{
  NumV(i32),
  BoolV(bool),
  StringV(String),
  Missing,
}


fn interp(exp: ExprC) -> Value
{
  match exp
  {
    ExprC::NumC(n) => return Value::NumV(n),
    ExprC::BoolC(b) => return Value::BoolV(b),
    ExprC::IdC(s) => return Value::StringV(s),
    
    ExprC::IfC(i, t, e) => return ifHelper(interp(*i), interp(*t), interp(*e)),
    ExprC::AppC(body, args) => return Value::Missing,
    ExprC::BinOpC(op, l, r) => return Value::Missing,
    ExprC::LamC(arg, body) => return Value::Missing,
	ExprC::WithC(v, a) => return Value::Missing,
	ExprC::Missing => return Value::Missing,
  }
}

fn ifHelper(i: Value, t: Value, e: Value) -> Value
{
  match i
  {
      Value::BoolV(b) => 
      {
        if b
        {
          return t;
        }
        else
        {
          return e;
        }
      },
      Value::NumV(n) => return Value::Missing,
      Value::StringV(s)=> return Value::Missing,
      Value::Missing => return Value::Missing,  
  }
}

fn main() {
    println!("running");
  let test = ExprC::IfC(Box::new(ExprC::BoolC(true)), Box::new(ExprC::NumC(1)), Box::new(ExprC::NumC(2)));

  let temp = interp(test);

  match temp
  {
    Value::NumV(n) => println!("Number is {}", n),
    Value::BoolV(b) => println!("Boolean is {}", b),
    Value::StringV(s) => println!("String is {}", s),
    Value::Missing => println!("error"),
  }
  
}