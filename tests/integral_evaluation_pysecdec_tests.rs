mod test_utils;
use vakint::{EvaluationMethod, EvaluationOrder, PySecDecOptions, VakintSettings};

use std::vec;

use symbolica::atom::Atom;
use test_utils::convert_test_params;

use crate::test_utils::{compare_vakint_evaluation_vs_reference, convert_test_externals};

const N_DIGITS_PYSECDEC_EVALUATION_FOR_TESTS: u32 = 10;
// PySecDec QMC is often very optimistic
const MAX_PULL: f64 = 1.0e5;

#[test_log::test]
fn test_integrate_2l_different_masses() {
    #[rustfmt::skip]
    compare_vakint_evaluation_vs_reference(
        VakintSettings::default(),
        EvaluationOrder(vec![EvaluationMethod::PySecDec(PySecDecOptions::default())]),
        Atom::parse(
            "(1)*topo(\
                prop(1,edge(1,2),k(1),muvsqA,1)\
                *prop(2,edge(1,2),k(2),muvsqB,1)\
                *prop(3,edge(2,1),k(1)+k(2),muvsqC,1)\
            )",
        )
        .unwrap()
        .as_view(),
        // Masses chosen equal on purpose here so as to have a reliable target analytical result
        convert_test_params(&[("muvsqA".into(), 1.0), ("muvsqB".into(), 1.0), ("muvsqC".into(), 1.0), ("mursq".into(), 1.0)].iter().cloned().collect(),
            N_DIGITS_PYSECDEC_EVALUATION_FOR_TESTS),
        convert_test_externals(
        &(1..=1)
            .map(|i| (i, (17.0*((i+1) as f64), 4.0*((i+2) as f64), 3.0*((i+3) as f64), 12.0*((i+4) as f64))))
            .collect(),
            N_DIGITS_PYSECDEC_EVALUATION_FOR_TESTS),
        vec![
            (-2, ("-146.1136365510036558546604990331".into(), "0.0".into()),),
            (-1, ("635.8146971740286947808753047759".into(),  "0.0".into()),),
            (0,  ("-1646.531034471454483109201793220".into(), "0.0".into()),),
            (1,  ("2240.516116133454318298096346441".into(),  "0.0".into()),),
        ],
        N_DIGITS_PYSECDEC_EVALUATION_FOR_TESTS, MAX_PULL
    );
}

#[test_log::test]
fn test_integrate_3l_o_eps() {
    #[rustfmt::skip]
    compare_vakint_evaluation_vs_reference(
        VakintSettings { number_of_terms_in_epsilon_expansion: 4, ..VakintSettings::default()},
        EvaluationOrder(vec![EvaluationMethod::PySecDec(PySecDecOptions::default())]),
        Atom::parse(
            "(1)*topo(\
                 prop(1,edge(1,2),k(1),muvsq,1)\
                *prop(2,edge(2,3),k(2),muvsq,1)\
                *prop(3,edge(3,1),k(3),muvsq,1)\
                *prop(4,edge(1,4),k(3)-k(1),muvsq,1)\
                *prop(5,edge(2,4),k(1)-k(2),muvsq,1)\
                *prop(6,edge(3,4),k(2)-k(3),muvsq,1)\
            )",
        )
        .unwrap()
        .as_view(),
        // Masses chosen equal on purpose here so as to have a reliable target analytical result
        convert_test_params(&[("muvsq".into(), 1.0), ("mursq".into(), 1.0)].iter().cloned().collect(),
            N_DIGITS_PYSECDEC_EVALUATION_FOR_TESTS),
        convert_test_externals(
        &(1..=1)
            .map(|i| (i, (17.0*((i+1) as f64), 4.0*((i+2) as f64), 3.0*((i+3) as f64), 12.0*((i+4) as f64))))
            .collect(),
            N_DIGITS_PYSECDEC_EVALUATION_FOR_TESTS),
            vec![
                // These first two entries are obtained from the analytic expression
                (-1, ("0.0".into(), "-2311.289033520460340396770711738".into()),),
                ( 0, ("0.0".into(), "35134.99893627257345553503414002".into()),),
                // This last entry does not have an analytical expression (MATAD not implemented yet)
                ( 1, ("0.0".into(), "-287175.6919264755".into()),),
            ],
        N_DIGITS_PYSECDEC_EVALUATION_FOR_TESTS, MAX_PULL
    );
}

#[test_log::test]
fn test_integrate_4l_h() {
    let vakint_default_settings = VakintSettings {
        number_of_terms_in_epsilon_expansion: 4,
        ..VakintSettings::default()
    };
    #[rustfmt::skip]
    compare_vakint_evaluation_vs_reference(
        vakint_default_settings,
        EvaluationOrder(vec![EvaluationMethod::PySecDec(
            PySecDecOptions{ min_n_evals: 100_000, max_n_evals: 1_000_000, ..PySecDecOptions::default()} )]),
        Atom::parse(
            "(1)*topo(\
                 prop(2,edge(5,1),k(1),muvsq,1)\
                *prop(3,edge(2,6),k(2),muvsq,1)\
                *prop(4,edge(6,5),k(3),muvsq,1)\
                *prop(5,edge(4,3),k(4),muvsq,1)\
                *prop(6,edge(3,5),k(1)-k(3),muvsq,1)\
                *prop(7,edge(6,4),k(2)-k(3),muvsq,1)\
                *prop(8,edge(3,2),k(3)-k(1)+k(4),muvsq,1)\
                *prop(9,edge(1,4),k(3)-k(2)+k(4),muvsq,1)\
                *prop(1,edge(2,1),k(3)-k(1)-k(2)+k(4),muvsq,1)\
            )",
        )
        .unwrap()
        .as_view(),
        // Masses chosen equal on purpose here so as to have a reliable target analytical result
        convert_test_params(&[("muvsq".into(), 1.0), ("mursq".into(), 1.0)].iter().cloned().collect(),
            4),
        convert_test_externals(
        &(1..=1)
            .map(|i| (i, (17.0*((i+1) as f64), 4.0*((i+2) as f64), 3.0*((i+3) as f64), 12.0*((i+4) as f64))))
            .collect(),
            4),
        vec![
            // This does not have an analytical expression yet (FMFT not implemented yet)
            (0,  ("-12799.61".into(), "0.0".into()),),
        ],
        4, MAX_PULL
    );
}
//git commit -m "Add three- and four-loop topologies to the collection of hardcoded topologies and add pysecdec tests for them."