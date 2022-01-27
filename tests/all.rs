#![feature(test)]
extern crate test;

#[cfg(test)]
mod formula_tests {
    use std::array::IntoIter;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    use rexp::eval::Evaluator;
    use rexp::parser::Parser;
    use rexp::value::Value;
    use test::Bencher;

    struct Case {
        pub formula: &'static str,
        pub context: HashMap<String, Value>,
        pub expect: Value,
    }
    #[test]
    fn eval_all() {
        let empty_context = HashMap::<String, Value>::from_iter(IntoIter::new([(
            "b0987654321".to_string(),
            Value::Empty,
        )]));

        let cases: [Case; 8] = [
            Case {
                formula: "-(1+(2+3)*4-5)",
                context: HashMap::default(),
                expect: Value::Number(-16.0),
            },
            Case {
                formula: "1",
                context: HashMap::default(),
                expect: Value::Number(1.0),
            },
            Case {
                formula: "\"50\"",
                context: HashMap::default(),
                expect: Value::String("50".to_string()),
            },
            Case {
                formula: "1<2",
                context: HashMap::default(),
                expect: Value::Boolean(true),
            },
            Case {
                formula: "SUM(1+1,1/1)",
                context: HashMap::default(),
                expect: Value::Number(3.0),
            },
            Case {
                formula: "REF(\"a1234567890.b0987654321\")",
                context: HashMap::<String, Value>::from_iter(IntoIter::new([(
                    "b0987654321".to_string(),
                    Value::Number(2.0),
                )])),
                expect: Value::Number(2.0),
            },
            Case {
                // string shoud be ignored
                formula: "AVERAGE(1,2,3,\"a\")",
                context: HashMap::default(),
                expect: Value::Number(2.0),
            },
            Case {
                formula:
                    "average(ref(\"a1234567890.b0987654321\"),ref(\"a1234567890.b0987654321\"))",
                context: empty_context,
                expect: Value::Empty,
            },
        ];
        for case in cases.iter() {
            let mut p = Parser::new(case.formula);
            p.tokenize();
            let e = p.parse();
            let evaluator = Evaluator {
                expr: e,
                context: &case.context,
            };
            let result = evaluator.resolve();
            assert_eq!(result, case.expect);
        }
    }
    // cargo test -- formula_tests   --bench
    // don't know why cargo bench won't run this
    #[bench]
    fn bench_eval(b: &mut Bencher) {
        let word = "-(1+(2+3)*4-5)";
        let mut p = Parser::new(word);
        b.iter(|| {
            p.tokenize();
        })
    }
}
