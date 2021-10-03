use std::num::ParseIntError;
use std::rc::Rc;
use crate::variable_scope::VariableScope;
use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;

struct Condition<'a> {
    value: Vec<&'a str>,
    processed: bool
}

struct Regexes {
    int_re: Regex,
    float_re: Regex,
    single_str: Regex,
    double_str: Regex,
}

impl Regexes {
    pub(crate) fn new() -> Regexes {
        Regexes {
            int_re: Regex::new(r"^\d+$").unwrap(),
            float_re: Regex::new(r"^\d+[\.,]\d+$").unwrap(),
            single_str: Regex::new(r"^'.+'$").unwrap(),
            double_str: Regex::new(r#"^".+"$"#).unwrap(),
        }
    }
}

struct SimpleCondition {
    negated: bool,
    lhs: String,
    rhs: String,
    operator: String
}

impl SimpleCondition {
    pub(crate) fn new_from_lhs(lhs: String) -> SimpleCondition {
        SimpleCondition {
            lhs,
            rhs: String::new(),
            operator: String::new(),
            negated: false
        }
    }

    pub(crate) fn new_from_negation(negated: bool) -> SimpleCondition {
        SimpleCondition {
            lhs: String::new(),
            rhs: String::new(),
            operator: String::new(),
            negated
        }
    }

    pub(crate) fn evaluate(self, variable_scope: Rc<VariableScope>) -> bool {
        let regs = Regexes::new();

        if self.lhs.as_str() == "true" {
            self.evaluate_after_boolean(true, variable_scope)
        } else if self.lhs.as_str() == "false" {
            self.evaluate_after_boolean(false, variable_scope)
        } else if regs.int_re.is_match(self.lhs.as_str()) {
            self.evaluate_after_int(self.lhs.parse::<i64>().unwrap(), variable_scope)
        } else if regs.float_re.is_match(self.lhs.as_str()) {
            self.evaluate_after_float(self.lhs.parse::<f64>().unwrap(), variable_scope)
        } else if regs.single_str.is_match(self.lhs.as_str()) {
            let substring: &str = &s.to_string()[1..s.len() - 1];
            self.evaluate_after_string(String::from(substring), variable_scope)
        } else if regs.double_str.is_match(self.lhs.as_str()) {
            let substring: &str = &s.to_string()[1..s.len() - 1];
            self.evaluate_after_string(String::from(substring), variable_scope)
        } else {
            self.evaluate_lhs_variable(variable_scope)
        }
    }

    fn evaluate_after_int(self, resolved_lhs: i64, variable_scope: Rc<VariableScope>) -> bool {
        false
    }

    fn evaluate_after_float(self, resolved_lhs: f64, variable_scope: Rc<VariableScope>) -> bool {
        false
    }

    fn evaluate_after_string(self, resolved_lhs: String, variable_scope: Rc<VariableScope>) -> bool {
        false
    }

    fn evaluate_after_boolean(self, resolved_lhs: bool, variable_scope: Rc<VariableScope>) -> bool {
        false
    }

    fn evaluate_lhs_variable(self, variable_scope: Rc<VariableScope>) -> bool {
        false
    }

    fn evaluate_int_int(resolved_lhs: i64, resolved_rhs: i64) -> bool {
        false
    }

    fn evaluate_int_float(resolved_lhs: i64, resolved_rhs: f64) -> bool {
        false
    }

    fn evaluate_int_string(resolved_lhs: i64, resolved_rhs: String) -> bool {
        false
    }

    fn evaluate_int_bool(resolved_lhs: i64, resolved_rhs: bool) -> bool {
        false
    }

    fn evaluate_float_int(resolved_lhs: f64, resolved_rhs: i64) -> bool {
        false
    }

    fn evaluate_float_float(resolved_lhs: f64, resolved_rhs: f64) -> bool {
        false
    }

    fn evaluate_float_string(resolved_lhs: f64, resolved_rhs: String) -> bool {
        false
    }

    fn evaluate_float_bool(resolved_lhs: f64, resolved_rhs: bool) -> bool {
        false
    }

    fn evaluate_string_int(resolved_lhs: String, resolved_rhs: i64) -> bool {
        false
    }

    fn evaluate_string_float(resolved_lhs: String, resolved_rhs: f64) -> bool {
        false
    }

    fn evaluate_string_string(resolved_lhs: String, resolved_rhs: String) -> bool {
        false
    }

    fn evaluate_string_bool(resolved_lhs: String, resolved_rhs: bool) -> bool {
        false
    }

    fn evaluate_bool_int(resolved_lhs: bool, resolved_rhs: i64) -> bool {
        false
    }

    fn evaluate_bool_float(resolved_lhs: bool, resolved_rhs: f64) -> bool {
        false
    }

    fn evaluate_bool_string(resolved_lhs: bool, resolved_rhs: String) -> bool {
        false
    }

