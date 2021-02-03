use qk_term::pid::Pid;
use qk_term::term::Term;

#[derive(Debug)]
pub struct SendMessageEvent {
  from: Pid,
  to: Pid,
  value: Term,
}
