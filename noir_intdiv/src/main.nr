// Implement integer division in Noir!
//
// Public Inputs:
// x: Field element
// y: Field element
// 
// Public Output: quotient Field element from integer division
// 
// Note: don't change the function signature / types below. This is for exercise purposes!
fn main(x: pub Field, y: pub Field) -> pub Field {
    /* Casts are range checks! */
    (x as u32 / y as u32) as Field
}

#[test]
fn test_main() {
    // 12 / 5 = 2
    let res = main(12, 5);

    assert(res == 2);
}
