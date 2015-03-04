pub enum ExprC
{
  NumC(i32),
  BoolC(bool),
  IdC(String),

  IfC(Box<ExprC>, Box<ExprC>, Box<ExprC>),
  BinOpC(String, Box<ExprC>, Box<ExprC>), 
  Missing,
}

pub enum Value
{
  NumV(i32),
  BoolV(bool),
  StringV(String),
  Missing,
}

fn equals(v1: Value, v2: Value) -> bool
{
  match v1
  {
    Value::NumV(val) =>
      {
        match v2
        {
          Value::NumV(val2) => return (val == val2),
          _ => return false,
        }
      },

      Value::BoolV(val) =>
      {
        match v2
        {
          Value::BoolV(val2) => return (val == val2),
          _ => return false,
        }
      },

      Value::StringV(val) =>
      {
        match v2
        {
          Value::StringV(val2) => return (val == val2),
          _ => return false,
        }
      },

      Value::Missing => return false,
  }
}

fn binOpHelper(op: String, first: ExprC, second: ExprC) -> Value
{
  
  match first
  {
      ExprC::NumC(n) => {
          match second 
          {
              ExprC::NumC(n2) => 
              if op == "+" { Value::NumV(n + n2) }
              else if op == "-" { Value::NumV(n - n2) }
              else if op == "/" { Value::NumV(n / n2) }
              else if op == "*" { Value::NumV(n * n2) }
              else { return Value::Missing },
              _ => return Value::Missing,
          }}
      ExprC::BoolC(b) => {
          match second
          {
            ExprC::BoolC(b2) =>
              if op == "eq?" { Value::BoolV(b == b2) }
              else if op == "<=" { Value::BoolV(b <= b2) }
              else { return Value::Missing },
              _ => return Value::Missing,
          }},
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

fn testHelper(test: ExprC) -> Value
{
  return interp(test);
  /*match temp
  {
    Value::NumV(n) => println!("Number is {}", n),
    Value::BoolV(b) => println!("Boolean is {}", b),
    Value::StringV(s) => println!("String is {}", s),
    Value::Missing => println!("error"),
  }*/
}

fn main() {
    println!("running");
    
  //tests!!!!!
  let test = ExprC::BoolC(true);
  assert!(equals(testHelper(test), Value::BoolV(true)));

  let mtest2 = ExprC::BoolC(false);
  assert!(equals(testHelper(mtest2), Value::BoolV(false)));

  let mtest = ExprC::NumC(1);
  assert!(equals(testHelper(mtest), Value::NumV(1)));
  
  let test2 = ExprC::IfC(Box::new(ExprC::BoolC(true)), Box::new(ExprC::NumC(1)), Box::new(ExprC::NumC(2)));
  assert!(equals(testHelper(test2), Value::NumV(1)));
  
  let str2 = "+".to_string();
  let test3 = ExprC::BinOpC(str2, Box::new(ExprC::NumC(1)), Box::new(ExprC::NumC(3)));
  assert!(equals(testHelper(test3), Value::NumV(4)));

  let rstr3 = "eq?".to_string();
  let rtest4 = ExprC::BinOpC(rstr3, Box::new(ExprC::BoolC(true)), Box::new(ExprC::BoolC(false)));
  assert!(equals(testHelper(rtest4), Value::BoolV(false)));

  
  println!("All tests passed.");
}
