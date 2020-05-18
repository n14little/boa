use crate::{exec::Interpreter, forward, realm::Realm};

#[test]
fn json_sanity() {
    let realm = Realm::create();
    let mut engine = Interpreter::new(realm);
    assert_eq!(
        forward(&mut engine, r#"JSON.parse('{"aaa":"bbb"}').aaa == 'bbb'"#),
        "true"
    );
    assert_eq!(
        forward(
            &mut engine,
            r#"JSON.stringify({aaa: 'bbb'}) == '{"aaa":"bbb"}'"#
        ),
        "true"
    );
}

#[test]
fn json_stringify_remove_undefined_values_from_objects() {
    let realm = Realm::create();
    let mut engine = Interpreter::new(realm);

    let actual = forward(
        &mut engine,
        r#"JSON.stringify({ aaa: undefined, bbb: 'ccc' })"#,
    );
    let expected = r#"{"bbb":"ccc"}"#;

    assert_eq!(actual, expected);
}

#[test]
fn json_stringify_remove_function_values_from_objects() {
    let realm = Realm::create();
    let mut engine = Interpreter::new(realm);

    let actual = forward(
        &mut engine,
        r#"JSON.stringify({ aaa: () => {}, bbb: 'ccc' })"#,
    );
    let expected = r#"{"bbb":"ccc"}"#;

    assert_eq!(actual, expected);
}

#[test]
fn json_stringify_remove_symbols_from_objects() {
    let realm = Realm::create();
    let mut engine = Executor::new(realm);

    let actual = forward(
        &mut engine,
        r#"JSON.stringify({ aaa: Symbol(), bbb: 'ccc' })"#,
    );
    let expected = r#"{"bbb":"ccc"}"#;

    assert_eq!(actual, expected);
}

#[test]
fn json_stringify_replacer_array_strings() {
    let realm = Realm::create();
    let mut engine = Interpreter::new(realm);
    let actual = forward(
        &mut engine,
        r#"JSON.stringify({aaa: 'bbb', bbb: 'ccc', ccc: 'ddd'}, ['aaa', 'bbb'])"#,
    );
    let expected = forward(&mut engine, r#"'{"aaa":"bbb","bbb":"ccc"}'"#);
    assert_eq!(actual, expected);
}

#[test]
fn json_stringify_replacer_array_numbers() {
    let realm = Realm::create();
    let mut engine = Interpreter::new(realm);
    let actual = forward(
        &mut engine,
        r#"JSON.stringify({ 0: 'aaa', 1: 'bbb', 2: 'ccc'}, [1, 2])"#,
    );
    let expected = forward(&mut engine, r#"'{"1":"bbb","2":"ccc"}'"#);
    assert_eq!(actual, expected);
}

#[test]
fn json_stringify_replacer_function() {
    let realm = Realm::create();
    let mut engine = Interpreter::new(realm);
    let actual = forward(
        &mut engine,
        r#"JSON.stringify({ aaa: 1, bbb: 2}, (key, value) => {
            if (key === 'aaa') {
                return undefined;
            }

            return value;
        })"#,
    );
    let expected = forward(&mut engine, r#"'{"bbb":2}'"#);
    assert_eq!(actual, expected);
}
