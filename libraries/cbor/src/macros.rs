// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_export]
macro_rules! cbor_map {
    // trailing comma case
    ( $( $key:expr => $value:expr, )+ ) => {
        cbor_map! ( $($key => $value),+ )
    };

    ( $( $key:expr => $value:expr ),* ) => {
        {
            // The import is unused if the list is empty.
            #[allow(unused_imports)]
            use $crate::values::{IntoCborKey, IntoCborValue};
            let mut _map = ::alloc::collections::BTreeMap::new();
            $(
                _map.insert($key.into_cbor_key(), $value.into_cbor_value());
            )*
            $crate::values::Value::Map(_map)
        }
    };
}

#[macro_export]
macro_rules! cbor_map_options {
    // trailing comma case
    ( $( $key:expr => $value:expr, )+ ) => {
        cbor_map_options! ( $($key => $value),+ )
    };

    ( $( $key:expr => $value:expr ),* ) => {
        cbor_extend_map_options! (
            ::alloc::collections::BTreeMap::<_, $crate::values::Value>::new(),
            $( $key => $value, )*
        )
    };
}

#[macro_export]
macro_rules! cbor_extend_map_options {
    // Add trailing comma if missing.
    ( $initial:expr, $( $key:expr => $value:expr ),+ ) => {
        cbor_extend_map_options! ( $initial, $($key => $value,)+ )
    };

    ( $initial:expr, $( $key:expr => $value:expr, )* ) => {
        {
            // The import is unused if the list is empty.
            #[allow(unused_imports)]
            use $crate::values::{IntoCborKey, IntoCborValueOption};
            let mut _map = $initial;
            $(
                if let Some(val) = $value.into_cbor_value_option() {
                    _map.insert($key.into_cbor_key(), val);
                }
            )*
            $crate::values::Value::Map(_map)
        }
    };
}

#[macro_export]
macro_rules! cbor_map_btree {
    ( $tree:expr ) => {
        $crate::values::Value::Map($tree)
    };
}

#[macro_export]
macro_rules! cbor_array {
    // trailing comma case
    ( $( $value:expr, )+ ) => {
        cbor_array! ( $($value),+ )
    };

    ( $( $value:expr ),* ) => {
        {
            // The import is unused if the list is empty.
            #[allow(unused_imports)]
            use $crate::values::IntoCborValue;
            $crate::values::Value::Array(vec![ $( $value.into_cbor_value(), )* ])
        }
    };
}

#[macro_export]
macro_rules! cbor_array_vec {
    ( $vec:expr ) => {{
        use $crate::values::IntoCborValue;
        $crate::values::Value::Array($vec.into_iter().map(|x| x.into_cbor_value()).collect())
    }};
}

#[cfg(test)]
macro_rules! cbor_true {
    ( ) => {
        $crate::values::Value::Simple($crate::values::SimpleValue::TrueValue)
    };
}

#[macro_export]
macro_rules! cbor_false {
    ( ) => {
        $crate::values::Value::Simple($crate::values::SimpleValue::FalseValue)
    };
}

#[macro_export]
macro_rules! cbor_null {
    ( ) => {
        $crate::values::Value::Simple($crate::values::SimpleValue::NullValue)
    };
}

#[cfg(test)]
macro_rules! cbor_undefined {
    ( ) => {
        $crate::values::Value::Simple($crate::values::SimpleValue::Undefined)
    };
}

#[macro_export]
macro_rules! cbor_bool {
    ( $x:expr ) => {
        $crate::values::Value::bool_value($x)
    };
}

// For key types, we construct a KeyType and call .into(), which will automatically convert it to a
// KeyType or a Value depending on the context.
#[macro_export]
macro_rules! cbor_unsigned {
    ( $x:expr ) => {
        cbor_key_unsigned!($x).into()
    };
}

#[macro_export]
macro_rules! cbor_int {
    ( $x:expr ) => {
        cbor_key_int!($x).into()
    };
}

#[macro_export]
macro_rules! cbor_text {
    ( $x:expr ) => {
        cbor_key_text!($x).into()
    };
}

#[macro_export]
macro_rules! cbor_bytes {
    ( $x:expr ) => {
        cbor_key_bytes!($x).into()
    };
}

// Macro to use with a literal, e.g. cbor_bytes_lit!(b"foo")
#[macro_export]
macro_rules! cbor_bytes_lit {
    ( $x:expr ) => {
        cbor_bytes!(($x as &[u8]).to_vec())
    };
}

// Some explicit macros are also available for contexts where the type is not explicit.
#[macro_export]
macro_rules! cbor_key_unsigned {
    ( $x:expr ) => {
        $crate::values::KeyType::Unsigned($x)
    };
}

#[macro_export]
macro_rules! cbor_key_int {
    ( $x:expr ) => {
        $crate::values::KeyType::integer($x)
    };
}

#[macro_export]
macro_rules! cbor_key_text {
    ( $x:expr ) => {
        $crate::values::KeyType::TextString($x.into())
    };
}

