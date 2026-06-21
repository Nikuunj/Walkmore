mod utils;

use utils::*;

#[test]
fn test_initialize() {
    let (mut svm, payer) = setup();

    let init_ix = initialize();
    send(&mut svm, &payer, &[init_ix]);
}
