

pub enum ExprC
{
  NumC(i32),
  BoolC(bool),
  IdC(String),

  IfC{Box<ExprC>, Box<ExprC>, Box<ExprC>},
  /*AppC(Box<ExprC>, Vec<Box<ExprC>>),
  BinOpC(String, Box<ExprC>, Box<ExprC>),
  LamC(Vec<String>, Box<ExprC>),
  WithC(Vec<Vec<String>>, Box<ExprC>),*/
  Missing,
}

pub enum Value
{
  NumV(i32),
  BoolV(bool),
  StringV(String),
  Missing,
}



fn interp(exp: &ExprC) -> Value
{
  match *exp
  {
    ExprC::NumC(n) => return Value::NumV(n),
    ExprC::BoolC(b) => return Value::BoolV(b),
    ExprC::IdC(s) => return Value::StringV(s),
    ExprC::Missing => return Value::Missing,
                      
  }
}


fn main() {
	println!("running");
  let test = ExprC::BoolC(true);

  let temp = interp(&test);

  match temp
  {
    Value::NumV(n) => println!("Number is {}", n),
    Value::BoolV(b) => println!("Boolean is {}", b),
    Value::StringV(s) => println!("String is {}", s),
    Value::Missing => println!("error"),
  }
  
}
