use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while1;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::character::complete::{char, one_of};
use nom::character::is_alphabetic;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::{many0, many1};
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use crate::commands::{Command, Expr, PrintFormat};

fn ident<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    take_while1(|c: char| is_alphabetic(c as u8) || c == '_')(input)
}

fn parse_name_expr<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    map(ident, |n| Expr::Name(n))(input)
}

fn parse_parens_expr<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    delimited(tag("("), parse_expr, tag(")"))(input)
}

fn parse_str_expr<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    map(delimited(tag("\""), take_until("\""), tag("\"")), |v| {
        Expr::Str(v)
    })(input)
}

fn parse_int_expr<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    map(digit1, |v: &str| {
        let v = v.parse::<i64>().unwrap();
        Expr::Int(v)
    })(input)
}

fn parse_hex_expr<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    let hex = preceded(
        alt((tag("0x"), tag("0X"))),
        recognize(many1(terminated(
            one_of("0123456789abcdefABCDEF"),
            many0(char('_')),
        ))),
    );
    map(hex, |v: &'a str| {
        let v = u64::from_str_radix(&str::replace(&v, "_", ""), 16).unwrap();
        Expr::Hex(v)
    })(input)
}

fn parse_expr<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    alt((
        parse_expr_member_access,
        parse_expr_cast,
        parse_str_expr,
        parse_parens_expr,
        parse_expr_deref,
        parse_hex_expr,
        parse_name_expr,
        parse_int_expr,
    ))(input)
}

fn parse_expr_deref<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    map(preceded(tag("*"), parse_expr), |expr| {
        Expr::Deref(Box::new(expr))
    })(input)
}

fn parse_expr_cast<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    let (input, type_) = delimited(tag("("), take_until(")"), tag(")"))(input)?;
    let (input, target) = preceded(space0, parse_expr)(input)?;
    Ok((input, Expr::Cast(type_, Box::new(target))))
}

/// Member access lhs accepts a single expression or a parenthesed expression
fn parse_expr_member_access<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    let object = alt((parse_parens_expr, parse_name_expr));

    map(
        tuple((object, tag("->"), ident)),
        |(object, _, member_access)| Expr::MemberAccess(Box::new(object), member_access),
    )(input)
}

