//! Hash Compiler Frontend library file
//!
//! All rights reserved 2021 (c) The Hash Language authors

use std::fmt::Alignment;

use crate::ast::*;

// const CROSS_PIPE: char = '├';
// const CORNER_PIPE: char = '└';
// const HORIZ_PIPE: char = '─';
const VERT_PIPE: &str = "│";

const END_PIPE: &str = "└─";
const MID_PIPE: &str = "├─";

/// Compile time function to determine which PIPE connector should
/// be used when converting an array of [AstNode]s.
const fn which_connector(index: usize, max_index: usize) -> &'static str {
    if max_index == 0 || index == max_index - 1 {
        END_PIPE
    } else {
        MID_PIPE
    }
}

const fn which_pipe(index: usize, max_index: usize) -> &'static str {
    if max_index == 0 || index == max_index - 1 {
        " "
    } else {
        "│"
    }
}

fn pad_lines(lines: &[String], padding: usize) -> Vec<String> {
    lines
        .iter()
        .map(|line| pad_str(line, ' ', padding, Alignment::Left))
        .collect()
}

/// Utility function to pad a string based on [Alignment]
fn pad_str(line: &str, pad_char: char, padding: usize, alignment: Alignment) -> String {
    // compute side padding based on the alignment
    let (left_pad, right_pad) = match alignment {
        Alignment::Left => (padding, 0),
        Alignment::Right => (0, padding),
        Alignment::Center => (padding / 2, padding / 2),
    };

    // pad the string as specified
    let mut s = String::new();

    for _ in 0..left_pad {
        s.push(pad_char)
    }
    s.push_str(line);
    for _ in 0..right_pad {
        s.push(pad_char)
    }

    s
}

/// utility function to draw parental branch when display the children of the [AstNode]
fn draw_branches_for_lines(lines: &[String], connector: &str, branch: &str) -> Vec<String> {
    let mut next_lines = vec![];

    for (child_index, line) in lines.iter().enumerate() {
        // @@Speed: is this really the best way to deal with string concatination.
        if child_index == 0 {
            next_lines.push(format!("{}{}", connector, line));
        } else {
            // it's only one space here since the 'branch' char already takes one up
            next_lines.push(format!("{}{}", branch, line));
        }
    }

    next_lines
}

pub trait NodeDisplay {
    fn node_display(&self, indent: usize) -> Vec<String>;
}

impl<T: NodeDisplay> NodeDisplay for AstNode<T> {
    fn node_display(&self, indent: usize) -> Vec<String> {
        self.body.node_display(indent)
    }
}

impl<T> std::fmt::Display for AstNode<T>
where
    Self: NodeDisplay,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.node_display(0)
                .into_iter()
                .map(|mut line| {
                    line.push('\n');
                    line
                })
                .collect::<String>()
        )
    }
}

///
/// We need a seperate implementation for [Module] since it won't be wrapped within
/// an [AstNode] unlike all the other variants
///
impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nodes: Vec<Vec<String>> = self.contents.iter().map(|s| s.node_display(0)).collect();

        writeln!(f, "module")?;

        for (count, node) in nodes.iter().enumerate() {
            // determine the connector that should be use to join the nodes
            let (connector, ending) = if count == nodes.len() - 1 {
                (END_PIPE, "")
            } else {
                (MID_PIPE, VERT_PIPE)
            };

            // println!("{:?}", node);
            write!(f, "{}{}\n{}", connector, node.join("\n"), ending)?
        }

        Ok(())
    }
}

impl NodeDisplay for Type {
    fn node_display(&self, _indent: usize) -> Vec<String> {
        match &self {
            Type::Named(_) => todo!(),
            Type::Ref(_) => todo!(),
            Type::TypeVar(_) => todo!(),
            Type::Existential => todo!(),
            Type::Infer => todo!(),
        }
    }
}

