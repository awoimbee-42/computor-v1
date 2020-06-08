use crate::TokenVec;

trait Operator {
    fn operate(TokenVec, usize) -> &'static str;
}

static OPERATORS: phf::Map<char, Operator> = phf_map! {
    '+' => Operator::new(1, Associativity::Any),
    '-' => Operator::new(1, Associativity::Left),
    '*' => Operator::new(2, Associativity::Any),
    '/' => Operator::new(2, Associativity::Left),
	'^' => Operator::new(3, Associativity::Right),
	'!' => Operator::new(3, Associativity::Left),
};
