use symbolica::{atom::Atom, state::State};
use vakint::{
    EvaluationOrder, NumericalEvaluationResult, Vakint, VakintExpression, VakintSettings,
};

fn main() {
    // Set vakint parameters
    let vakint = Vakint::new(Some(VakintSettings {
        evaluation_order: EvaluationOrder::matad_only(None),
        run_time_decimal_precision: 32,
        ..VakintSettings::default()
    }))
    .unwrap();

    let mut integral = Atom::parse(
        "(
                  k(1,11)*k(2,11)*k(1,22)*k(2,22)
                + p(1,11)*k(3,11)*k(3,22)*p(2,22)
                + p(1,11)*p(2,11)*(k(2,22)+k(1,22))*k(2,22) 
             )
            *topo(\
                  prop(1,edge(1,2),k(1),muvsq,1)\
                * prop(2,edge(2,3),k(2),muvsq,1)\
                * prop(3,edge(3,1),k(3),muvsq,1)\
                * prop(4,edge(1,4),k(3)-k(1),muvsq,1)\
                * prop(5,edge(2,4),k(1)-k(2),muvsq,1)\
                * prop(6,edge(3,4),k(2)-k(3),muvsq,1)\
            )",
    )
    .unwrap();
    println!(
        "\nInput integral:\n{}\n",
        VakintExpression::try_from(integral.clone()).unwrap()
    );

    integral = vakint.evaluate(integral.as_view()).unwrap();
    println!("Evaluated integral:\n{}\n", integral.clone());

    // Set some value for the mass parameters
    let params = vakint.params_from_f64(
        &[("muvsq".into(), 2.0), ("mursq".into(), 3.0)]
            .iter()
            .cloned()
            .collect(),
    );

    // And for the external momenta part of the numerator
    let externals = vakint.externals_from_f64(
        &(1..=2)
            .map(|i| {
                (
                    i,
                    (
                        0.17 * ((i + 1) as f64),
                        0.4 * ((i + 2) as f64),
                        0.3 * ((i + 3) as f64),
                        0.12 * ((i + 4) as f64),
                    ),
                )
            })
            .collect(),
    );

    let (eval, error) = vakint
        .numerical_evaluation(integral.as_view(), &params, Some(&externals))
        .unwrap();
    println!("Numerical evaluation:\n{}\n", eval);
    let eval_atom = eval.to_atom(State::get_symbol(vakint.settings.epsilon_symbol.clone()));
    println!("Numerical evaluation as atom:\n{}\n", eval_atom);
    #[rustfmt::skip]
    let target_eval =  NumericalEvaluationResult::from_vec(
    vec![
            (-3, ( "7.48999999999990314303310867673e-2".into(),"0.0".into()),),
            (-2, ("-10.85566719804186267791124506061".into(),  "0.0".into()),),
            (-1, ("-27.70898883293408751146176885254".into(),  "0.0".into()),),
            ( 0, ( "87.47075938732425643179631959599".into(),  "0.0".into()),),
        ],
        &vakint.settings);
    let (matches, match_msg) = target_eval.does_approx_match(
        &eval,
        error.as_ref(),
        10.0_f64.powi(-((vakint.settings.run_time_decimal_precision - 4) as i32)),
        1.0,
    );
    if matches {
        println!("Numerical evaluation matches target result.");
    } else {
        println!(
            "Numerical evaluation does not match target result:\n{}",
            match_msg
        );
    }
}