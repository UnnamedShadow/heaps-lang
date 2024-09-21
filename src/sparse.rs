use proc_macro::{Group, TokenStream, TokenTree};

#[derive(Debug, Clone)]
pub enum Statement {
    VarUsage {
        name: String,
    },
    FunctionCall {
        function: Box<Statement>,
        args: Vec<Statement>,
    },
    None,
    Literal {
        content: String,
    },
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Statement>,
}

fn parse_statement(data: Vec<TokenTree>) -> Statement {
    match data.len() {
        0 => Statement::None,
        1 => match data[0].clone() {
            TokenTree::Literal(lit) => {
                let s = lit.to_string();
                let mut l = s.split('"');
                l.next();
                Statement::Literal {
                    content: l.next().unwrap().to_string(),
                }
            }
            _ => Statement::VarUsage {
                name: data[0].to_string(),
            },
        },
        _ => match data.last().unwrap() {
            TokenTree::Group(group) => {
                let mut args = vec![];
                let mut last = None;
                for t in group.stream() {
                    if t.to_string() == "," {
                        args.push(last.unwrap());
                        last = Some(vec![]);
                    } else {
                        last = last.map_or(Some(vec![t.clone()]), |l| {
                            let mut a = l;
                            a.push(t);
                            Some(a)
                        });
                    }
                }
                last.map(|x| args.push(x));
                Statement::FunctionCall {
                    function: Box::new(parse_statement(data[0..data.len() - 1].to_vec())),
                    args: args.iter().map(|x| parse_statement(x.clone())).collect(),
                }
            }
            _ => panic!("invalid statement"),
        },
    }
}

fn parse_body(group: Group) -> Vec<Statement> {
    let mut lines = vec![];
    let mut last = vec![];
    for i in group.stream() {
        if i.to_string() == ";" {
            lines.push(last);
            last = vec![];
        } else {
            last.push(i);
        }
    }
    lines.push(last);

    lines.iter().map(|x| parse_statement(x.clone())).collect()
}

pub fn sparse(input: TokenStream) -> Vec<Function> {
    let mut functions = vec![];
    let mut last = vec![];
    for token in input {
        last.push(token);
        if last.len() == 4 {
            if last[2].to_string() != "=" {
                panic!("the third token should be an '='")
            }
            match last[1].clone() {
                TokenTree::Group(args) => match last[3].clone() {
                    TokenTree::Group(body) => functions.push(Function {
                        name: last[0].to_string(),
                        args: args.stream().into_iter().map(|x| x.to_string()).collect(),
                        body: parse_body(body),
                    }),
                    _ => {
                        panic!("body not a group")
                    }
                },
                _ => {
                    panic!("args not a group")
                }
            }
            last = vec![];
        }
    }
    functions
}
