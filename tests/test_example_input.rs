use day_3::*;

#[test]
#[ignore]
fn test_parse_and_multiplication() {
    let table: Vec<(String, i32, i32)> = parse_operation(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        r"\w\w\w"
    );

    let total = process(table);

    assert_eq!(total, 161);
}