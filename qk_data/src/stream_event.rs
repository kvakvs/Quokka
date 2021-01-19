use qk_term::atom::Atom;
use qk_term::pid::Pid;
use qk_term::term::Term;

use crate::execute_function_trace_event::ExecuteFunctionTraceEvent;
use crate::send_message_event::SendMessageEvent;

type UtcDatetime = chrono::DateTime<chrono::Utc>;

pub trait TStreamEvent {
    fn start_time(&self) -> Option<UtcDatetime>;
    fn end_time(&self) -> Option<UtcDatetime>;
    fn duration(&self) -> std::time::Duration;
}

/// Full Mod:Fun/Arity representation, to store in a lookup table. Use MFArityIndex instead.
pub struct MFArity {
    module: Atom,
    fun: Atom,
    arity: u16,
}

pub struct MFArityIndex(usize);

pub enum StreamEvent {
    ExecuteFunctionEvent(ExecuteFunctionTraceEvent),
    SendMessageEvent(SendMessageEvent),
}