pub(crate) fn parse_command<'a>(input: &'a str) -> IResult<&'a str, Command<'a>> {
    let (input, word) = alpha1(input)?;

    Ok(match word {
        "bt" => (input, Command::Backtrace),
        "x" => {
            let (input, params) = opt(preceded(tag("/"), pair(digit1, opt(alpha1))))(input)?;
            let params = if let Some((number, format)) = params {
                let number = number.parse::<u32>().unwrap();
                let format = match format {
                    Some("s") => Some(PrintFormat::String),
                    Some(_) => unimplemented!(),
                    _ => None,
                };

                (Some(number), format)
            } else {
                (None, None)
            };

            let (input, what) = preceded(space1, parse_expr)(input)?;
            (input, Command::Examine(what, params))
        }
        "p" => {
            let (input, format) = opt(tag("/s"))(input)?;

            let format = if let Some(format) = format {
                match format {
                    "/s" => PrintFormat::String,
                    e => unimplemented!("unknow format {}", e),
                }
            } else {
                PrintFormat::None
            };

            let (input, what) = preceded(space1, parse_expr)(input)?;
            (input, Command::Print(format, what))
        }
        "f" => {
            let (input, n) = preceded(tag(" "), digit1)(input)?;
            let n = n.parse::<usize>().unwrap();

            (input, Command::SelectFrame(n))
        }
        "info" => {
            let (input, what) = preceded(space1, ident)(input)?;
            let (input, arg0) = opt(preceded(space1, parse_expr))(input)?;
            let args = if let Some(arg0) = arg0 {
                vec![arg0]
            } else {
                vec![]
            };

            (input, Command::Info(what, args))
        }
        "find" => {
            let start = opt(terminated(parse_expr, tag(", ")));
            let end = opt(terminated(parse_expr, tag(", ")));
            let (input, (start, end, expr)) =
                preceded(space0, tuple((start, end, parse_expr)))(input)?;

            (input, Command::Find(start, end, expr))
        }
        _ => (input, Command::Unknown),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_name() {
        use Expr::*;

        let (_, cmd) = parse_expr("test").unwrap();
        assert_eq!(cmd, Name("test"));
    }

    #[test]
    fn test_expr_member_access() {
        use Expr::*;

        let (_, cmd) = parse_expr("test->ab_").unwrap();
        assert_eq!(cmd, MemberAccess(Box::new(Name("test")), "ab_"));
    }

    #[test]
    fn test_expr_deref_name() {
        use Expr::*;

        let (_, cmd) = parse_expr("*test").unwrap();
        assert_eq!(cmd, Deref(Box::new(Name("test"))));
    }

    #[test]
    fn test_expr_deref_member_access() {
        use Expr::*;

        let (_, cmd) = parse_expr("*test->ab").unwrap();
        assert_eq!(
            cmd,
            Deref(Box::new(MemberAccess(Box::new(Name("test")), "ab")))
        );
    }

    #[test]
    fn test_expr_deref_parens_deref() {
        use Expr::*;

        let (_, cmd) = parse_expr("*(*test)").unwrap();
        assert_eq!(cmd, Deref(Box::new(Deref(Box::new(Name("test"))))));
    }

    #[test]
    fn test_expr_deref_parens_member_access() {
        use Expr::*;

        let (_, cmd) = parse_expr("(*test)->ab").unwrap();
        assert_eq!(
            cmd,
            MemberAccess(Box::new(Deref(Box::new(Name("test")))), "ab")
        );
    }

    #[test]
    fn test_expr_cast() {
        use Expr::*;

        let (_, expr) = parse_expr("(float) var").unwrap();
        assert_eq!(expr, Cast("float", Box::new(Name("var"))));

        let (_, expr) = parse_expr("(float)var").unwrap();
        assert_eq!(expr, Cast("float", Box::new(Name("var"))));

        let (_, expr) = parse_expr("(a::float<usize>) 0x1").unwrap();
        assert_eq!(expr, Cast("a::float<usize>", Box::new(Hex(1))));
    }

    #[test]
    fn test_expr_member_access_cast() {
        use Expr::*;

        let (_, cmd) = parse_expr("((usize) 0x3)->ab").unwrap();
        assert_eq!(
            cmd,
            MemberAccess(Box::new(Cast("usize", Box::new(Hex(3)))), "ab")
        );
    }

    #[test]
    fn test_print_var() {
        use Command::*;
        use Expr::*;

        let (_, cmd) = parse_command("p var").unwrap();
        assert_eq!(cmd, Print(PrintFormat::None, Name("var")));
    }

    #[test]
    fn test_print_var_as_string() {
        use Command::*;
        use Expr::*;

        let (_, cmd) = parse_command("p/s var").unwrap();
        assert_eq!(cmd, Print(PrintFormat::String, Name("var")));
    }

    #[test]
    fn test_info_types() {
        use Command::*;

        let (_, cmd) = parse_command("info types").unwrap();
        assert_eq!(cmd, Info("types", vec![]));
    }

    #[test]
    fn test_info_symbol() {
        use Command::*;

        let (_, cmd) = parse_command("info symbol 1234").unwrap();
        assert_eq!(cmd, Info("symbol", vec![Expr::Int(1234)]));
    }

    #[test]
    fn test_find() {
        use Command::*;

        let (_, cmd) = parse_command("find \"a\"").unwrap();
        assert_eq!(cmd, Find(None, None, Expr::Str("a")));

        let (_, cmd) = parse_command("find 0x12").unwrap();
        assert_eq!(cmd, Find(None, None, Expr::Hex(0x12)));
    }
}
