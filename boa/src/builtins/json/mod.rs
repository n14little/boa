//! This module implements the global `JSON` object.
//!
//! The `JSON` object contains methods for parsing [JavaScript Object Notation (JSON)][spec]
//! and converting values to JSON. It can't be called or constructed, and aside from its
//! two method properties, it has no interesting functionality of its own.
//!
//! More information:
//!  - [ECMAScript reference][spec]
//!  - [MDN documentation][mdn]
//!  - [JSON specification][json]
//!
//! [spec]: https://tc39.es/ecma262/#sec-json
//! [json]: https://www.json.org/json-en.html
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON

use crate::builtins::{
    function::make_builtin_fn,
    object::ObjectKind,
    value::{ResultValue, Value, ValueData},
};
use crate::exec::Interpreter;
use serde_json::{self, Value as JSONValue};
use std::ops::Deref;

#[cfg(test)]
mod tests;

/// `JSON.parse( text[, reviver] )`
///
/// This `JSON` method parses a JSON string, constructing the JavaScript value or object described by the string.
///
/// An optional `reviver` function can be provided to perform a transformation on the resulting object before it is returned.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#sec-json.parse
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse
// TODO: implement optional revever argument.
pub fn parse(_: &mut Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    match serde_json::from_str::<JSONValue>(
        &args
            .get(0)
            .expect("cannot get argument for JSON.parse")
            .clone()
            .to_string(),
    ) {
        Ok(json) => Ok(Value::from(json)),
        Err(err) => Err(Value::from(err.to_string())),
    }
}

/// `JSON.stringify( value[, replacer[, space]] )`
///
/// This `JSON` method converts a JavaScript object or value to a JSON string.
///
/// This medhod optionally replaces values if a `replacer` function is specified or
/// optionally including only the specified properties if a replacer array is specified.
///
/// An optional `space` argument can be supplied of type `String` or `Number` that's used to insert
/// white space into the output JSON string for readability purposes.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#sec-json.stringify
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/stringify
pub fn stringify(_: &mut Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    let object = args.get(0).expect("cannot get argument for JSON.stringify");
    let object_to_return = Value::new_object(None);
    if let Some(arg) = args.get(1) {
        if let ValueData::Object(ref obj) = arg.data() {
            let derefed_obj = (*obj).deref();
            let borrowed_derefed_obj = derefed_obj.borrow();
            if borrowed_derefed_obj.kind == ObjectKind::Array {
                for (key, value) in borrowed_derefed_obj.properties.iter() {
                    if let Some(Value(x)) = &value.value {
                        if key != "length" {
                            object_to_return.set_property(
                                x.to_string(),
                                object.get_property(&x.to_string()).unwrap(),
                            );
                        }
                    }
                }
                return Ok(Value::from(object_to_return.to_json().to_string()));
            } else {
                panic!("replacer only supports arrays at this time");
            }
        }
    }
    let json = object.to_json().to_string();
    Ok(Value::from(json))
}

/// Create a new `JSON` object.
pub fn create(global: &Value) -> Value {
    let json = Value::new_object(Some(global));

    make_builtin_fn(parse, "parse", &json, 2);
    make_builtin_fn(stringify, "stringify", &json, 3);

    json
}

/// Initialise the `JSON` object on the global object.
#[inline]
pub fn init(global: &Value) {
    global.set_field("JSON", create(global));
}
