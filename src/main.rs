use core::fmt;
use std::collections::{HashMap, VecDeque};

use actix_web::{
    post,
    web::{self, Redirect},
    App, HttpResponse, HttpServer, Responder,
};
use ego_tree::{NodeId, NodeRef};
use scraper::{Html, Node, Selector};

#[derive(Debug, Clone)]
pub enum HtmlValueType {
    Str(String),
    Int(i64),
    Bool(bool),
    Null,
    Array(Vec<HtmlValueType>),
}

impl fmt::Display for HtmlValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            HtmlValueType::Str(v) => write!(f, "{}", v),
            HtmlValueType::Int(v) => write!(f, "{}", v),
            HtmlValueType::Bool(v) => write!(f, "{}", v),
            HtmlValueType::Null => write!(f, "null"),
            HtmlValueType::Array(v) => {
                write!(
                    f,
                    "[{}]",
                    v.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        };
    }
}

impl PartialEq for HtmlValueType {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (HtmlValueType::Str(v1), HtmlValueType::Str(v2)) => v1 == v2,
            (HtmlValueType::Int(v1), HtmlValueType::Int(v2)) => v1 == v2,
            (HtmlValueType::Bool(v1), HtmlValueType::Bool(v2)) => v1 == v2,
            (HtmlValueType::Null, HtmlValueType::Null) => true,
            (HtmlValueType::Array(v1), HtmlValueType::Array(v2)) => {
                if v1.len() != v2.len() {
                    return false;
                }
                for (v1, v2) in v1.iter().zip(v2) {
                    if v1 != v2 {
                        return false;
                    }
                }
                return true;
            }
            _ => false,
        };
    }
}

#[post("/exec_html_the_programming_language")]
async fn exec_html_the_programming_language_handler(req_body: String) -> impl Responder {
    let html_doc = Html::parse_fragment(&req_body);
    let mut stack = VecDeque::<HtmlValueType>::new();
    let mut cursors_stack = VecDeque::<Option<NodeId>>::new();
    let mut vars_context = HashMap::<String, HtmlValueType>::new();
    let mut stdout = String::new();

    let select_main = Selector::parse("main").unwrap();
    let select_dfn = Selector::parse("dfn").unwrap();

    let mut document_select = html_doc.select(&select_main);
    let main_node = match document_select.next() {
        Some(v) => v,
        None => {
            return HttpResponse::BadRequest()
                .content_type("text/plain")
                .body("ERROR: `main` tag is required".to_string())
        }
    };

    // TODO: Terminar de detectar dfn dentro de main
    match main_node.select(&select_dfn).next() {
        Some(_) => {}
        None => {}
    }

    let mut cursor_node = match main_node.first_child() {
        Some(n) => Some(n.id()),
        None => return HttpResponse::Ok().body(""),
    };

    match html_exec_loop(
        &html_doc,
        &mut cursor_node,
        &mut stack,
        &mut cursors_stack,
        &mut vars_context,
        &mut stdout,
    ) {
        Ok(()) => HttpResponse::Ok().content_type("text/plain").body(stdout),
        Err(err) => HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(format!("ERROR: {}", err)),
    }
}

