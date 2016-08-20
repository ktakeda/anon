#[macro_use(anon)]
extern crate anon;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let x = anon! { a: 1, b:2 };
        assert_eq!(1, x.a);
        assert_eq!(2, x.b);
    }
    #[test]
    fn iter_test() {
        let y: Vec<_> = vec![1,2].into_iter().map(|x| {
            anon! { a: x*x }
        }).collect();
        assert_eq!(1, y[0].a);
        assert_eq!(4, y[1].a);
    }
    #[test]
    fn array_test() {
        let z = [anon! {a: 1, b: 2}, anon! {a: 1, b: 2}];
        assert_eq!(1, z[0].a);
        assert_eq!(4, z[1].a);
    }
}
