use crate::executor::ast::Expression::*;
use crate::executor::ast::SourceUnitPart::Statement;
use crate::executor::ast::Statement::*;
use crate::executor::ast::*;
use crate::lexer::tokens::TokenType::Number;
use crate::lexer::Lexer;
use crate::parser::parse;

#[test]
fn parser_test() {
    let code = "
        pwoli_sadhanam i;
        i=0;
        seriyano_mwone i um 0 um same_alle {
            i = 10;
        } seri_allel {
            i = -1;
        }
        repeat_adi i um 0 um same_alle {
            i = i-1;
        }
        dhe_pidicho i;
    ";
    let lex = Lexer::new(&code);
    let parsed = parse(&code, lex);
    println!("{:?}", parsed);
    let expected = SourceUnit(
        [
            Statement(Declaration((9, 26), Symbol((24, 24), "i"))),
            Statement(Assignment(
                (35, 39),
                Symbol((35, 35), "i"),
                Integer((37, 38), Number(0)),
            )),
            Statement(Conditional(
                (48, 157),
                NotEquals(
                    (73, 81),
                    Box::new(Symbol((63, 63), "i")),
                    Box::new(Integer((68, 69), Number(0))),
                ),
                SourceUnit(
                    [Statement(Assignment(
                        (97, 104),
                        Symbol((97, 97), "i"),
                        Integer((101, 103), Number(10)),
                    ))]
                    .to_vec(),
                ),
                Some(SourceUnit(
                    [Statement(Assignment(
                        (140, 147),
                        Symbol((140, 140), "i"),
                        UnaryMinus((144, 145), Box::new(Integer((145, 146), Number(1)))),
                    ))]
                    .to_vec(),
                )),
            )),
            Statement(Loop(
                (166, 229),
                NotEquals(
                    (187, 195),
                    Box::new(Symbol((177, 177), "i")),
                    Box::new(Integer((182, 183), Number(0))),
                ),
                SourceUnit(
                    [Statement(Assignment(
                        (211, 219),
                        Symbol((211, 211), "i"),
                        Subtract(
                            (216, 217),
                            Box::new(Symbol((215, 215), "i")),
                            Box::new(Integer((217, 218), Number(1))),
                        ),
                    ))]
                    .to_vec(),
                ),
            )),
            Statement(Write((238, 252), Symbol((250, 250), "i"))),
        ]
        .to_vec(),
    );
    assert_eq!(expected, parsed.unwrap());
}
