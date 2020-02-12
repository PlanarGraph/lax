use lax::{Lazy, LazyFn, LazyValue};

fn lazy_sum<S, T>(a: S, b: T) -> i32
where
    S: Lazy<Output = i32>,
    T: Lazy<Output = i32>,
{
    a.eval() + b.eval()
}

#[test]
fn chain_operations() {
    let v = LazyValue::new(5 as i32)
        .op(|x| x + 2)
        .op(|x| x * 2)
        .op(|x| x.pow(2));

    assert_eq!(v.eval(), 196);
}

#[test]
fn sum_two_lazy_fns() {
    let x = LazyFn::new(|| 5);
    let y = LazyFn::new(|| 12).op(|x| x + 8);

    assert_eq!(lazy_sum(x, y), 25);
}

#[test]
fn map_int_to_string() {
    let is = LazyValue::new(819926 as i32).op(|x| format!("{}", x));
    assert_eq!("819926", is.eval())
}
