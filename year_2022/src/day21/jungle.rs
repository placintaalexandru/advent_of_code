use crate::day21::expression::Expression;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub(crate) struct Monkey {
    job: Expression,
}

impl Monkey {
    pub(crate) fn new(job: Expression) -> Self {
        Self { job }
    }
}

#[derive(Debug)]
pub(crate) struct Jungle {
    monkeys: HashMap<String, Monkey>,
}

impl Jungle {
    pub(crate) fn parse(path: &str) -> Self {
        let mut monkeys = HashMap::new();

        BufReader::new(File::open(path).unwrap())
            .lines()
            .for_each(|line| {
                let content = line.as_ref().unwrap();
                if let Ok((source_monkey, monkey_dep1, op, monkey_dep2)) =
                    scan_fmt!(content, r"{}: {} {} {}", String, String, String, String)
                {
                    match op.as_str() {
                        "+" => monkeys.insert(
                            source_monkey,
                            Monkey::new(Expression::Add(monkey_dep1, monkey_dep2)),
                        ),
                        "-" => monkeys.insert(
                            source_monkey,
                            Monkey::new(Expression::Sub(monkey_dep1, monkey_dep2)),
                        ),
                        "/" => monkeys.insert(
                            source_monkey,
                            Monkey::new(Expression::Div(monkey_dep1, monkey_dep2)),
                        ),
                        "*" => monkeys.insert(
                            source_monkey,
                            Monkey::new(Expression::Mul(monkey_dep1, monkey_dep2)),
                        ),
                        _ => panic!("Unknown {}", content),
                    };
                }

                if let Ok((source_monkey, value)) = scan_fmt!(content, "{}: {d}", String, i64) {
                    monkeys.insert(source_monkey, Monkey::new(Expression::Value(value)));
                }
            });

        Self { monkeys }
    }

    pub(crate) fn eval(&self, monkey_name: &str) -> i64 {
        match &self.monkeys[monkey_name].job {
            Expression::Add(m1, m2) => self.eval(&m1) + self.eval(&m2),
            Expression::Sub(m1, m2) => self.eval(&m1) - self.eval(&m2),
            Expression::Mul(m1, m2) => self.eval(&m1) * self.eval(&m2),
            Expression::Div(m1, m2) => self.eval(&m1) / self.eval(&m2),
            Expression::Value(m) => *m,
        }
    }

    fn eval_evaluable_expressions(
        &self,
        monkey_name: &str,
        results: &mut HashMap<String, i64>,
    ) -> Option<i64> {
        if monkey_name == "humn" {
            return None;
        }

        match &self.monkeys[monkey_name].job {
            Expression::Add(m1, m2) => {
                let r1 = self.eval_evaluable_expressions(m1, results);
                let r2 = self.eval_evaluable_expressions(m2, results);

                if r1.is_none() || r2.is_none() {
                    return None;
                }

                results.insert(monkey_name.to_owned(), r1.unwrap() + r2.unwrap());
            }
            Expression::Sub(m1, m2) => {
                let r1 = self.eval_evaluable_expressions(m1, results);
                let r2 = self.eval_evaluable_expressions(m2, results);

                if r1.is_none() || r2.is_none() {
                    return None;
                }

                results.insert(monkey_name.to_owned(), r1.unwrap() - r2.unwrap());
            }
            Expression::Mul(m1, m2) => {
                let r1 = self.eval_evaluable_expressions(m1, results);
                let r2 = self.eval_evaluable_expressions(m2, results);

                if r1.is_none() || r2.is_none() {
                    return None;
                }

                results.insert(monkey_name.to_owned(), r1.unwrap() * r2.unwrap());
            }
            Expression::Div(m1, m2) => {
                let r1 = self.eval_evaluable_expressions(m1, results);
                let r2 = self.eval_evaluable_expressions(m2, results);

                if r1.is_none() || r2.is_none() {
                    return None;
                }

                results.insert(monkey_name.to_owned(), r1.unwrap() / r2.unwrap());
            }
            Expression::Value(v) => {
                results.insert(monkey_name.to_owned(), *v);
            }
        }

        Some(results[monkey_name])
    }

    fn solve_equation(
        &self,
        monkey_name: &str,
        results: &HashMap<String, i64>,
        desired_result: i64,
    ) -> i64 {
        match &self.monkeys[monkey_name].job {
            Expression::Add(m1, m2) => {
                if results.contains_key(m1) && !results.contains_key(m2) {
                    return self.solve_equation(m2, results, desired_result - results[m1]);
                }

                if !results.contains_key(m1) && results.contains_key(m2) {
                    return self.solve_equation(m1, results, desired_result - results[m2]);
                }

                unreachable!()
            }
            Expression::Sub(m1, m2) => {
                if results.contains_key(m1) && !results.contains_key(m2) {
                    return self.solve_equation(m2, results, results[m1] - desired_result);
                }

                if !results.contains_key(m1) && results.contains_key(m2) {
                    return self.solve_equation(m1, results, desired_result + results[m2]);
                }

                unreachable!()
            }
            Expression::Mul(m1, m2) => {
                if results.contains_key(m1) && !results.contains_key(m2) {
                    return self.solve_equation(m2, results, desired_result / results[m1]);
                }

                if !results.contains_key(m1) && results.contains_key(m2) {
                    return self.solve_equation(m1, results, desired_result / results[m2]);
                }

                unreachable!()
            }
            Expression::Div(m1, m2) => {
                if results.contains_key(m1) && !results.contains_key(m2) {
                    return self.solve_equation(m2, results, results[m1] / desired_result);
                }

                if !results.contains_key(m1) && results.contains_key(m2) {
                    return self.solve_equation(m1, results, desired_result * results[m2]);
                }

                unreachable!()
            }
            _ => desired_result,
        }
    }

    pub(crate) fn solve(&self, monkey_name: &str) -> i64 {
        let mut evaluable_expressions = HashMap::new();
        self.eval_evaluable_expressions(monkey_name, &mut evaluable_expressions);

        if let Some([m1, m2]) = self.monkeys[monkey_name].job.dependencies() {
            if evaluable_expressions.contains_key(m1) && !evaluable_expressions.contains_key(m2) {
                return self.solve_equation(m2, &evaluable_expressions, evaluable_expressions[m1]);
            }

            if !evaluable_expressions.contains_key(m1) && evaluable_expressions.contains_key(m2) {
                return self.solve_equation(m1, &evaluable_expressions, evaluable_expressions[m2]);
            }
        }

        unreachable!()
    }
}
