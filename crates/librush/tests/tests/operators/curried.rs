//! Tests for the curried versions of operators.

use util::*;


#[test]
fn left_simple() {
    assert_eq!("4", eval("(2+) $ 2"));
    assert_eq!("9", eval("((3*))(3)"));  // function call form
    assert_eq!("foofoofoo", eval("(foo*) $ 3"));
}

#[test]
fn right_simple() {
    assert_eq!("4", eval("(*2) $ 2"));
    assert_eq!("6", eval("((*2))(3)"));  // function call form
    assert_eval_error("(+42) $ 11");  // unary expression!
    assert_eq!("aaaaa", eval("(*5) $ a"));
}

#[test]
fn none_simple() {
    assert_eq!("9", eval("(+) $ 5 $ 4"));  // just application
    assert_eq!("7", eval("((+) $ 3)(4)"));  // hybrid: application + call
    assert_eq!("7", eval("((+))(3) $ 4"));  // hybrid: call + application
    assert_eq!("5", eval("((+))(1)(4)"));  // just function calls
}

#[test]
fn map_with_left() {
    let input = [1, 2, 3].to_literal();
    assert_eq!(unlines![2, 3, 4], eval(&format!("map((1+), {})", input)));
    assert_eq!(unlines![2, 4, 6], eval(&format!("map((2*), {})", input)));
    assert_eq!(unlines![3, 9, 27], eval(&format!("map((3**), {})", input)));
    assert_eq!(unlines!["x", "xx", "xxx"], eval(&format!("map((x*), {})", input)));
}

#[test]
fn map_with_right() {
    let input = [2, 4, 6].to_literal();
    assert_eval_error(&format!("map((+1), {})", input));  // unary expression!
    assert_eq!(unlines![4, 8, 12], eval(&format!("map((*2), {})", input)));
    assert_eq!(unlines![8, 64, 216], eval(&format!("map((**3), {})", input)));
    assert_eq!(unlines![1, 2, 3], eval(&format!("map((/2), {})", input)));
}
