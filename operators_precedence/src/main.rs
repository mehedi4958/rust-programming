//---------------------------------------
//          - Operator Precedence
//---------------------------------------

/*
   Operator precedence (top to bottom)     Associativity (when multiple operators in a single expression)
   -----------------------------------     --------------------------------------------------------------
   *,/,%                                   Left to Right
   +,-                                     Left to Right
   <<,>>                                   Left to Right
   &                                       Left to Right
   ^                                       Left to Right
   |                                       Left to Right
   ==, !=, <, >, <=, >=                    Require Parentheses (non-associative)
   &&                                      Left to Right
   ||                                      Left to Right
   .., ..=                                 Require Parentheses (non-associative)
   =, +=, -=, *=, /=, %=, etc.             Left to Right
*/

fn main() {
    let _result = 5 + 2 * 3; // 5 + (2 * 3)
    let _expression = 16 / 4 * 2; // (6 / 4) * 2
    //
    let _x = (5 + 2) * 3;
    //
    let a = 3;
    let b = 4;
    let c = 5;
    let _result = a + b - c == 2 || a == 3; // 1.(((a + b) - c) == 2) || 2(a == 3)
}
