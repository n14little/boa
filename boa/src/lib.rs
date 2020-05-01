//! This is an experimental Javascript lexer, parser and compiler written in Rust. Currently, it has support for some of the language.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/jasonwilliams/boa/master/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/jasonwilliams/boa/master/assets/logo.svg"
)]
#![deny(
    unused_qualifications,
    clippy::all,
    unused_qualifications,
    unused_import_braces,
    unused_lifetimes,
    unreachable_pub,
    trivial_numeric_casts,
    // rustdoc,
    missing_debug_implementations,
    missing_copy_implementations,
    deprecated_in_future,
    meta_variable_misuse,
    non_ascii_idents,
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]
#![warn(clippy::perf, clippy::single_match_else, clippy::dbg_macro)]
#![allow(
    clippy::missing_inline_in_public_items,
    clippy::cognitive_complexity,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::as_conversions,
    missing_doc_code_examples
)]

pub mod builtins;
pub mod environment;
pub mod exec;
pub mod realm;
pub mod syntax;
use crate::{
    builtins::value::ResultValue,
    exec::{Executor, Interpreter},
    realm::Realm,
    syntax::{ast::node::Node, lexer::Lexer, parser::Parser},
};

fn parser_expr(src: &str) -> Result<Node, String> {
    let mut lexer = Lexer::new(src);
    lexer.lex().map_err(|e| format!("SyntaxError: {}", e))?;
    let tokens = lexer.tokens;
    Parser::new(&tokens)
        .parse_all()
        .map_err(|e| format!("ParsingError: {}", e))
}

/// Execute the code using an existing Interpreter
/// The str is consumed and the state of the Interpreter is changed
pub fn forward(engine: &mut Interpreter, src: &str) -> String {
    // Setup executor
    let expr = match parser_expr(src) {
        Ok(v) => v,
        Err(error_string) => {
            return error_string;
        }
    };
    let result = engine.run(&expr);
    match result {
        Ok(v) => v.to_string(),
        Err(v) => format!("{}: {}", "Error", v.to_string()),
    }
}

/// Execute the code using an existing Interpreter.
/// The str is consumed and the state of the Interpreter is changed
/// Similar to `forward`, except the current value is returned instad of the string
/// If the interpreter fails parsing an error value is returned instead (error object)
pub fn forward_val(engine: &mut Interpreter, src: &str) -> ResultValue {
    // Setup executor
    match parser_expr(src) {
        Ok(expr) => engine.run(&expr),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

/// Create a clean Interpreter and execute the code
pub fn exec(src: &str) -> String {
    // Create new Realm
    let realm = Realm::create();
    let mut engine: Interpreter = Executor::new(realm);
    forward(&mut engine, src)
}

/// FIXME: Temporary spot for BigInt structure
use gc::{unsafe_empty_trace, Finalize, Trace};
use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq)]
pub struct BigInt(rug::Integer);

impl BigInt {
    #[inline]
    pub fn into_inner(self) -> rug::Integer {
        self.0
    }
}

impl std::ops::Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        BigInt(self.0 + other.0)
    }
}

impl std::ops::Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        BigInt(self.0 - other.0)
    }
}

impl std::ops::Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        BigInt(self.0 * other.0)
    }
}

impl std::ops::Div for BigInt {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        BigInt(self.0 / other.0)
    }
}

impl std::ops::Rem for BigInt {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        BigInt(self.0 % other.0)
    }
}

impl std::ops::BitAnd for BigInt {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        BigInt(self.0 & other.0)
    }
}

impl std::ops::BitOr for BigInt {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        BigInt(self.0 | other.0)
    }
}

impl std::ops::BitXor for BigInt {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        BigInt(self.0 | other.0)
    }
}

impl std::ops::Shr for BigInt {
    type Output = Self;

    fn shr(self, other: Self) -> Self::Output {
        let other = other.0.to_i32().unwrap_or(std::i32::MAX);
        BigInt(self.0 >> other)
    }
}

impl std::ops::Shl for BigInt {
    type Output = Self;

    fn shl(self, other: Self) -> Self::Output {
        match other.0.to_i32() {
            Some(n) => BigInt(self.0 << n),
            None => panic!("RangeError: Maximum BigInt size exceeded"),
        }
    }
}

impl std::ops::Neg for BigInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl PartialEq<i32> for BigInt {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<BigInt> for i32 {
    fn eq(&self, other: &BigInt) -> bool {
        *self == other.0
    }
}

impl PartialEq<f64> for BigInt {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<BigInt> for f64 {
    fn eq(&self, other: &BigInt) -> bool {
        *self == other.0
    }
}

impl std::fmt::Debug for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for BigInt {
    type Target = rug::Integer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BigInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Finalize for BigInt {}
unsafe impl Trace for BigInt {
    unsafe_empty_trace!();
}
