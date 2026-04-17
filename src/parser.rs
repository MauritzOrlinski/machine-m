use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, take_till, take_while, take_while1},
    character::complete::char,
    combinator::{all_consuming, map, map_res, opt, value},
    number::complete::recognize_float,
    sequence::{delimited, preceded},
};

use crate::ast::Ops;

pub fn parse_program(input: &str) -> Result<Vec<Ops>, nom::Err<nom::error::Error<&str>>> {
    input
        .lines()
        .map(|line| parse_line(line).map(|(_, op)| op))
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, Ops> {
    all_consuming(delimited(
        hspace0,
        alt((
            map(instruction_with_comment, |op| op),
            value(Ops::Nop, opt(comment)),
        )),
        hspace0,
    ))
    .parse(input)
}

fn instruction_with_comment(input: &str) -> IResult<&str, Ops> {
    let (input, op) = instruction(input)?;
    let (input, _) = hspace0(input)?;
    let (input, _) = opt(comment).parse(input)?;
    Ok((input, op))
}

fn instruction(input: &str) -> IResult<&str, Ops> {
    alt((
        simple_instruction,
        unary_instruction,
        optional_unary_instruction,
        numeric_instruction,
    ))
    .parse(input)
}

fn simple_instruction(input: &str) -> IResult<&str, Ops> {
    alt((
        value(Ops::Stop, tag("STOP")),
        value(Ops::Nop, tag("NOP")),
        value(Ops::Read, tag("READ")),
        value(Ops::Print, tag("PRINT")),
        value(Ops::Sign, tag("SIGN")),
        value(Ops::Abs, tag("ABS")),
        value(Ops::Sqrt, tag("SQRT")),
        value(Ops::Exp, tag("EXP")),
        value(Ops::Log, tag("LOG")),
        value(Ops::Sin, tag("SIN")),
        value(Ops::Cos, tag("COS")),
        value(Ops::Tan, tag("TAN")),
        value(Ops::Floor, tag("FLOOR")),
        value(Ops::Ceil, tag("CEIL")),
        value(Ops::Trunc, tag("TRUNC")),
        value(Ops::Round, tag("ROUND")),
        value(Ops::Rand, tag("RAND")),
    ))
    .parse(input)
}

fn unary_instruction(input: &str) -> IResult<&str, Ops> {
    alt((
        map(write_instruction, Ops::Write),
        map(call_usize("STORE"), Ops::Store),
        map(call_usize("SWAP"), Ops::Swap),
        map(call_usize("COPY"), Ops::Copy),
        map(call_usize("ADD"), Ops::Add),
        map(call_usize("SUB"), Ops::Sub),
        map(call_usize("MULT"), Ops::Mul),
        map(call_usize("DIV"), Ops::Div),
        map(call_usize("POSITIVE"), Ops::Positive),
        map(call_usize("NEGATIVE"), Ops::Negative),
        map(call_usize("ZERO"), Ops::Zero),
    ))
    .parse(input)
}

fn optional_unary_instruction(input: &str) -> IResult<&str, Ops> {
    alt((
        map(call_optional_usize("RECALL"), Ops::Recall),
        map(call_optional_usize("JUMP"), Ops::Jump),
    ))
    .parse(input)
}

fn numeric_instruction(input: &str) -> IResult<&str, Ops> {
    map(call_f32("CONST"), Ops::Const).parse(input)
}

fn call_usize<'a>(
    name: &'static str,
) -> impl Parser<&'a str, Output = usize, Error = nom::error::Error<&'a str>> {
    preceded(tag(name), parens(ws(usize_literal)))
}

fn call_optional_usize<'a>(
    name: &'static str,
) -> impl Parser<&'a str, Output = Option<usize>, Error = nom::error::Error<&'a str>> {
    preceded(tag(name), opt(parens(ws(usize_literal))))
}

fn call_f32<'a>(
    name: &'static str,
) -> impl Parser<&'a str, Output = f32, Error = nom::error::Error<&'a str>> {
    preceded(tag(name), parens(ws(float_literal)))
}

fn write_instruction(input: &str) -> IResult<&str, String> {
    preceded(tag("WRITE"), preceded(hspace1, ws(string_literal))).parse(input)
}

fn parens<'a, O, P>(
    inner: P,
) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
{
    delimited(char('('), inner, ws(char(')')))
}

fn usize_literal(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), str::parse).parse(input)
}

fn float_literal(input: &str) -> IResult<&str, f32> {
    map_res(recognize_float, str::parse).parse(input)
}

fn string_literal(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        escaped_transform(
            is_not("\\\""),
            '\\',
            alt((
                value("\\", char('\\')),
                value("\"", char('"')),
                value("\n", char('n')),
                value("\t", char('t')),
                value("\r", char('r')),
            )),
        ),
        char('"'),
    )
    .parse(input)
}

fn comment(input: &str) -> IResult<&str, ()> {
    value((), (char('#'), take_till(|_| false))).parse(input)
}

fn hspace0(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c == ' ' || c == '\t')(input)
}

fn hspace1(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == ' ' || c == '\t')(input)
}

fn ws<'a, O, P>(inner: P) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
{
    delimited(hspace0, inner, hspace0)
}

#[cfg(test)]
mod tests {
    use super::parse_program;
    use crate::ast::Ops;

    #[test]
    fn parses_mixed_program_with_comments() {
        let source = "# startup\nCONST(4.5)\nSTORE(0)\nRECALL(0) # restore accumulator\nADD(0)\nJUMP(6)\nSTOP";

        let parsed = parse_program(source).unwrap();

        assert_eq!(
            parsed,
            vec![
                Ops::Nop,
                Ops::Const(4.5),
                Ops::Store(0),
                Ops::Recall(Some(0)),
                Ops::Add(0),
                Ops::Jump(Some(6)),
                Ops::Stop,
            ]
        );
    }

    #[test]
    fn parses_optional_operands_without_parentheses() {
        let source = "READ\nRECALL\nJUMP\nPRINT";
        let parsed = parse_program(source).unwrap();

        assert_eq!(
            parsed,
            vec![Ops::Read, Ops::Recall(None), Ops::Jump(None), Ops::Print]
        );
    }

    #[test]
    fn parses_string_and_escaped_characters() {
        let source = r#"WRITE "hello\n\"machine\"""#;
        let parsed = parse_program(source).unwrap();

        assert_eq!(parsed, vec![Ops::Write("hello\n\"machine\"".to_string())]);
    }

    #[test]
    fn parses_heron_example() {
        let source = include_str!("../examples/heron.m");
        let parsed = parse_program(source).unwrap();

        assert_eq!(parsed.first(), Some(&Ops::Jump(Some(6))));
        assert_eq!(parsed.last(), Some(&Ops::Stop));
        assert_eq!(parsed.get(1), Some(&Ops::Nop));
        assert!(parsed.iter().any(|op| matches!(op, Ops::Mul(3))));
    }

    #[test]
    fn inserts_nops_for_empty_lines_and_comment_lines() {
        let source = "READ\n\n# comment\nPRINT";
        let parsed = parse_program(source).unwrap();

        assert_eq!(parsed, vec![Ops::Read, Ops::Nop, Ops::Nop, Ops::Print]);
    }
}
