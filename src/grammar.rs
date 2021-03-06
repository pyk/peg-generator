// Generated by rust-peg. Do not edit.
use self::RuleResult::{Matched, Failed};
fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
fn char_range_at(s: &str, pos: usize) -> (char, usize) {
    let c = &s[pos..].chars().next().unwrap();
    let next_pos = pos + c.len_utf8();
    (*c, next_pos)
}
#[derive(Clone)]
enum RuleResult<T> {
    Matched(usize, T),
    Failed,
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub expected: ::std::collections::HashSet<&'static str>,
}
pub type ParseResult<T> = Result<T, ParseError>;
impl ::std::fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        try!(write!(
            fmt,
            "error at {}:{}: expected ",
            self.line,
            self.column
        ))
;
        if self.expected.len() == 0 {
            try!(write!(fmt, "EOF"));
        } else if self.expected.len() == 1 {
            try!(write!(
                fmt,
                "`{}`",
                escape_default(self.expected.iter().next().unwrap())
            ));
        } else {
            let mut iter = self.expected.iter();
            try!(write!(
                fmt,
                "one of `{}`",
                escape_default(iter.next().unwrap())
            ));
            for elem in iter {
                try!(write!(fmt, ", `{}`", escape_default(elem)));
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for ParseError {
    fn description(&self) -> &str {
        "parse error"
    }
}
fn slice_eq(input: &str, state: &mut ParseState, pos: usize, m: &'static str) -> RuleResult<()> {
# ! [ inline ] # ! [ allow ( dead_code ) ]    let l = m.len();
    if input.len() >= pos + l && &input.as_bytes()[pos..pos + l] == m.as_bytes() {
        Matched(pos + l, ())
    } else {
        state.mark_failure(pos, m)
    }
}
fn slice_eq_case_insensitive(
    input: &str,
    state: &mut ParseState,
    pos: usize,
    m: &'static str,
) -> RuleResult<()> {
# ! [ inline ] # ! [ allow ( dead_code ) ]    let mut used = 0usize;
    let mut input_iter = input[pos..].chars().flat_map(|x| x.to_uppercase());
    for m_char_upper in m.chars().flat_map(|x| x.to_uppercase()) {
        used += m_char_upper.len_utf8();
        let input_char_result = input_iter.next();
        if input_char_result.is_none() || input_char_result.unwrap() != m_char_upper {
            return state.mark_failure(pos, m);
        }
    }
    Matched(pos + used, ())
}
fn any_char(input: &str, state: &mut ParseState, pos: usize) -> RuleResult<()> {
# ! [ inline ] # ! [ allow ( dead_code ) ]    if input.len() > pos {
        let (_, next) = char_range_at(input, pos);
        Matched(next, ())
    } else {
        state.mark_failure(pos, "<character>")
    }
}
fn pos_to_line(input: &str, pos: usize) -> (usize, usize) {
    let before = &input[..pos];
    let line = before.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1;
    let col = before.chars().rev().take_while(|&c| c != '\n').count() + 1;
    (line, col)
}
impl<'input> ParseState<'input> {
    fn mark_failure(&mut self, pos: usize, expected: &'static str) -> RuleResult<()> {
        if self.suppress_fail == 0 {
            if pos > self.max_err_pos {
                self.max_err_pos = pos;
                self.expected.clear();
            }
            if pos == self.max_err_pos {
                self.expected.insert(expected);
            }
        }
        Failed
    }
}
struct ParseState<'input> {
    max_err_pos: usize,
    suppress_fail: usize,
    expected: ::std::collections::HashSet<&'static str>,
    _phantom: ::std::marker::PhantomData<&'input ()>,
}
impl<'input> ParseState<'input> {
    fn new() -> ParseState<'input> {
        ParseState {
            max_err_pos: 0,
            suppress_fail: 0,
            expected: ::std::collections::HashSet::new(),
            _phantom: ::std::marker::PhantomData,
        }
    }
}

fn __parse_program<'input>(
    __input: &'input str,
    __state: &mut ParseState<'input>,
    __pos: usize,
) -> RuleResult<String> {# ! [ allow ( non_snake_case , unused ) ]    {
        let __seq_res = {
            let str_start = __pos;
            match slice_eq(__input, __state, __pos, "hello world") {
                Matched(__newpos, _) => Matched(__newpos, &__input[str_start..__newpos]),
                Failed => Failed,
            }
        };
        match __seq_res {
            Matched(__pos, n) => {
                Matched(__pos, {
                    n.to_string()
                })
            }
            Failed => Failed,
        }
    }
}

pub fn program<'input>(__input: &'input str) -> ParseResult<String> {# ! [ allow ( non_snake_case , unused ) ]    let mut __state = ParseState::new();
    match __parse_program(__input, &mut __state, 0) {
        Matched(__pos, __value) => {
            if __pos == __input.len() {
                return Ok(__value);
            }
        }
        _ => {}
    }
    let (__line, __col) = pos_to_line(__input, __state.max_err_pos);
    Err(ParseError {
        line: __line,
        column: __col,
        offset: __state.max_err_pos,
        expected: __state.expected,
    })
}
