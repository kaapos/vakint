mod test_utils;
use log::debug;
use symbolica::atom::Atom;
use test_utils::{compare_output, get_vakint};
use vakint::{VakintError, VakintSettings};

#[test_log::test]
fn test_1l_matching() {
    let vakint = get_vakint(VakintSettings {
        allow_unknown_integrals: false,
        ..VakintSettings::default()
    });

    debug!("Topologies:\n{}", vakint.topologies);

    compare_output(
        vakint
            .to_canonical(
                Atom::parse(
                    "(k(8,2)*k(8,2)+k(8,33)*p(12,33))*topo(\
                    prop(77,edge(42,42),k(8),muvsq,1)\
                )",
                )
                .unwrap()
                .as_view(),
                false,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse(
            "(k(1,2)^2+k(1,33)*p(12,33))*topo(\
                prop(1,edge(1,1),k(1),muvsq,1)\
            )",
        )
        .unwrap(),
    );

    compare_output(
        vakint
            .to_canonical(
                Atom::parse(
                    "(k(8,2)*k(8,2)+k(8,33)*p(12,33))*topo(\
                    prop(77,edge(42,42),k(8),muvsq,1)\
                )",
                )
                .unwrap()
                .as_view(),
                true,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse("(k(1,2)^2+k(1,33)*p(12,33))*topo(I1L(muvsq,1))").unwrap(),
    );

    compare_output(
        vakint
            .to_canonical(
                Atom::parse("(k(1,2)^2+k(1,33)*p(12,33))*topo(I1L(muvsq,-3))")
                    .unwrap()
                    .as_view(),
                false,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse("(k(1,2)^2+k(1,33)*p(12,33))*topo(prop(1,edge(1,1),k(1),muvsq,-3))").unwrap(),
    );
}

#[test]
fn test_2l_matching_3prop() {
    let vakint = get_vakint(VakintSettings {
        allow_unknown_integrals: false,
        ..VakintSettings::default()
    });

    //println!("Topologies:\n{}", vakint.topologies);

    compare_output(
        vakint
            .to_canonical(
                Atom::parse(
                    "(k(11,2)*k(11,2)+k(11,77)*k(22,77)+k(22,33)*p(42,33))*topo(\
                            prop(9,edge(7,10),k(11),mUVsq,1)*\
                            prop(33,edge(7,10),k(22),mUVsq,2)*\
                            prop(55,edge(7,10),k(11)+k(22),mUVsq,1)\
                        )",
                )
                .unwrap()
                .as_view(),
                false,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse(
            "(k(1,2)*k(1,2)+k(1,77)*k(2,77)+k(2,33)*p(42,33))*topo(\
                        prop(1,edge(1,2),k(1),mUVsq,1)*\
                        prop(2,edge(1,2),k(2),mUVsq,2)*\
                        prop(3,edge(2,1),k(1)+k(2),mUVsq,1)\
                    )",
        )
        .unwrap(),
    );

    compare_output(
        vakint
            .to_canonical(
                Atom::parse(
                    "(k(11,2)*k(11,2)+k(11,77)*k(22,77)+k(22,33)*p(42,33))*topo(\
                            prop(9,edge(7,10),k(11),mUVsq,1)*\
                            prop(33,edge(7,10),k(22),mUVsq,2)*\
                            prop(55,edge(7,10),k(11)+k(22),mUVsq,1)\
                        )",
                )
                .unwrap()
                .as_view(),
                true,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse("(k(1,2)*k(1,2)+k(1,77)*k(2,77)+k(2,33)*p(42,33))*topo(I2L(mUVsq,1,2,1))")
            .unwrap(),
    );

    compare_output(
        vakint.to_canonical(
            Atom::parse("(k(1,2)*k(1,2)+k(1,77)*k(2,77)+k(2,33)*p(42,33))*topo(I2L(mUVsq,1,2,1))")
                .unwrap()
                .as_view(),
            false,
        ).as_ref().map(|a| a.as_view()),
        Atom::parse("(k(1,2)*k(1,2)+k(1,77)*k(2,77)+k(2,33)*p(42,33))*topo(\
                                                prop(1,edge(1,2),k(1),mUVsq,1)*prop(2,edge(1,2),k(2),mUVsq,2)*\
                                                prop(3,edge(2,1),k(1)+k(2),mUVsq,1)\
                                            )").unwrap(),
    );
}

#[test_log::test]
fn test_2l_matching_pinched() {
    let vakint = get_vakint(VakintSettings {
        allow_unknown_integrals: false,
        ..VakintSettings::default()
    });

    compare_output(
        vakint
            .to_canonical(
                Atom::parse(
                    "(k(11,2)*k(11,2)+k(11,77)*k(22,77)+k(22,33)*p(42,33))*topo(\
                            prop(33,edge(10,10),k(22),mUVsq,2)*\
                            prop(55,edge(10,10),k(11),mUVsq,1)\
                        )",
                )
                .unwrap()
                .as_view(),
                false,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse(
            "(k(2,2)^2+k(1,33)*p(42,33)+k(1,77)*k(2,77))*topo(\
                        prop(1,edge(1,1),k(1),mUVsq,2)*\
                        prop(2,edge(1,1),k(2),mUVsq,1)
                    )",
        )
        .unwrap(),
    );

    compare_output(
        vakint
            .to_canonical(
                Atom::parse(
                    "(k(11,2)*k(11,2)+k(11,77)*k(22,77)+k(22,33)*p(42,33))*topo(\
                            prop(33,edge(10,10),k(22),mUVsq,2)*\
                            prop(55,edge(10,10),k(11),mUVsq,1)\
                        )",
                )
                .unwrap()
                .as_view(),
                true,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse("(k(2,2)^2+k(1,33)*p(42,33)+k(1,77)*k(2,77))*topo(I2L_pinch_3(mUVsq,2,1,0))")
            .unwrap(),
    );

    compare_output(
        vakint
            .to_canonical(
                Atom::parse("(k(2,2)^2+k(1,33)*p(42,33)+k(1,77)*k(2,77))*topo(I2L(mUVsq,2,1,0))")
                    .unwrap()
                    .as_view(),
                false,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse(
            "(k(2,2)^2+k(1,33)*p(42,33)+k(1,77)*k(2,77))*topo(\
                        prop(1,edge(1,2),k(1),mUVsq,2)*\
                        prop(2,edge(1,2),k(2),mUVsq,1)*\
                        prop(3,edge(2,1),k(1)+k(2),mUVsq,0)
                )",
        )
        .unwrap(),
    );

    compare_output(
        vakint
            .to_canonical(
                Atom::parse(
                    "(k(2,2)^2+k(1,33)*p(42,33)+k(1,77)*k(2,77))*topo(I2L_pinch_3(mUVsq,2,1,0))",
                )
                .unwrap()
                .as_view(),
                false,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse(
            "(k(2,2)^2+k(1,33)*p(42,33)+k(1,77)*k(2,77))*topo(\
                        prop(1,edge(1,1),k(1),mUVsq,2)*\
                        prop(2,edge(1,1),k(2),mUVsq,1)
                )",
        )
        .unwrap(),
    );
}

#[test_log::test]
fn test_3l_matching_with_zero_powers_in_short_form() {
    let vakint = get_vakint(VakintSettings {
        allow_unknown_integrals: false,
        ..VakintSettings::default()
    });

    debug!("Topologies:\n{}", vakint.topologies);

    compare_output(
        vakint
            .to_canonical(
                Atom::parse(
                    "( 1 )*topo(\
                prop(1,edge(1,2),k(1),muvsq,1)\
              * prop(2,edge(1,2),k(2),muvsq,1)\
              * prop(3,edge(1,2),k(3),muvsq,1)\
              * prop(4,edge(2,1),k(1)+k(2)+k(3),muvsq,2)\
            )",
                )
                .unwrap()
                .as_view(),
                true,
            )
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse("topo(I3L_pinch_1_6(muvsq,0,1,1,1,2,0))").unwrap(),
    );
}

#[test]
fn test_unknown_integrals() {
    let vakint = get_vakint(VakintSettings {
        allow_unknown_integrals: false,
        ..VakintSettings::default()
    });
    let vakint_with_unknown_integrals = get_vakint(VakintSettings {
        allow_unknown_integrals: true,
        ..VakintSettings::default()
    });

    let unknown_integral = Atom::parse(
        "(k(1,2)*k(1,2)+k(1,77)*k(2,77)+k(2,33)*p(42,33))*topo(\
                    prop(1,edge(7,10),k(2),mA,2)*\
                    prop(2,edge(7,10),k(1),mB,1)\
                )",
    )
    .unwrap();

    assert!(matches!(
        vakint.to_canonical(unknown_integral.as_view(), false),
        Err(VakintError::UnreckognizedIntegral(_))
    ));

    // println!("Topologies:\n{}", vakint_with_unknown_integrals.topologies);

    compare_output(
        vakint_with_unknown_integrals
            .to_canonical(unknown_integral.as_view(), false)
            .as_ref()
            .map(|a| a.as_view()),
        Atom::parse(
            "(k(1,2)^2+k(1,77)*k(2,77)+k(2,33)*p(42,33))*topo(UNKNOWN(\
                        prop(1,edge(7,10),k(2),mA,2)*\
                        prop(2,edge(7,10),k(1),mB,1))\
                    )",
        )
        .unwrap(),
    );
}
