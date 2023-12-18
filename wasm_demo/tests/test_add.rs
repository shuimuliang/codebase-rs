#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    fn add_function(a: i32, b: i32) -> i32 {
        a + b
    }

    // 标记测试函数
    #[wasm_bindgen_test]
    fn test_addition() {
        // 调用您的 Rust 函数
        let result = add_function(2, 3);

        // 使用断言宏来验证函数的行为
        assert_eq!(result, 5);
    }
}