#[macro_export]
macro_rules! cbor_key_bytes {
    ( $x:expr ) => {
        $crate::values::KeyType::ByteString($x)
    };
}

#[cfg(test)]
mod test {
    use super::super::values::{KeyType, SimpleValue, Value};
    use alloc::collections::BTreeMap;

    #[test]
    fn test_cbor_simple_values() {
        assert_eq!(cbor_true!(), Value::Simple(SimpleValue::TrueValue));
        assert_eq!(cbor_false!(), Value::Simple(SimpleValue::FalseValue));
        assert_eq!(cbor_null!(), Value::Simple(SimpleValue::NullValue));
        assert_eq!(cbor_undefined!(), Value::Simple(SimpleValue::Undefined));
    }

    #[test]
    fn test_cbor_bool() {
        assert_eq!(cbor_bool!(true), Value::Simple(SimpleValue::TrueValue));
        assert_eq!(cbor_bool!(false), Value::Simple(SimpleValue::FalseValue));
    }

    #[test]
    fn test_cbor_int_unsigned() {
        assert_eq!(cbor_key_int!(0), KeyType::Unsigned(0));
        assert_eq!(cbor_key_int!(1), KeyType::Unsigned(1));
        assert_eq!(cbor_key_int!(123456), KeyType::Unsigned(123456));
        assert_eq!(
            cbor_key_int!(std::i64::MAX),
            KeyType::Unsigned(std::i64::MAX as u64)
        );
    }

    #[test]
    fn test_cbor_int_negative() {
        assert_eq!(cbor_key_int!(-1), KeyType::Negative(-1));
        assert_eq!(cbor_key_int!(-123456), KeyType::Negative(-123456));
        assert_eq!(
            cbor_key_int!(std::i64::MIN),
            KeyType::Negative(std::i64::MIN)
        );
    }

