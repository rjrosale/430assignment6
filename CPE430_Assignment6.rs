pub enum ExprC
{
  NumC(i32),
  BoolC(bool),
  IdC(String),

  IfC(Box<ExprC>, Box<ExprC>, Box<ExprC>),
  BinOpC(String, Box<ExprC>, Box<ExprC>), 
  /*AppC(Box<ExprC>, Vec<Box<ExprC>>),
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

fn binOpHelper(op: String, first: ExprC, second: ExprC) -> Value
{
  
  match first
  {
      ExprC::NumC(n) => {
          match second 
          {
              ExprC::NumC(n2) => Value::NumV(n + n2),
              _ => return Value::Missing,
          }}
      _ => return Value::Missing
      
  }
}

fn interp(exp: ExprC) -> Value
{
  match exp
  {
    ExprC::NumC(n) => return Value::NumV(n),
    ExprC::BoolC(b) => return Value::BoolV(b),
    ExprC::IdC(s) => return Value::StringV(s),
    
    ExprC::IfC(i, t, e) => return ifHelper(interp(*i), interp(*t), interp(*e)),
     
    ExprC::BinOpC(op, first, second) => return binOpHelper(op, *first, *second),
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

fn testHelper(test: ExprC)
{
  let temp = interp(test);
  match temp
  {
    Value::NumV(n) => println!("Number is {}", n),
    Value::BoolV(b) => println!("Boolean is {}", b),
    Value::StringV(s) => println!("String is {}", s),
    Value::Missing => println!("error"),
  }
}

fn main() {
    println!("running");
    
  //tests!!!!!
  let test = ExprC::BoolC(true);
  testHelper(test);
  
  let test2 = ExprC::IfC(Box::new(ExprC::BoolC(true)), Box::new(ExprC::NumC(1)), Box::new(ExprC::NumC(2)));
  testHelper(test2);
  
  let str2 = "+".to_string();
  let test3 = ExprC::BinOpC(str2, Box::new(ExprC::NumC(1)), Box::new(ExprC::NumC(3)));
  testHelper(test3);
}
