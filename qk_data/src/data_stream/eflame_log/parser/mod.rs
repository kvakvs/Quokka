pub mod atom;

use qk_term::pid::Pid;
use crate::data_stream::eflame_log::defs::{ExecutionTime, EflameLogLine, EflameValue};
use qk_term::mfarity::MFArity;

fn parse_u64(i: &str) -> nom::IResult<&str, u64> {
  nom::combinator::map_res(
    nom::multi::many1(nom::character::complete::one_of("0123456789")),
    |result_v: Vec<char>| {
      let result_str: String = result_v.into_iter().collect();
      u64::from_str_radix(&result_str, 10)
    },
  )(i)
}

fn parse_u16(i: &str) -> nom::IResult<&str, u16> {
  nom::combinator::map_res(
    nom::multi::many1(nom::character::complete::one_of("0123456789")),
    |result_v: Vec<char>| {
      let result_str: String = result_v.into_iter().collect();
      u16::from_str_radix(&result_str, 10)
    },
  )(i)
}

fn parse_tail_without_sleep(i: &str) -> nom::IResult<&str, EflameValue> {
  nom::sequence::preceded(
    nom::character::complete::space1,
    nom::combinator::map_res(
      nom::multi::many1(nom::character::complete::one_of("0123456789")),
      |result_v: Vec<char>| -> Result<EflameValue, nom::error::Error<&str>> {
        let result_str: String = result_v.into_iter().collect();
        let n = u64::from_str_radix(&result_str, 10).unwrap();
        let et = ExecutionTime { is_sleep: false, time: n };
        Ok(EflameValue::ExecutionTime(et))
      },
    ),
  )(i)
}

fn parse_tail_with_sleep(i: &str) -> nom::IResult<&str, EflameValue> {
  match nom::sequence::tuple((
    nom::bytes::complete::tag(";SLEEP"),
    nom::character::complete::space1,
    parse_u64,
  ))(i) {
    Ok((remaining, (_sleep, _, time))) => {
      let et = ExecutionTime { is_sleep: true, time };
      Ok((remaining, EflameValue::ExecutionTime(et)))
    }
    Err(e) =>
      Err(e),
  }
}

fn parse_tail(i: &str) -> nom::IResult<&str, EflameValue> {
  nom::branch::alt((parse_tail_with_sleep, parse_tail_without_sleep))(i)
}

fn parse_numeric_pid(i: &str) -> nom::IResult<&str, EflameValue> {
  match nom::multi::separated_list1(
    nom::bytes::complete::tag("."),
    parse_u16,
  )(i) {
    Ok((remaining, pid_component)) => {
      let p = Pid(pid_component[0], pid_component[1], pid_component[2]);
      Ok((remaining, EflameValue::Pid(p)))
    }
    Err(e) =>
      Err(e),
  }
}

fn parse_mfarity(i: &str) -> nom::IResult<&str, EflameValue> {
  match nom::sequence::tuple(
    (atom::parse_atom,
     nom::bytes::complete::tag(":"),
     atom::parse_atom,
     nom::bytes::complete::tag("/"),
     parse_u64,
    ))(i) {
    Ok((remaining, (m, _, f, _, arity))) => {
      let mfarity = MFArity::new_a(m, f, arity as u16);
      let mfav = EflameValue::MFArity(mfarity);
      Ok((remaining, mfav))
    }
    Err(e) =>
      Err(e),
  }
}

/// Parse '(' followed by pid1.pid2.pid3 components, terminated by ')'
fn parse_parenthesized_pid(i: &str) -> nom::IResult<&str, EflameValue> {
  nom::sequence::delimited(
    nom::bytes::complete::tag("("),
    parse_numeric_pid,
    nom::bytes::complete::tag(")"),
  )(i)
}

/// Try consume (pid_in_parentheses) ";" Vec<MFarity>_sep_by(;) ";" tail_piece
pub(crate) fn parse_eflame_log_line(i: &str) -> nom::IResult<&str, EflameLogLine> {
  match nom::sequence::tuple(
    (parse_parenthesized_pid,
     nom::bytes::complete::tag(";"),
     nom::multi::separated_list1(
       nom::bytes::complete::tag(";"),
       parse_mfarity),
     parse_tail,
    ))(i) {
    Ok((remaining,
         (efv_pid, _sep1, efv_stack, efv_tail))) =>
      {
        // Extract pieces from a tuple of EflameValues and construct an EflameLogLine
        let pid = efv_pid.get_pid();
        let stack: Vec<MFArity> = efv_stack
            .into_iter()
            .map(|efv_mfa| -> MFArity { efv_mfa.get_mfarity() })
            .collect();
        let tail = efv_tail.get_execution_time();

        Ok((remaining, EflameLogLine { pid, stack, tail }))
      }
    Err(e) =>
      Err(e),
  }
}

pub fn parse_test() {
  let atom_input = "'++'";
  println!("parse_quoted_atom {:?}", atom::parse_quoted_atom(atom_input));
  println!("atom {:?}", atom::parse_atom(atom_input));

  // let ppid = "(0.310.0)";
  // println!("ppid {:?}", parse_parenthesized_pid(ppid));
  //
  // let data = "(0.310.0);proc_lib:init_p_do_apply/3;ssh_acceptor:acceptor_loop/6;\
  //   inet_tcp:accept/2;prim_inet:accept0/3;prim_inet:async_accept/2;\
  //   prim_inet:ctl_cmd/3;erts_internal:port_control/3;prim_inet:accept0/3;\
  //   gen:do_call/4;gen:do_call/4;SLEEP 0";
  // println!("{:?}", parse_eflame_log_line(data));
}