async fn redirect_to_index() -> impl Responder {
    Redirect::to("/").permanent()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(exec_html_the_programming_language_handler)
            .service(actix_files::Files::new("/", "./public").index_file("index.html"))
            .default_service(web::to(redirect_to_index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

pub fn html_exec_loop(
    document: &Html,
    cursor_node: &mut Option<NodeId>,
    stack: &mut VecDeque<HtmlValueType>,
    cursors_stack: &mut VecDeque<Option<NodeId>>,
    vars_context: &mut HashMap<String, HtmlValueType>,
    stdout: &mut String,
) -> Result<(), String> {
    loop {
        if cursor_node.is_none() {
            return Ok(());
        };
        if let Some(node) = document.tree.get(cursor_node.unwrap()) {
            if let Some(next_node) = node.next_sibling() {
                *cursor_node = Some(next_node.id());
            } else {
                return Ok(());
            }

            exec_html_instructions(
                &document,
                cursor_node,
                &node,
                stack,
                cursors_stack,
                vars_context,
                stdout,
            )?;
        } else {
            return Ok(());
        }
    }
}

pub fn exec_html_instructions(
    document: &Html,
    cursor_node: &mut Option<NodeId>,
    node: &NodeRef<Node>,
    stack: &mut VecDeque<HtmlValueType>,
    cursors_stack: &mut VecDeque<Option<NodeId>>,
    vars_context: &mut HashMap<String, HtmlValueType>,
    stdout: &mut String,
) -> Result<(), String> {
    if let Some(element) = node.value().as_element() {
        match element.name() {
                // Valores
                "s" => {
                    match node.first_child() {
                        Some(v) => match v.value().as_text() {
                            Some(v) => stack.push_front(HtmlValueType::Str(v.to_string())),
                            None => return Err("The `s` tag child must be plain text".into()),
                        },
                        None => stack.push_front(HtmlValueType::Str("".into())),
                    };
                }
                "data" => {
                    match element.attr("value") {
                        Some(v) => match v.parse::<i64>() {
                            Ok(v) => stack.push_front(HtmlValueType::Int(v)),
                            Err(_) => {
                                return Err(
                                    "The `value` attribute must containt a valid integer".into()
                                )
                            }
                        },
                        None => return Err("The data must have the `value` attribute".into()),
                    };
                }
                "ol" => {
                    let original_len = stack.len();
                    for ol_child_node in node.children() {
                        if let Some(ol_child_element) = ol_child_node.value().as_element() {
                            if ol_child_element.name() == "li" {
                                for il_child_node in ol_child_node.children() {
                                    if il_child_node.value().is_element() {
                                        exec_html_instructions(document, cursor_node, &il_child_node, stack, cursors_stack, vars_context,stdout)?;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    let diff = stack.len() - original_len;
                    if diff > 0 {
                        let mut arr = Vec::<HtmlValueType>::with_capacity(diff);
                        for _ in 0..diff {
                            arr.push(stack.pop_front().unwrap());
                        }
                        arr.reverse();
                        stack.push_front(HtmlValueType::Array(arr));
                    }
                }

                // Comandos
                "cite" => {
                    match node.first_child() {
                        Some(v) => match v.value().as_text() {
                            Some(v) => match v.to_string().as_str() {
                                "true" => stack.push_front(HtmlValueType::Bool(true)),
                                "false" => stack.push_front(HtmlValueType::Bool(false)),
                                "null" => stack.push_front(HtmlValueType::Null),
                                var_name => {
                                    if let Some(v) = vars_context.get(&var_name.to_string()) {
                                        stack.push_front(v.clone());
                                        return Ok(());
                                    }
                                    stack.push_front(HtmlValueType::Null);
                                }
                            },
                            None => {
                                return Err("The `cite` tag child must be boolean or null".into())
                            }
                        },
                        None => return Err("The `cite` tag child must be boolean or null".into())
                    };
                }
                //io
                "output" => {
                    match stack.front() {
                        Some(v) => stdout.push_str(&format!("{}\n", v)),
                        None => {
                            return Err("Attemp to peek an empty stack from `output` tag".into())
                        }
                    };
                }

                // Operaciones del stack
                "dt" => {
                    if let Some(v) = stack.front() {
                        stack.push_front(v.clone());
                    }
                }
                "del" => {
                    stack.pop_front();
                }

                // Comparacion
                "small" => {
                    match (stack.pop_front(), stack.pop_front()) {
                        (Some(HtmlValueType::Int(v1)), Some(HtmlValueType::Int(v2))) => {
                            stack.push_front(HtmlValueType::Bool(v1 > v2));
                        }
                        (Some(HtmlValueType::Str(v1)), Some(HtmlValueType::Str(v2))) => {
                            stack.push_front(HtmlValueType::Bool(v1.len() > v2.len()));
                        }
                        (Some(HtmlValueType::Array(v1)), Some(HtmlValueType::Array(v2))) => {
                            stack.push_front(HtmlValueType::Bool(v1.len() > v2.len()));
                        }
                        _ => return Err(
                            "Fail to compare with `small`, the stack must have at least two values valid compareable values"
                                .into(),
                        ),
                    }
                }
                "big" => match (stack.pop_front(), stack.pop_front()) {
                    (Some(HtmlValueType::Int(v1)), Some(HtmlValueType::Int(v2))) => {
                        stack.push_front(HtmlValueType::Bool(v1 < v2));
                    }
                    (Some(HtmlValueType::Str(v1)), Some(HtmlValueType::Str(v2))) => {
                        stack.push_front(HtmlValueType::Bool(v1.len() < v2.len()));
                    }
                    (Some(HtmlValueType::Array(v1)), Some(HtmlValueType::Array(v2))) => {
                        stack.push_front(HtmlValueType::Bool(v1.len() < v2.len()));
                    }
                    _ => {
                        return Err("Fail to compare with `big`, the stack must have at least two values valid compareable values".into())
                    }
                },
                "em" => match (stack.pop_front(), stack.pop_front()) {
                    (Some(v1), Some(v2)) => stack.push_front(HtmlValueType::Bool(v1 == v2)),
                    _ => {
                        return Err("Fail to compare with `em`, the stack must have at least two valid compareable values".into())
                    }
                },

                // Aritmetica
                "dd" => match (stack.pop_front(), stack.pop_front()) {
                    (Some(HtmlValueType::Int(v1)), Some(HtmlValueType::Int(v2))) => {
                        stack.push_front(HtmlValueType::Int(v2.saturating_add(v1)));
                    },
                    (Some(HtmlValueType::Str(v1)), Some(HtmlValueType::Str(v2))) => {
                        stack.push_front(HtmlValueType::Str(v2+&v1));
                    },
                    (Some(HtmlValueType::Array(mut v1)), Some(HtmlValueType::Array(mut v2))) => {
                        v2.append(&mut v1);
                        stack.push_front(HtmlValueType::Array(v2));
                    }
                    _ => {
                        return Err("Fail to add, the stack must have at least two intergers".into())
                    }
                },
                "sub" => match (stack.pop_front(), stack.pop_front()) {
                    (Some(HtmlValueType::Int(v1)), Some(HtmlValueType::Int(v2))) => {
                        stack.push_front(HtmlValueType::Int(v2.saturating_sub(v1)));
                    }
                    _ => {
                        return Err("Fail to substract, the stack must have at least two intergers".into())
                    }
                },
                "ul" => match (stack.pop_front(), stack.pop_front()) {
                    (Some(HtmlValueType::Int(v1)), Some(HtmlValueType::Int(v2))) => {
                        stack.push_front(HtmlValueType::Int(v1.saturating_mul(v2)));
                    }
                    _ => {
                        return Err("Fail to multiply, the stack must have at least two intergers".into())
                    }
                },
                "div" => match (stack.pop_front(), stack.pop_front()) {
                    (Some(HtmlValueType::Int(v1)), Some(HtmlValueType::Int(v2))) => {
                        stack.push_front(HtmlValueType::Int(v2.saturating_div(v1)));
                    }
                    _ => {
                        return Err("Fail to divied, the stack must have at least two intergers".into())
                    }
                },

                // Operaciones logicas
                "b" => match (stack.pop_front(), stack.pop_front()) {
                    (Some(HtmlValueType::Bool(v1)), Some(HtmlValueType::Bool(v2))) => {
                        stack.push_front(HtmlValueType::Bool(v1 && v2));
                    }
                    _ => {
                        return Err("Fail logical and, the stack must have at least two booleans".into())
                    }
                },
                "bdo" => match (stack.pop_front(), stack.pop_front()) {
                    (Some(HtmlValueType::Bool(v1)), Some(HtmlValueType::Bool(v2))) => {
                        stack.push_front(HtmlValueType::Bool(v1 || v2));
                    }
                    _ => {
                        return Err("Fail logical or, the stack must have at least two booleans".into())
                    }
                },
                "dbi" => match stack.pop_front() {
                    Some(HtmlValueType::Bool(v1)) => {
                        stack.push_front(HtmlValueType::Bool(!v1));
                    }
                    _ => {
                        return Err("Fail logical not, the stack must have at least two booleans".into())
                    }
                },

                // Flujo de control
                "i" => {
                    if let Some(HtmlValueType::Bool(v)) = stack.pop_front() {
                        if v {
                            for i_child_node in node.children() {
                                exec_html_instructions(document, cursor_node, &i_child_node, stack, cursors_stack, vars_context, stdout)?;
                            }
                        }
                    }
                }

                "a" => {
                    let mut html_tag_id = match element.attr("href") {
                        Some(v) => v.to_string(),
                        None => return Err("".into()),
                    };

                    if html_tag_id.starts_with("javascript:") {
                        for child in node.children() {
                            exec_html_instructions(document, cursor_node, &child, stack, cursors_stack, vars_context, stdout)?;
                       }

                        match node.next_sibling() {
                            Some(v) => cursors_stack.push_front(Some(v.id())),
                            None => cursors_stack.push_front(None),
                        };

                        html_tag_id = html_tag_id.trim_start_matches("javascript:").to_owned();
                        html_tag_id = html_tag_id.trim_end_matches("()").to_owned();
                        html_tag_id = "#".to_owned() + &html_tag_id;
                    }
                    let selector = match Selector::parse(&html_tag_id) {
                        Ok(s) => s,
                        Err(_) => {
                            return Err("".into());
                        },
                    };

                   let target_node = document.select(&selector).next().unwrap();
                    *cursor_node = Some(target_node.id());
                }

                "rt" => {
                    *cursor_node = None;
                }

                "var" => {
                    match stack.pop_front() {
                        Some(value) => {
                            let var_name = match element.attr("title") {
                                Some(v) => v,
                                None => return Err("".into()),
                            };
                            vars_context.insert(var_name.to_string(), value);
                        },
                        None => todo!(),
                    }
                }

                "dfn" => {
                    *cursor_node = Some(node.first_child().unwrap().id());

                    let return_point = cursors_stack.front().cloned();
                    html_exec_loop(document, cursor_node, stack, cursors_stack, &mut vars_context.clone(), stdout)?;
                    // for dfn_child_node in node.children() {
                    //     exec_html_instructions(document, cursor_node, &dfn_child_node, stack, cursors_stack, &new_vars_context)?;
                    // }

                    match cursors_stack.pop_front() {
                        Some(v) => *cursor_node = v,
                        None => match return_point {
                            Some(v) => *cursor_node = v,
                            None => return Err("NO RETURN POINT".to_string())                            
                        },
                    }
                }

                _ => {
                     print!("Unknow {} element", element.name());
                    return Ok(());
                },
            }
    }

    Ok(())
}