    fn evaluate_bool_bool(resolved_lhs: bool, resolved_rhs: bool) -> bool {
        false
    }
}

fn process_condition(condition: String, variable_scope: Rc<VariableScope>) -> bool {
    if condition.to_lowercase() == "true" {
        return true;
    } else if condition.to_lowercase() == "false" {
        return false;
    }

    let mut condition = Condition {
        value: condition.graphemes(true).collect::<Vec<&str>>(),
        processed: false
    };

    let mut parenthesis_depth: u16 = 0;
    let mut inside_quotes: bool = false;
    let mut temp_start: usize = usize::MIN;
    for i in 0..condition.value.len() {
        match condition.value[i] {
            "\"" => {
                if condition.value[i - 1] != "\\" {
                    inside_quotes = !inside_quotes
                }
            },
            "\'" => {
                if condition.value[i - 1] != "\\" {
                    inside_quotes = !inside_quotes
                }
            },
            "(" => {
                if !inside_quotes {
                    parenthesis_depth += 1;
                }
            },
            ")" => {
                if !inside_quotes {
                    parenthesis_depth -= 1;
                }
            },
            " " => {

            },
            _ => {

            }
        }
    }

    condition.processed = true;
    false
}

fn solve_condition(condition: Condition, variable_scope: Rc<VariableScope>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use crate::condition_processing::process_condition;
    use crate::variable_scope::VariableScope;
    use std::rc::Rc;

    #[test]
    fn condition_is_true() {
        let var_scope = VariableScope::create();
        let res = process_condition("true".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn condition_is_false() {
        let var_scope = VariableScope::create();
        let res = process_condition("false".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    // simple conditions
    #[test]
    fn bool_variable() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"true".to_string());
        let res = process_condition("myVar".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn variable_compare_lte() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar lte 1".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn variable_compare_not_lte() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar lte 0".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn variable_compare_lt() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar lt 2".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn variable_compare_not_lt() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar lt 1".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn variable_compare_gte() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar gte 1".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn variable_compare_not_gte() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar gte 2".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn variable_compare_gt() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar gt 2".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn variable_compare_not_gt() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar gt 1".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn variable_compare_eq() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar eq 1".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn variable_compare_not_eq() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar eq 2".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn variable_compare_neq() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar neq 2".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn variable_compare_not_neq() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"1".to_string());
        let res = process_condition("myVar neq 1".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    // compound tests
    // ands
    #[test]
    fn simple_and_without_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 and myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_and_with_partial_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) and myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_and_with_partial_parenthesis_case_two() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 and (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_and_with_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) and (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_and_without_parenthesis_with_first_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("not myVar neq 1 and myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_with_partial_parenthesis_with_first_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("not (myVar neq 1) and myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_without_parenthesis_with_second_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 and not myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_with_partial_parenthesis_with_second_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) and not myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_with_partial_parenthesis_with_second_not_type_two() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 and not (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_with_parenthesis_with_second_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) and not (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_without_parenthesis_double_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("not myVar neq 1 and not myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_with_parenthesis_after_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("not (myVar neq 1 and myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_with_parenthesis_first_not_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(not (myVar neq 1)) and myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_with_parenthesis_with_second_not_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) and (not (myVar neq 3))".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_and_with_parenthesis_with_double_not_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(not (myVar neq 1)) and (not (myVar neq 3))".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    // ors
    #[test]
    fn simple_or_and_without_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 or myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_partial_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) or myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_partial_parenthesis_case_two() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 or (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) or (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_without_parenthesis_with_first_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("not myVar neq 1 or myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_partial_parenthesis_with_first_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("not (myVar neq 1) or myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_and_with_parenthesis_with_first_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("not (myVar neq 1) or (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_without_parenthesis_with_second_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 or not myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_partial_parenthesis_with_second_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) or not myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_partial_parenthesis_with_second_not_type_two() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 or not (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_parenthesis_with_second_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) or not (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_without_parenthesis_double_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("not myVar neq 1 or not myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_or_with_parenthesis_after_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("not (myVar neq 1 or myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    #[test]
    fn simple_or_with_partial_parenthesis_first_not_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(not (myVar neq 1)) or myVar neq 3".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_parenthesis_first_not() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(not (myVar neq 1)) or (myVar neq 3)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_parenthesis_with_second_not_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1) or (not (myVar neq 3))".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn simple_or_with_parenthesis_with_double_not_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(not (myVar neq 1)) or (not (myVar neq 3))".to_string(), Rc::new(var_scope));
        assert_eq!(res, false);
    }

    // mix
    #[test]
    fn and_or_without_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 and myVar neq 2 or myVar neq 0".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn and_with_parenthesis_or() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1 and myVar neq 2) or myVar neq 0".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn and_or_with_partial_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("myVar neq 1 and (myVar neq 2 or myVar neq 0)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn and_or_with_parenthesis() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1 or myVar neq 2) and (myVar neq 0 or myVar neq 2)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }

    #[test]
    fn and_or_with_parenthesis_with_parenthesis_nots() {
        let mut var_scope = VariableScope::create();
        var_scope.create_variable(&"myVar".to_string(), &"2".to_string());
        let res = process_condition("(myVar neq 1 or not myVar neq 2) and (myVar neq 0 or not myVar neq 2)".to_string(), Rc::new(var_scope));
        assert_eq!(res, true);
    }
}