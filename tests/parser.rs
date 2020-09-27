
use rust_lisp::{parse, model::{Value,List}};

#[macro_use]
extern crate rust_lisp;

#[test]
fn parse_basic_expression() {
  let ast = parse("
    (list 
      (* 1 2)  ;; a comment
      (/ 6 3 \"foo\"))").next().unwrap().unwrap();

  assert_eq!(ast, lisp! {
    (list 
      (* 1 2)
      (/ 6 3 "foo"))
  });
}

#[test]
fn parse_nil() {
  let source = "()";
  let ast = parse(source).next().unwrap().unwrap();

  assert_eq!(ast, Value::NIL);
}

#[test]
fn parse_atom() {
  let source = "12";
  let ast = parse(source).next().unwrap().unwrap();

  assert_eq!(ast, Value::Int(12));
}

#[test]
fn parse_multiple_lines() {
  let ast = parse("
    (print 1)
    (print 2)
    (print 3)").collect::<Result<Vec<_>, _>>().unwrap();


  assert_eq!(ast, vec![
    lisp! { (print 1) },
    lisp! { (print 2) },
    lisp! { (print 3) },
  ]);
}