impl NodeDisplay for Literal {
    fn node_display(&self, indent: usize) -> Vec<String> {
        let mut lines = vec![];
        let mut next_lines = vec![];

        match &self {
            Literal::Str(s) => lines.push(format!("string \"{}\"", s)),
            Literal::Char(c) => lines.push(format!("char \'{}\'", c)),
            Literal::Int(i) => lines.push(format!("number {}", i)),
            Literal::Float(f) => lines.push(format!("float {}", f)),

            // all these variants have the possibility of containing multiple elements, so
            // we have to evaluate the children first and then construct the branches...
            Literal::Set(SetLiteral { elements })
            | Literal::List(ListLiteral { elements })
            | Literal::Tuple(TupleLiteral { elements }) => {
                // @@Dumbness: rust doesn't allow to bind patterns if there are pattern binds
                // after '@', this can be enabled on Rust nightly, but we aren't that crazy!
                // so we're matching a second time just to get the right literal name
                match &self {
                    Literal::Set(_) => lines.push("set".to_string()),
                    Literal::List(_) => lines.push("list".to_string()),
                    Literal::Tuple(_) => lines.push("tuple".to_string()),
                    _ => unreachable!(),
                };

                // convert all the children and add them to the new lines
                for (index, element) in elements.iter().enumerate() {
                    let connector = which_connector(index, elements.len());

                    // @@Cleanup: make this a function!
                    let branch = if index == elements.len() - 1 {
                        " "
                    } else {
                        "│"
                    };

                    // reset the indent here since we'll be doing indentation here...
                    let child_lines = element.node_display(1);
                    next_lines.extend(draw_branches_for_lines(&child_lines, connector, branch));
                }
            }
            Literal::Map(_map_lit) => {}
            Literal::Struct(_struct_lit) => {}
            Literal::Function(_fn_lit) => {}
        };

        // @@Cleanup: can we make this transformation generic, so we don't have to call it at the end of each implementation??
        // we need to pad each line by the number of spaces specified by 'ident'
        let next_lines: Vec<String> = next_lines
            .into_iter()
            .map(|line| pad_str(line.as_str(), ' ', indent, Alignment::Left))
            .collect();

        lines.extend(next_lines);
        lines
    }
}

impl NodeDisplay for AccessName {
    fn node_display(&self, _indent: usize) -> Vec<String> {
        let names: Vec<&str> = self.names.iter().map(|n| n.body.string.as_ref()).collect();
        vec![names.join("::")]
    }
}

impl NodeDisplay for Statement {
    fn node_display(&self, indent: usize) -> Vec<String> {
        let mut lines = vec![];
        let mut next_lines = vec![];
        let next_indent = indent + 1; // only another indent is added since the other indent is considered as the connector character

        match &self {
            Statement::Expr(expr) => lines.extend(expr.node_display(next_indent)),
            Statement::Return(expr) => {
                lines.push("ret".to_string());

                // if a return statement has a line, display it as the child
                // @@Cleanup: make this a function!
                if let Some(ret_expr) = expr {
                    next_lines.push(format!(
                        "{}{}",
                        END_PIPE,
                        ret_expr.node_display(next_indent).join("\n")
                    ));
                }
            }
            Statement::Block(block) => next_lines.push(format!(
                "{}{}",
                END_PIPE,
                block.node_display(next_indent).join("\n")
            )),
            Statement::Break => lines.push("break".to_string()),
            Statement::Continue => lines.push("continue".to_string()),
            Statement::Let(_decl) => todo!(),
            Statement::Assign(_decl) => todo!(),
            Statement::StructDef(_def) => todo!(),
            Statement::EnumDef(_def) => todo!(),
            Statement::TraitDef(_def) => todo!(),
        };

        // we need to pad each line by the number of spaces specified by 'ident'
        let mut lines: Vec<String> = lines
            .into_iter()
            .map(|line| pad_str(line.as_str(), ' ', indent, Alignment::Left))
            .collect();

        let next_lines: Vec<String> = next_lines
            .into_iter()
            .map(|line| pad_str(line.as_str(), ' ', next_indent, Alignment::Left))
            .collect();

        lines.extend(next_lines);
        lines
    }
}

