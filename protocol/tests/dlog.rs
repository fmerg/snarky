use backend::{scalar, genG1, genG2, smul1, smul2};
use protocol::dlog::{prove_dlog, verify_dlog};
use util::map;

#[test]
fn test_dlog_proof() {
    let parametrization = map! {
        (100, 100, 100) => true,
        (666, 100, 100) => false,
        (100, 666, 100) => false,
        (100, 100, 666) => false
    };
    for ((f1, f2, w), expected) in parametrization {
        let G = genG1!();
        let H = genG2!();
        let elem_1 = smul1!(genG1!(), scalar!(f1));
        let elem_2 = smul2!(genG2!(), scalar!(f2));
        let phi = (elem_1, elem_2);
        let witness = scalar!(w);
        let proof = prove_dlog(phi, witness);
        let verified = verify_dlog(&G, &H, phi, proof);
        assert_eq!(verified, expected);
    }
}