    #[test]
    fn test_cbor_int_literals() {
        let a = cbor_array![
            std::i64::MIN,
            std::i32::MIN,
            -123456,
            -1,
            0,
            1,
            123456,
            std::i32::MAX,
            std::i64::MAX,
            std::u64::MAX,
        ];
        let b = Value::Array(vec![
            Value::KeyValue(KeyType::Negative(std::i64::MIN)),
            Value::KeyValue(KeyType::Negative(std::i32::MIN as i64)),
            Value::KeyValue(KeyType::Negative(-123456)),
            Value::KeyValue(KeyType::Negative(-1)),
            Value::KeyValue(KeyType::Unsigned(0)),
            Value::KeyValue(KeyType::Unsigned(1)),
            Value::KeyValue(KeyType::Unsigned(123456)),
            Value::KeyValue(KeyType::Unsigned(std::i32::MAX as u64)),
            Value::KeyValue(KeyType::Unsigned(std::i64::MAX as u64)),
            Value::KeyValue(KeyType::Unsigned(std::u64::MAX)),
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_cbor_array() {
        let a = cbor_array![
            -123,
            456,
            true,
            cbor_null!(),
            "foo",
            b"bar",
            cbor_array![],
            cbor_array![0, 1],
            cbor_map! {},
            cbor_map! {2 => 3},
        ];
        let b = Value::Array(vec![
            Value::KeyValue(KeyType::Negative(-123)),
            Value::KeyValue(KeyType::Unsigned(456)),
            Value::Simple(SimpleValue::TrueValue),
            Value::Simple(SimpleValue::NullValue),
            Value::KeyValue(KeyType::TextString(String::from("foo"))),
            Value::KeyValue(KeyType::ByteString(b"bar".to_vec())),
            Value::Array(Vec::new()),
            Value::Array(vec![
                Value::KeyValue(KeyType::Unsigned(0)),
                Value::KeyValue(KeyType::Unsigned(1)),
            ]),
            Value::Map(BTreeMap::new()),
            Value::Map(
                [(KeyType::Unsigned(2), Value::KeyValue(KeyType::Unsigned(3)))]
                    .iter()
                    .cloned()
                    .collect(),
            ),
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_cbor_array_vec_empty() {
        let a = cbor_array_vec!(Vec::<bool>::new());
        let b = Value::Array(Vec::new());
        assert_eq!(a, b);
    }

    #[test]
    fn test_cbor_array_vec_int() {
        let a = cbor_array_vec!(vec![1, 2, 3, 4]);
        let b = Value::Array(vec![
            Value::KeyValue(KeyType::Unsigned(1)),
            Value::KeyValue(KeyType::Unsigned(2)),
            Value::KeyValue(KeyType::Unsigned(3)),
            Value::KeyValue(KeyType::Unsigned(4)),
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_cbor_array_vec_text() {
        let a = cbor_array_vec!(vec!["a", "b", "c"]);
        let b = Value::Array(vec![
            Value::KeyValue(KeyType::TextString(String::from("a"))),
            Value::KeyValue(KeyType::TextString(String::from("b"))),
            Value::KeyValue(KeyType::TextString(String::from("c"))),
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_cbor_array_vec_bytes() {
        let a = cbor_array_vec!(vec![b"a", b"b", b"c"]);
        let b = Value::Array(vec![
            Value::KeyValue(KeyType::ByteString(b"a".to_vec())),
            Value::KeyValue(KeyType::ByteString(b"b".to_vec())),
            Value::KeyValue(KeyType::ByteString(b"c".to_vec())),
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_cbor_map() {
        let a = cbor_map! {
            -1 => -23,
            4 => 56,
            "foo" => true,
            b"bar" => cbor_null!(),
            5 => "foo",
            6 => b"bar",
            7 => cbor_array![],
            8 => cbor_array![0, 1],
            9 => cbor_map!{},
            10 => cbor_map!{2 => 3},
        };
        let b = Value::Map(
            [
                (
                    KeyType::Negative(-1),
                    Value::KeyValue(KeyType::Negative(-23)),
                ),
                (KeyType::Unsigned(4), Value::KeyValue(KeyType::Unsigned(56))),
                (
                    KeyType::TextString(String::from("foo")),
                    Value::Simple(SimpleValue::TrueValue),
                ),
                (
                    KeyType::ByteString(b"bar".to_vec()),
                    Value::Simple(SimpleValue::NullValue),
                ),
                (
                    KeyType::Unsigned(5),
                    Value::KeyValue(KeyType::TextString(String::from("foo"))),
                ),
                (
                    KeyType::Unsigned(6),
                    Value::KeyValue(KeyType::ByteString(b"bar".to_vec())),
                ),
                (KeyType::Unsigned(7), Value::Array(Vec::new())),
                (
                    KeyType::Unsigned(8),
                    Value::Array(vec![
                        Value::KeyValue(KeyType::Unsigned(0)),
                        Value::KeyValue(KeyType::Unsigned(1)),
                    ]),
                ),
                (KeyType::Unsigned(9), Value::Map(BTreeMap::new())),
                (
                    KeyType::Unsigned(10),
                    Value::Map(
                        [(KeyType::Unsigned(2), Value::KeyValue(KeyType::Unsigned(3)))]
                            .iter()
                            .cloned()
                            .collect(),
                    ),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn test_cbor_map_options() {
        let a = cbor_map_options! {
            -1 => -23,
            4 => Some(56),
            11 => None::<String>,
            "foo" => true,
            12 => None::<&str>,
            b"bar" => Some(cbor_null!()),
            13 => None::<Vec<u8>>,
            5 => "foo",
            14 => None::<&[u8]>,
            6 => Some(b"bar" as &[u8]),
            15 => None::<bool>,
            7 => cbor_array![],
            16 => None::<i32>,
            8 => Some(cbor_array![0, 1]),
            17 => None::<i64>,
            9 => cbor_map!{},
            18 => None::<u64>,
            10 => Some(cbor_map!{2 => 3}),
        };
        let b = Value::Map(
            [
                (
                    KeyType::Negative(-1),
                    Value::KeyValue(KeyType::Negative(-23)),
                ),
                (KeyType::Unsigned(4), Value::KeyValue(KeyType::Unsigned(56))),
                (
                    KeyType::TextString(String::from("foo")),
                    Value::Simple(SimpleValue::TrueValue),
                ),
                (
                    KeyType::ByteString(b"bar".to_vec()),
                    Value::Simple(SimpleValue::NullValue),
                ),
                (
                    KeyType::Unsigned(5),
                    Value::KeyValue(KeyType::TextString(String::from("foo"))),
                ),
                (
                    KeyType::Unsigned(6),
                    Value::KeyValue(KeyType::ByteString(b"bar".to_vec())),
                ),
                (KeyType::Unsigned(7), Value::Array(Vec::new())),
                (
                    KeyType::Unsigned(8),
                    Value::Array(vec![
                        Value::KeyValue(KeyType::Unsigned(0)),
                        Value::KeyValue(KeyType::Unsigned(1)),
                    ]),
                ),
                (KeyType::Unsigned(9), Value::Map(BTreeMap::new())),
                (
                    KeyType::Unsigned(10),
                    Value::Map(
                        [(KeyType::Unsigned(2), Value::KeyValue(KeyType::Unsigned(3)))]
                            .iter()
                            .cloned()
                            .collect(),
                    ),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        );
        assert_eq!(a, b);
    }

    #[test]
    fn test_cbor_map_btree_empty() {
        let a = cbor_map_btree!(BTreeMap::new());
        let b = Value::Map(BTreeMap::new());
        assert_eq!(a, b);
    }

    #[test]
    fn test_cbor_map_btree_foo() {
        let a = cbor_map_btree!(
            [(KeyType::Unsigned(2), Value::KeyValue(KeyType::Unsigned(3)))]
                .iter()
                .cloned()
                .collect()
        );
        let b = Value::Map(
            [(KeyType::Unsigned(2), Value::KeyValue(KeyType::Unsigned(3)))]
                .iter()
                .cloned()
                .collect(),
        );
        assert_eq!(a, b);
    }
}
