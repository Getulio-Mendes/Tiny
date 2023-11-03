use std::collections::HashMap;
use crate::interpreter::expressions::Expression;
use crate::interpreter::memory;

#[derive (Clone)]
pub enum Command {
  Block(Vec::<Command>),
  Assignment(AssignCmd),
  If(IfCmd),
  While(WhileCmd),
  Output(Expression)
}

impl std::fmt::Debug for Command {
  fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
    match self {
      Command::Block(_b) => write!(f, "Block"),
      Command::Assignment(_a) => write!(f,"Assignment"),
      Command::If(_i) => write!(f,"If"),
      Command::While(_w) => write!(f,"While"),
      Command::Output(_o) => write!(f,"Output")
    }
  }
}

#[derive (Clone)]
pub struct AssignCmd {
  pub expr: Expression,
  pub var: String
}

#[derive (Clone)]
pub struct IfCmd {
  pub condition: Expression,
  pub then_cmds: Vec::<Command>,
  pub else_cmds: Vec::<Command> 
}

#[derive (Clone)]
pub struct WhileCmd {
  pub cond: Expression,
  pub cmds: Vec::<Command>,
}

pub fn cmd_exec(list: Vec::<Command>, map: &mut HashMap::<String,i32>){
  //println!("{:?}",list);
  
  for cmd in list {
    
    match cmd{
      Command::Assignment(asg) => {
        let result = asg.expr.exec(map);
        memory::write(map,&asg.var,result);
      },
      Command::Block(b) => {
        cmd_exec(b,map);
      },
      Command::If(i) => {
        if i.condition.exec(map) != 0 {
          cmd_exec(i.then_cmds,map);
        }
        else {
          cmd_exec(i.else_cmds,map);
        }
      },
      Command::Output(o) => {
        println!("{}",o.exec(map));
      },
      Command::While(w) => {
        
        while w.cond.exec(map) != 0 {
      cmd_exec(w.cmds.clone(),map);
        }
      }
    }
  }
}