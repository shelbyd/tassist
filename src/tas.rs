use failure::Error;
use nom::{self, types::CompleteStr};

#[derive(Debug, PartialEq)]
pub struct Tas {
    start_state_file: String,
    expressions: Vec<Expression>,
}

impl Tas {
    pub fn as_lua(&self) -> String {
        format!(
            include_str!("tas_template.tpl.lua"),
            initial_save_state_file=self.start_state_file,
            gameplay=self.gameplay_lua())
    }

    pub fn parse(contents: &str) -> Result<Tas, Error> {
        tas(CompleteStr(contents))
            .map(|(_remaining, tas)| tas)
            .map_err(|e| format_err!("{}", e))
    }

    fn gameplay_lua(&self) -> String {
        self.expressions
            .iter()
            .map(Expression::gameplay_lua)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, PartialEq)]
enum Expression {
    Wait(u64),
    Input(Vec<String>),
    Comment(String),
}
use self::Expression::*;

impl Expression {
    fn gameplay_lua(&self) -> String {
        match *self {
            Wait(frames) => {
                format!("advance_frames({})", frames)
            }
            Input(ref buttons) => {
                let button_text = buttons
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("press({{{}}})", button_text)
            }
            Comment(ref text) => {
                format!("--{}", text)
            }
        }
    }
}

named!(tas<CompleteStr, Tas>, do_parse!(
    start_state: start_state >>
    expressions: expressions >>
    (Tas {
        start_state_file: start_state.to_string(),
        expressions,
    })
));

named!(start_state<CompleteStr, CompleteStr>, ws!(preceded!(tag!("StartState"), string)));

named!(string<CompleteStr, CompleteStr>, delimited!(quote, inner_string, quote));

named!(quote<CompleteStr, CompleteStr>, tag!("\""));
named!(inner_string<CompleteStr, CompleteStr>, escaped!(none_of!("\\\""), '\\', one_of!("\"\\")));

named!(expressions<CompleteStr, Vec<Expression>>, ws!(many0!(expr)));

named!(expr<CompleteStr, Expression>, alt!(
    wait |
    input |
    comment
));

named!(wait<CompleteStr, Expression>, map!(
    ws!(preceded!(
        tag!("Wait"), integer)),
        (Expression::Wait)
));

named!(
    integer<CompleteStr, u64>,
    map_res!(
      nom::digit,
      |s: CompleteStr| s.parse::<u64>()
    )
);

named!(input<CompleteStr, Expression>,
    map!(
        ws!(preceded!(tag!("Input"), buttons)),
        (Expression::Input)
    )
);

named!(buttons<CompleteStr, Vec<String>>,
    map!(
        separated_nonempty_list!(tag!(","), bare_word),
        |buttons| buttons.into_iter().map(|s| s.to_string()).collect()
    )
);

named!(bare_word<CompleteStr, CompleteStr>,
    call!(nom::alphanumeric)
);

named!(comment<CompleteStr, Expression>,
    map!(
        preceded!(tag!("//"),
            alt!(
                take_until_and_consume!("\n") |
                recognize!(
                    tuple!(
                        many0!(nom::anychar),
                        eof!()
                    )
                )
            )
        ),
        |text| Expression::Comment(text.to_string())
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_parsing() {
        assert_eq!(inner_string(CompleteStr("abc")), Ok((CompleteStr(""), CompleteStr("abc"))));
        assert_eq!(string(CompleteStr("\"abc\"")), Ok((CompleteStr(""), CompleteStr("abc"))));
    }

    #[test]
    fn start_state_parsing() {
        assert_eq!(start_state(CompleteStr("StartState \"foo\"")),
                   Ok((CompleteStr(""), CompleteStr("foo"))));
        assert_eq!(start_state(CompleteStr("StartState \"foo\"\n\nStuff")),
                   Ok((CompleteStr("Stuff"), CompleteStr("foo"))));
    }

    #[test]
    fn wait_parsing() {
        assert_eq!(expr(CompleteStr("Wait 100")), Ok((CompleteStr(""), Expression::Wait(100))));
    }

    #[test]
    fn input_parsing() {
        assert_eq!(expr(CompleteStr("Input Foo")),
                   Ok((CompleteStr(""), Expression::Input(vec!["Foo".to_string()]))));
        assert_eq!(expr(CompleteStr("Input Foo\n")),
                   Ok((CompleteStr(""), Expression::Input(vec!["Foo".to_string()]))));
    }

    #[test]
    fn expressions_parsing() {
        assert_eq!(expressions(CompleteStr("\n\nWait 100")),
                   Ok((CompleteStr(""), vec![Expression::Wait(100)])));
    }

    #[test]
    fn comment_parsing() {
        assert_eq!(comment(CompleteStr("// Wait 100\n")),
                   Ok((CompleteStr(""), Expression::Comment(" Wait 100".to_string()))));
        assert_eq!(comment(CompleteStr("// Wait 100")),
                   Ok((CompleteStr(""), Expression::Comment(" Wait 100".to_string()))));
    }

    #[test]
    fn just_start_state() {
        let parsed = tas(CompleteStr("StartState \"test.State\"")).unwrap();
        assert_eq!(parsed, (CompleteStr(""), Tas {
            start_state_file: "test.State".to_string(),
            expressions: vec![],
        }));
    }

    #[test]
    fn few_expressions() {
        let parsed = tas(
            CompleteStr("StartState \"test.State\"\n\nWait 100\nInput Start\n// Comment")
        ).unwrap();
        assert_eq!(parsed, (CompleteStr(""), Tas {
            start_state_file: "test.State".to_string(),
            expressions: vec![
                Expression::Wait(100),
                Expression::Input(vec!["Start".to_string()]),
                Expression::Comment(" Comment".to_string()),
            ],
        }));
    }

    #[test]
    fn expression_lua() {
        assert_eq!(Expression::Input(vec!["Start".to_string(), "A".to_string()]).gameplay_lua(),
                   "press({\"Start\", \"A\"})".to_string());
    }
}