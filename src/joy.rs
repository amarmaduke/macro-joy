use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JoyValue {
    Num(i64),
    Core(fn(Vec<JoyValue>) -> Vec<JoyValue>),
    Quote(Vec<JoyValue>)
}

pub type JoyStack = Vec<JoyValue>;

impl From<i64> for JoyValue {
    fn from(x:i64) -> Self {
        JoyValue::Num(x)
    }
}

pub fn pop_slice<const N:usize>(stack : &mut JoyStack) -> Option<[JoyValue; N]> {
    let mut data = vec![];
    for _ in 0..N {
        if let Some(value) = stack.pop() {
            data.push(value);
        }
    }
    if data.len() == N {
        data.try_into().ok()
    } else {
        for v in data.drain(..) {
            stack.push(v);
        }
        None
    }
}

pub fn meta_apply(program : JoyStack, stack : JoyStack) -> JoyStack {
    let mut program = program;
    let mut result = stack;
    for v in program.drain(..) {
        result = match v {
            JoyValue::Core(f) => { f(result) },
            x => { 
                result.push(x);
                result
            }
        };
    }
    result
}

pub fn apply(stack : JoyStack) -> JoyStack {
    use JoyValue::*;
    let mut result = stack;
    if let Some(Quote(program)) = result.pop() {
        result = meta_apply(program, result);
    }
    result
}

pub fn pop(stack : JoyStack) -> JoyStack {
    let mut result = stack;
    result.pop();
    result
}

pub fn copy(stack : JoyStack) -> JoyStack {
    let mut result = stack;
    if let Some(value) = result.pop() {
        result.push(value.clone());
        result.push(value);
    }
    result
}

pub fn unquote(stack : JoyStack) -> JoyStack {
    let mut result = stack;
    if let Some(JoyValue::Quote(mut q)) = result.pop() {
        result.append(&mut q);
    }
    result
}

pub fn debug(stack : JoyStack) -> JoyStack {
    eprintln!("{:?}", stack);
    stack
}

pub fn id(stack : JoyStack) -> JoyStack {
    stack
}

pub fn cmp(stack : JoyStack) -> JoyStack {
    use JoyValue::*;
    let mut result = stack;
    if let Some([x, y]) = pop_slice(&mut result) {
        let v = x.cmp(&y) as i64;
        result.push(Num(v));
    }
    result
}

pub fn add(stack : JoyStack) -> JoyStack {
    use JoyValue::*;
    let mut result = stack;
    if let Some([Num(x), Num(y)]) = pop_slice(&mut result) {
        result.push(Num(x + y));
    }
    result
}

pub fn ite(stack : JoyStack) -> JoyStack {
    use JoyValue::*;
    let mut result = stack;
    if let Some([Num(cond), Quote(x), Quote(y)]) = pop_slice(&mut result) {
        if cond == 0 {
            result = meta_apply(x, result);
        } else {
            result = meta_apply(y, result);
        }
    }
    result
}

macro_rules! joy_value_expr {
    ($x:literal) => {
        $crate::joy::JoyValue::Num($x)
    };
    ($x:ident) => {
        $crate::joy::JoyValue::Core($x)
    };
    (($x:ident)) => {
        From::from($x)
    };
    (($x:expr)) => {
        From::from($x)
    };
    ([$($x:tt) +]) => {
        $crate::joy::JoyValue::Quote(vec![$(joy_value_expr!{ $x }),+])
    };
}

macro_rules! joy_eval {
    ($($x:tt) +) => {
        {
            #[allow(unused_imports)]
            use $crate::joy::*;
            let result = vec![];
            let program = vec![$(joy_value_expr!{ $x }),+];
            meta_apply(program, result)
        }
    };
}

macro_rules! joy_define {
    ($fun_name:ident, $($x:tt) +) => {
        pub fn $fun_name(stack : $crate::joy::JoyStack) -> $crate::joy::JoyStack {
            use $crate::joy::*;
            let result = stack;
            let program = vec![$(joy_value_expr!{ $x }),+];
            meta_apply(program, result)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::joy::*;

    fn assert_value(stack : JoyStack, value : JoyValue) {
        let mut stack = stack;
        assert!(stack.len() == 1);
        match stack.pop() {
            Some(x) => {
                assert_eq!(x, value)
            }
            None => unreachable!()
        }
    }

    #[test]
    fn cmp() {
        use JoyValue::*;
        assert_value(joy_eval!(2 2 cmp), Num(0));
        assert_value(joy_eval!((-3) (-3) cmp), Num(0));
        assert_value(joy_eval!(0 0 cmp), Num(0));
        assert_value(joy_eval!(2 4 cmp), Num(1));
        assert_value(joy_eval!((-5) 4 cmp), Num(1));
        assert_value(joy_eval!(7 2 cmp), Num(-1));
        assert_value(joy_eval!(7 (-11) cmp), Num(-1));
    }

    #[test]
    fn ite() {
        use JoyValue::*;
        assert_value(joy_eval!([3] [6] 0 ite), Num(6));
        assert_value(joy_eval!([3] [6] (-5) ite), Num(3));
        assert_value(joy_eval!([3] [6] 11 ite), Num(3));
    }
}