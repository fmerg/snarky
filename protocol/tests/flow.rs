use backend::*;
use circuits::QAP;
use protocol::{SRS, Trapdoor, BatchProof, Phase, Verification};
use protocol;

use num_traits::identities::Zero;
use ark_ec::AffineCurve;

macro_rules! run_setup {
    ($qap: expr, $trapdoor:expr) => {
        {
            match $trapdoor {
                "given"  => SRS::setup(&$qap, Some(Trapdoor::from_u64(1, 2, 3, 4))),
                "random" => SRS::setup_with_random_trapdoor(&$qap),
                "unit"   => SRS::setup_with_unit_trapdoor(&$qap),
                _        => panic!("Trapdoor generation unspecified")
            }
        }
    }
}

macro_rules! run_updates {
    ($qap: expr, $srs: expr, $batch: expr, $phase: expr, $nr: expr) => {
        let mut count = 0;
        while count < $nr {
            protocol::update(&$qap, &mut $srs, &mut $batch, $phase);
            count += 1
        }
    }
}

macro_rules! corrupt {
    ($batch: expr, $comp: expr, $index: expr) => {
        match $comp {
            1 => {
                let val = &$batch.batch_1[$index][0].com.0;
                $batch.batch_1[$index][0].com.0 = add1!(val, genG1!());
            }
            2 => {
                let val = &$batch.batch_2[$index].com.0;
                $batch.batch_2[$index].com.0 = add1!(val, genG1!());
            }
            _ => panic!("Batch comp may be either 1 or 2")
        }

    }
}

macro_rules! run_protocol {
    (
        $m: expr, $n: expr, $l: expr, $trapdoor: expr, 
        $nr_1: expr => $cor_1: expr, $nr_2: expr => $cor_2: expr
    ) => {
        {
            let qap = QAP::create_default($m, $n, $l).unwrap();
            let (mut srs, trp) = run_setup!(qap, $trapdoor);

            let mut batch = BatchProof::initiate();
            run_updates!(qap, srs, batch, Phase::ONE, $nr_1);
            run_updates!(qap, srs, batch, Phase::TWO, $nr_2);

            // Make corruptions, if any
            for (j, cor) in [$cor_1, $cor_2].iter().enumerate() {
                let comp = j + 1;
                let length = if comp == 1 { $nr_1 } else { $nr_2 };
                match cor {
                    &"all" => {
                        for index in 0..length {
                            corrupt!(batch, comp, index);
                        }
                    },
                    &"one" => corrupt!(batch, comp, 0),
                    _ => {}
                }
            }

            protocol::verify(&qap, &srs, &batch)
        }
    }
}

#[test]
fn test_flow_with_given_trapdoor() {
    let res = run_protocol!(5, 4, 3, "given", 1 => "ok", 1 => "ok");
    assert_eq!(bool::from(res), true);
}

#[test]
fn test_flow_with_random_trapdoor() {
    let res = run_protocol!(5, 4, 3, "random", 1 => "ok", 1 => "ok");
    assert_eq!(bool::from(res), true);
}

#[test]
fn test_flow_with_unit_trapdoor() {
    let res = run_protocol!(5, 4, 3, "unit", 1 => "ok", 1 => "ok");
    assert_eq!(bool::from(res), true);
}

#[test]
fn test_flow_with_one_phase_1_proof_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 2 => "one", 1 => "ok");
    assert_eq!(bool::from(res), false);
}

#[test]
fn test_flow_with_all_phase_1_proofs_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 2 => "all", 1 => "ok");
    assert_eq!(bool::from(res), false);
}

#[test]
fn test_flow_with_one_phase_2_proof_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 1 => "ok", 2 => "one");
    assert_eq!(bool::from(res), false);
}

#[test]
fn test_flow_with_all_phase_2_proofs_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 1 => "ok", 2 => "all");
    assert_eq!(bool::from(res), false);
}

#[test]
fn test_flow_with_all_proofs_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 2 => "all", 2 => "all");
    assert_eq!(bool::from(res), false);
}
