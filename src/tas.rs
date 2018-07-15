use failure::Error;

#[derive(Debug, PartialEq, Default)]
pub struct Tas {
    start_state_file: String,
}

impl Tas {
    pub fn as_lua(&self) -> String {
        format!(
            include_str!("tas_template.tpl.lua"),
            initial_save_state_file=self.start_state_file)
    }

    pub fn parse(contents: &str) -> Result<Tas, Error> {
        tas(contents)
            .map(|(_remaining, tas)| tas)
            .map_err(|e| format_err!("{}", e))
    }
}

named!(tas<&str, Tas>, do_parse!(
    start_state: start_state >>
    (Tas {
        start_state_file: start_state.to_string(),
    })
));

named!(start_state<&str, &str>, preceded!(ws!(tag!("StartState")), string));

named!(string<&str, &str>, delimited!(quote, inner_string, quote));

named!(quote<&str, &str>, tag!("\""));
named!(inner_string<&str, &str>, escaped!(none_of!("\\\""), '\\', one_of!("\"\\")));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_parsing() {
        assert_eq!(inner_string("abc"), Ok(("", "abc")));
        assert_eq!(string("\"abc\""), Ok(("", "abc")));
    }

    #[test]
    fn start_state_parsing() {
        assert_eq!(start_state("StartState \"foo\""), Ok(("", "foo")));
    }

    #[test]
    fn just_start_state() {
        let parsed = Tas::parse(r#"StartState "test.State""#).unwrap();
        assert_eq!(parsed, Tas {
            start_state_file: "test.State".to_string(),
        });
    }
}