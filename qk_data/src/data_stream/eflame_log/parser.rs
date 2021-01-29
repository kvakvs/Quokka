use qk_term::pid::Pid;
use nom::IResult;
use crate::mfarity::{MFArity};
use nom::combinator::map_res;
use crate::data_stream::eflame_log::defs::{ExecutionTime, EflameLogLine};

/// Eflame log line looks like
/// (0.310.0);proc_lib:init_p_do_apply/3;ssh_acceptor:acceptor_loop/6;...;gen:do_call/4;gen:do_call/4;SLEEP 0
/// Where mfarities are the call stack, SLEEP is if there was sleeping time included, and the
/// number is microseconds spent in that call stack location.
#[derive(Debug, PartialEq)]
pub enum EflameValue {
  Pid(Pid),
  MFArity(MFArity),
  ExecutionTime(ExecutionTime),
}

fn parse_u64(i: &str) -> nom::IResult<&str, u64> {
  nom::combinator::map_res(
    nom::multi::many1(nom::character::complete::one_of("0123456789")),
    |result_v: Vec<char>| {
      let result_str: String = result_v.into_iter().collect();
      u64::from_str_radix(&result_str, 10)
    },
  )(i)
}

fn parse_tail_without_sleep(i: &str) -> nom::IResult<&str, EflameValue> {
  nom::combinator::map_res(
    nom::multi::many1(nom::character::complete::one_of("0123456789")),
    |result_v: Vec<char>| -> Result<EflameValue, nom::error::Error<&str>> {
      let result_str: String = result_v.into_iter().collect();
      let n = u64::from_str_radix(&result_str, 10).unwrap();
      let et = ExecutionTime { is_sleep: false, time: n };
      Ok(EflameValue::ExecutionTime(et))
    },
  )(i)
}

fn parse_tail_with_sleep(i: &str) -> nom::IResult<&str, EflameValue> {
  match nom::combinator::all_consuming(nom::sequence::tuple((
    nom::bytes::complete::tag("SLEEP"),
    nom::character::complete::space1,
    parse_u64,
  )))(i) {
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
  match nom::combinator::all_consuming(nom::sequence::tuple((
    parse_u64,
    nom::character::complete::char('.'),
    parse_u64,
    nom::character::complete::char('.'),
    parse_u64,
  )))(i) {
    Ok((remaining, (p1, _, p2, _, p3))) => {
      let p = Pid(p1 as u16, p2 as u16, p3 as u16);
      Ok((remaining, EflameValue::Pid(p)))
    }
    Err(e) =>
      Err(e),
  }
}

// TODO: starts not with a capital letter (may be not necessary for Flame logs)
fn is_atom_character(c: char) -> bool {
  c.is_alphanumeric() || c == '_'
}

fn parse_atom_body(i: &str) -> nom::IResult<&str, &str> {
  nom::bytes::complete::take_while1(is_atom_character)(i)
}

fn parse_atom(i: &str) -> nom::IResult<&str, &str> {
  nom::branch::alt(
    (parse_atom_body,
     nom::sequence::delimited(
       nom::character::complete::char('{'),
       parse_atom_body, // TODO: in quotes, escaped characters are allowed
       nom::character::complete::char('}'))
    ))(i)
}

fn parse_mfarity(i: &str) -> nom::IResult<&str, EflameValue> {
  match nom::combinator::all_consuming(nom::sequence::tuple(
    (parse_atom,
     nom::character::complete::char(':'),
     parse_atom,
     nom::character::complete::char('/'),
     parse_u64,
    )))(i) {
    Ok((remaining, (m, _, f, _, arity))) => {
      let mfarity = MFArityValue::new(m, f, arity as u16);
      Ok((remaining, EflameValue::MFArity(mfarity)))
    }
    Err(e) =>
      Err(e),
  }
}

fn parse_parenthesized_pid(i: &str) -> nom::IResult<&str, EflameValue> {
  // Parse '(' followed by pid.pid.pid triple, terminated by ')'
  nom::sequence::preceded(
    nom::character::complete::char('('),
    nom::sequence::terminated(
      parse_numeric_pid,
      nom::character::complete::char(')'),
    ))(i)
}

fn root(i: &str) -> nom::IResult<&str, EflameLogLine> {
  match nom::combinator::all_consuming(nom::sequence::tuple(
    (parse_parenthesized_pid,
     nom::multi::separated_list1(
       nom::character::complete::char(';'),
       parse_mfarity),
     parse_tail,
    )))(i) {
    Ok((remaining, (efvPid, efvStack, efvTail))) => {
      let EflameValue::Pid(pid) = efvPid;
      let stack = efvStack
          .into_iter()
          .map(|efv_mfa| -> MFArityValue {
            let EflameValue::MFArity(mfa) = efv_mfa;
            mfa
          })
          .collect();
      let EflameValue::ExecutionTime(tail) = efvTail;
      let log_line = EflameLogLine { pid, stack, tail };
      Ok((remaining, log_line))
    }
    Err(e) =>
      Err(e),
  }
}

pub fn parse() {
  let data = "(0.310.0);proc_lib:init_p_do_apply/3;ssh_acceptor:acceptor_loop/6;\
    inet_tcp:accept/2;prim_inet:accept0/3;prim_inet:async_accept/2;\
    prim_inet:ctl_cmd/3;erts_internal:port_control/3;prim_inet:accept0/3;\
    gen:do_call/4;gen:do_call/4;SLEEP 0";
  println!("{:?}", root(data));

  // println!(
  //   "parsing a valid line:\n{:#?}\n",
  //   root::<(&str, ErrorKind)>(data)
  // );
}