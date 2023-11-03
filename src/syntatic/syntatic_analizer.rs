use std::collections::HashMap;

use crate::lexical::Lexeme;
use crate::lexical::tokens::Token;

use crate::interpreter::expressions::Expression;
use crate::interpreter::expressions::ExprType;
use crate::interpreter::expressions::BoolOp;
use crate::interpreter::expressions::IntOp;
use crate::interpreter::expressions::Expr;
use crate::interpreter::expressions::BinaryInt;
use crate::interpreter::expressions::SingleBool;

use crate::interpreter::commands::Command;
use crate::interpreter::commands::AssignCmd;
use crate::interpreter::commands::WhileCmd;
use crate::interpreter::commands::IfCmd;
use crate::interpreter::commands::cmd_exec;

struct State {
  pub lexes: Vec::<Lexeme>,
  pub current: usize,
  pub line_number: u16,
}

pub fn start(lexes: Vec::<Lexeme>){
    let mut state = State {lexes,current:0,line_number:1};
    let mut map = HashMap::<String,i32>::new();
  
    let cmds = proc_program(&mut state);
    eat(&mut state,Token::EndOfFile);

    if let Command::Block(b) = cmds {
      cmd_exec(b,&mut map);
    }
}

fn show_error(lex: &Lexeme){
  match lex.ttype {
    Token::EndOfFile => {
      println!("{}:: Fim de arquivo inesperado",lex.line_number);
    },
    _ => print!("{}:: Lexema não experado [{},{}]\n ",lex.line_number,lex.token,Token::to_string(&lex.ttype).expect("Bad token"))
  }
  std::process::exit(1);
}
  
fn eat(state: &mut State,_desired: Token){
  /*println!("Expected: {}, Found: ({},{})",
           Token::to_string(&desired).expect("Bad token"),
           state.lexes[state.current].token,
           Token::to_string(&state.lexes[state.current].ttype).expect("Bad token")); */

  if matches!(&state.lexes[state.current].ttype, desired){
    state.current += 1;
  }
  else {
    show_error(&state.lexes[state.current]);
  }
}

// <program>   ::= program <cmdlist>
fn proc_program(state: &mut State) -> Command {
  eat(state,Token::Program);
  return proc_cmd_list(state);
}
  
  // <cmdlist>   ::= <cmd> { <cmd> }
fn proc_cmd_list(state: &mut State) -> Command {
  let mut cmds = Vec::<Command>::new();
  cmds.push(proc_cmd(state));

  while matches!(state.lexes[state.current].ttype, Token::Var) ||
    matches!(state.lexes[state.current].ttype, Token::Output) ||
    matches!(state.lexes[state.current].ttype, Token::If) ||
    matches!(state.lexes[state.current].ttype, Token::While) {

    cmds.push(proc_cmd(state));
  }
  
  return Command::Block(cmds);
}
  
// <cmd>       ::= (<assign> | <output> | <if> | <while>) ;
fn proc_cmd(state: &mut State) -> Command {
  let mut cmd: Command = Command::Block(Vec::<Command>::new());

  if matches!(state.lexes[state.current].ttype, Token::Var) {
    cmd = proc_assign(state);
  }
  else if matches!(state.lexes[state.current].ttype, Token::Output) {
    cmd = proc_output(state);
  }
  else if matches!(state.lexes[state.current].ttype, Token::If) {
    cmd = proc_if(state);
  }
  else if matches!(state.lexes[state.current].ttype, Token::While) {
    cmd = proc_while(state);
  }
  else {
    show_error(&state.lexes[state.current]);
  }
  eat(state,Token::Semicolon);

  return cmd;
}

// <if>        ::= if <boolexpr> then <cmdlist> [ else <cmdlist> ] done
fn proc_if(state: &mut State) -> Command {
  eat(state,Token::If);
  let condition = proc_bool_expr(state,false);
  
  eat(state,Token::Then);
  let then_cmds = proc_cmd_list(state);
  
  let mut else_cmds = Command::Block(Vec::<Command>::new());

  if matches!(state.lexes[state.current].ttype, Token::Else){
    state.current += 1;
    else_cmds = proc_cmd_list(state);
  }
  
  eat(state,Token::Done);

  if let Command::Block(i) = then_cmds {
    if let Command::Block(e) = else_cmds{
      return Command::If(IfCmd{condition,then_cmds:i,else_cmds:e});
    }
  }

  panic!("Bad blocks");
}
  
// <while>     ::= while <boolexpr> do <cmdlist> done
fn proc_while(state: &mut State) -> Command {
  eat(state,Token::While);
  let cond = proc_bool_expr(state,false);
  
  eat(state,Token::Do);
  let cmds = proc_cmd_list(state);
  
  eat(state,Token::Done);

  if let Command::Block(b) = cmds {
    return Command::While(WhileCmd{cond, cmds: b});
  }

  panic!("Bad blocks");
}

// <assign>    ::= <var> = <intexpr>
fn proc_assign(state: &mut State) -> Command {
  let var = proc_var(state);
  eat(state,Token::Assign);
  
  let expr = proc_int_expr(state);
  
  if let Expression::Ex(ex) = var {
    return Command::Assignment(AssignCmd{expr,var: ex.name});
  }

  panic!("Bad name");
}

