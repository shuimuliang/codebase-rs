#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
    fn func1(&self) -> u32;
    fn func2(&self, x: u32, y: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_foo() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);
        assert_eq!(5, mock.foo(4));
    }

    #[test]
    fn test_mock_funcs() {
        let mut mock = MockMyTrait::new();
        mock.expect_func1().return_const(42u32);
        mock.expect_func2().returning(|x, y| x + y);

        assert_eq!(42u32, mock.func1());
        assert_eq!(10, mock.func2(4, 6));
    }
}