impl NodeDisplay for Import {
    fn node_display(&self, _indent: usize) -> Vec<String> {
        vec![
            "import".to_string(),
            format!("{} \"{}\"", END_PIPE, self.path),
        ]
    }
}

impl NodeDisplay for Expression {
    fn node_display(&self, indent: usize) -> Vec<String> {
        let mut lines = vec![];

        match &self {
            Expression::FunctionCall(func) => {
                lines.push("function".to_string());

                // deal with the subject of the function call, this is for sure going to
                // be a VariableExpr, so the child branch will be labelled as 'ident'...
                let connector = which_connector(0, func.args.body.entries.len());

                lines.push(format!(" {}subject", connector));

                // deal with the 'subject' as a child and then append it to the next lines
                let subject_lines =
                    draw_branches_for_lines(&func.subject.node_display(2), END_PIPE, " ");

                lines.extend(pad_lines(&subject_lines, 3));

                // now deal with the function args

                lines
            }
            Expression::Intrinsic(intrinsic) => {
                lines.push(format!("intrinsic \"{}\"", intrinsic.name.as_ref()));
                lines
            }
            Expression::Variable(var) => {
                // check if the length of type_args to this ident, if not
                // we don't produce any children nodes for it
                if !var.type_args.is_empty() {
                    todo!()
                } else {
                    let name = var.name.node_display(0).join("");
                    lines.push(format!("ident \"{}\"", name));
                    lines
                }
            }
            Expression::PropertyAccess(_) => todo!(),
            Expression::Ref(expr) | Expression::Deref(expr) => {
                // Match again to determine whether it is a deref or a ref!
                match &self {
                    Expression::Ref(_) => lines.push("ref".to_string()),
                    Expression::Deref(_) => lines.push("deref".to_string()),
                    _ => unreachable!(),
                };

                let next_lines = draw_branches_for_lines(&expr.node_display(indent), END_PIPE, "");
                lines.extend(pad_lines(&next_lines, 1));

                lines
            }
            Expression::LiteralExpr(literal) => literal.node_display(indent),
            Expression::Typed(_) => todo!(),
            Expression::Block(block) => block.node_display(indent),
            Expression::Import(import) => import.node_display(indent),
        }
    }
}

impl NodeDisplay for Block {
    fn node_display(&self, indent: usize) -> Vec<String> {
        match &self {
            Block::Match(_match_body) => todo!(),
            Block::Loop(_loop_body) => {
                // first of all, we need to call format on all of the children statements
                // of the block and then compute the height of the formatted string by
                // just checking the number of lines that are in the resultant string.
                // let statements = ;
                todo!()
            }
            Block::Body(body) => body.node_display(indent),
        }
    }
}

impl NodeDisplay for BodyBlock {
    fn node_display(&self, indent: usize) -> Vec<String> {
        let mut lines = vec!["block".to_string()]; // do we need an initial connector here?
        let mut next_lines = vec![];

        for (index, statement) in self.statements.iter().enumerate() {
            // special case when the block contains an expression, we should use the MID_PIPE instead
            // of the end pipe
            let connector = if matches!(self.expr, Some(_)) && self.statements.len() - 1 == index {
                MID_PIPE
            } else {
                which_connector(index, self.statements.len())
            };

            let branch = if matches!(self.expr, None) {
                " "
            } else {
                which_pipe(index, self.statements.len())
            };

            // reset the indent here since we'll be doing indentation here...
            let lines = statement.node_display(0);
            next_lines.extend(draw_branches_for_lines(&lines, connector, branch));
        }

        // check if body block has an expression at the end
        if let Some(expr) = &self.expr {
            let child_lines = expr.node_display(1);
            next_lines.extend(draw_branches_for_lines(&child_lines, END_PIPE, " "));
        }

        // @@Cleanup: can we make this transformation generic, so we don't have to call it at the end of each implementation??
        // we need to pad each line by the number of spaces specified by 'ident'
        let next_lines: Vec<String> = next_lines
            .into_iter()
            .map(|line| pad_str(line.as_str(), ' ', indent, Alignment::Left))
            .collect();

        lines.extend(next_lines);
        lines
    }
}