// <output>    ::= output <intexpr>
fn proc_output(state: &mut State) -> Command {
  eat(state,Token::Output);
  let expr = proc_int_expr(state);

  return Command::Output(expr);
}

// <var>       ::= id
fn proc_var(state: &mut State) -> Expression {
  
  let ex = Expression::Ex(Expr{expr: ExprType::Var,name: state.lexes[state.current].token.clone(), value: 0});

  eat(state,Token::Var);

  return ex;
}

// <const>     ::= number
fn proc_const(state: &mut State) -> Expression {
  let tmp = state.lexes[state.current].token.clone();

  eat(state,Token::Number);

  let value = tmp.parse::<i32>().expect("Bad number");

  return Expression::Ex(Expr {expr: ExprType::ConstInt, value, name: String::new()});
}
  
// <boolexpr>  ::= false | true |
//                 not <boolexpr> |
//                 <intterm> (== | != | < | > | <= | >=) <intterm>
fn proc_bool_expr(state: &mut State, not: bool) -> Expression {
  if matches!(&state.lexes[state.current].ttype, Token::False) {
    state.current += 1;
    let mut value = 0;
    if not {value = !value}
    
    return Expression::Ex(Expr {expr: ExprType::ConstBool,value, name: String::new()});
    
  }
  else if matches!(&state.lexes[state.current].ttype, Token::True) {
    state.current += 1;
    let mut  value = 1;
    if not {value = !value}
    
    return Expression::Ex(Expr {expr: ExprType::ConstBool,value, name: String::new()});
    
  }
  else if matches!(&state.lexes[state.current].ttype, Token::Not) {
    state.current += 1;
    return proc_bool_expr(state,true);
  }
  else {
    let mut bool_op: BoolOp = BoolOp::Equal;
    let left = proc_int_term(state);

    match state.lexes[state.current].ttype {
      Token::Equal => {
        bool_op = BoolOp::Equal;
      },
      Token::NotEqual => {
        bool_op = BoolOp::NotEqual;
      },
      Token::Lower => {
        bool_op = BoolOp::Lower;
      },
      Token::Greater => {
        bool_op = BoolOp::Greater;
      },
      Token::GreaterEqual => {
        bool_op = BoolOp::GreaterEqual;
      },
      Token::LowerEqual => {
        bool_op = BoolOp::LowerEqual;
      },
      _ => show_error(&state.lexes[state.current])
    }
    state.current += 1;
    let right = proc_int_term(state);

    if let Expression::Ex(l) = left {
      if let Expression::Ex(r) = right{
        return Expression::Sb(SingleBool{left:l,right:r,op: bool_op});
      }
    }

    panic!("Bad bool expressions");
  }
}
  
// <intexpr>   ::= [ + | - ] <intterm> [ (+ | - | * | / | % | ^) <intterm> ]
fn proc_int_expr(state: &mut State) -> Expression {
  let mut negative = false;

  if matches!(&state.lexes[state.current].ttype, Token::Add){
    state.current += 1;
  }
  else if matches!(&state.lexes[state.current].ttype, Token::Sub) {
    state.current += 1;
    negative = true;
  }

  let mut term = proc_int_term(state);

  if negative {
    if let Expression::Ex(ref mut e) = term {
      e.expr = ExprType::NegInt;
    }
    else {
      panic!("Can't invert int term (Bad term)");
    }
  }

  if  matches!(&state.lexes[state.current].ttype, Token::Add) ||
    matches!(&state.lexes[state.current].ttype, Token::Sub) ||
    matches!(&state.lexes[state.current].ttype, Token::Mul) ||
    matches!(&state.lexes[state.current].ttype, Token::Div) ||
    matches!(&state.lexes[state.current].ttype, Token::Mod) ||
    matches!(&state.lexes[state.current].ttype, Token::Pow) 
  {

    let mut op: IntOp = IntOp::Add;

    match state.lexes[state.current].ttype {
      Token::Add => {
        op = IntOp::Add
      },
      Token::Sub => {
        op = IntOp::Sub
      },
      Token::Mul => {
        op = IntOp::Mul
      },
      Token::Div => {
        op = IntOp::Div
      },
      Token::Mod => {
        op = IntOp::Mod
      },
      Token::Pow => {
        op = IntOp::Pow
      }
      _ => show_error(&state.lexes[state.current])
    }

    state.current += 1;

    let right = proc_int_term(state);

    if let Expression::Ex(ref l) = term {
      if let Expression::Ex(ref r) = right{
        return Expression::Bi( BinaryInt{left:l.clone(),right:r.clone(),op} );
      }
    }
  }
  return term;
}
  
// <intterm>   ::= <var> | <const> | read
fn proc_int_term(state: &mut State) -> Expression{
  if matches!(state.lexes[state.current].ttype, Token::Var){
    return proc_var(state);
  }
  else if matches!(state.lexes[state.current].ttype, Token::Number){
    return proc_const(state);
  }
  else {
    eat(state,Token::Read);
    
    return Expression::Ex(Expr {expr: ExprType::ReadInt,value:0, name: String::new()});
  }
}
