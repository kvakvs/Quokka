use qk_term::pid::Pid;
use qk_term::mfarity::MFArity;
use std::fmt::Debug;
use std::fmt;

#[derive(PartialEq, Clone)]
pub struct ExecutionTime {
  pub is_sleep: bool,
  pub time: u64,
}

impl Debug for ExecutionTime {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}({}.{:03}ms)",
           if self.is_sleep { "Sleep" } else { "Run" },
           self.time / 1000,
           self.time % 1000)
  }
}

#[derive(Debug, PartialEq)]
pub struct EflameLogLine {
  pub pid: Pid,
  pub stack: Vec<MFArity>,
  pub tail: ExecutionTime,
}


/// Eflame log line looks like
/// (0.310.0);proc_lib:init_p_do_apply/3;ssh_acceptor:acceptor_loop/6;...;gen:do_call/4;gen:do_call/4;SLEEP 0
/// Where mfarities are the call stack, SLEEP is if there was sleeping time included, and the
/// number is microseconds spent in that call stack location.
#[derive(Debug, PartialEq, Clone)]
pub enum EflameValue {
  Pid(Pid),
  MFArity(MFArity),
  ExecutionTime(ExecutionTime),
}

impl EflameValue {
  pub fn get_pid(&self) -> Pid {
    if let EflameValue::Pid(p) = self { p.clone() } else { panic!("Pid value expected") }
  }

  pub fn get_mfarity(&self) -> MFArity {
    if let EflameValue::MFArity(mfa) = self { mfa.clone() } else { panic!("MFArity value expected") }
  }

  pub fn get_execution_time(&self) -> ExecutionTime {
    if let EflameValue::ExecutionTime(e) = self { e.clone() } else { panic!("ExecutionTime value expected") }
  }
}