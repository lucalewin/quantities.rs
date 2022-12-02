// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#[cfg(test)]
mod quantity_with_ref_unit_tests {
    use quantities::{assert_almost_eq, prelude::*, AMNT_ONE};

    /// Foo, a completely useless quantity
    #[quantity]
    #[ref_unit(A, "aaa", MEGA)]
    #[unit(B, "b", 0.4)]
    #[unit(C, "c", CENTI, 0.01)]
    struct Foo {}

    #[test]
    fn test_unit() {
        let a = A;
        let b = B;
        assert_eq!(a.name(), "A");
        assert_eq!(a.symbol(), "aaa");
        assert_eq!(a.si_prefix(), Some(SIPrefix::MEGA));
        assert_eq!(a.scale(), 1.0);
        assert_eq!(b.name(), "B");
        assert_eq!(b.symbol(), "b");
        assert_eq!(b.si_prefix(), None);
        assert_eq!(b.scale(), 0.4);
        assert_eq!(b.ratio(&a), 0.4);
        assert_eq!(a.ratio(&b), 2.5);
    }

    #[test]
    fn test_unit_to_string() {
        let unit = FooUnit::A;
        assert_eq!(unit.to_string(), "aaa");
    }

    #[test]
    fn test_unit_fmt() {
        let unit = FooUnit::A;
        assert_eq!(format!("{:>7}", unit), "    aaa");
        assert_eq!(format!("{:>7.2}", unit), "     aa");
    }

    #[test]
    fn test_unit_iter() {
        let mut iter_units = FooUnit::iter();
        assert_eq!(iter_units.next(), Some(&C));
        assert_eq!(iter_units.next(), Some(&B));
        assert_eq!(iter_units.next(), Some(&A));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_unit_from_symbol() {
        assert_eq!(FooUnit::from_symbol("aaa"), Some(A));
        assert_eq!(FooUnit::from_symbol("b"), Some(B));
        assert_eq!(FooUnit::from_symbol("c"), Some(C));
        assert_eq!(FooUnit::from_symbol("x"), None);
    }

    #[test]
    fn test_unit_from_scale() {
        assert_eq!(FooUnit::from_scale(AMNT_ONE), Some(A));
        assert_eq!(FooUnit::from_scale(0.4), Some(B));
        assert_eq!(FooUnit::from_scale(0.01), Some(C));
        assert_eq!(FooUnit::from_scale(10.0), None);
    }

    #[test]
    fn test_qty_iter_units() {
        let mut iter_units = Foo::iter_units();
        assert_eq!(iter_units.next(), Some(&C));
        assert_eq!(iter_units.next(), Some(&B));
        assert_eq!(iter_units.next(), Some(&A));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_fit() {
        let amnt = 0.007;
        let foo = Foo::_fit(amnt);
        assert_almost_eq!(foo.value, amnt / 0.01);
        assert_eq!(foo.unit, C);
        let amnt = 0.07;
        let foo = Foo::_fit(amnt);
        assert_almost_eq!(foo.value, amnt / 0.01);
        assert_eq!(foo.unit, C);
        let amnt = 0.7;
        let foo = Foo::_fit(amnt);
        assert_almost_eq!(foo.value, amnt / 0.01);
        assert_eq!(foo.unit, C);
        let amnt = 7.;
        let foo = Foo::_fit(amnt);
        assert_almost_eq!(foo.value, amnt);
        assert_eq!(foo.unit, A);
    }

    #[test]
    fn test_qty_unit_from_symbol() {
        assert_eq!(Foo::unit_from_symbol("aaa"), Some(A));
        assert_eq!(Foo::unit_from_symbol("b"), Some(B));
        assert_eq!(Foo::unit_from_symbol("c"), Some(C));
        assert_eq!(Foo::unit_from_symbol("x"), None);
    }

    #[test]
    fn test_qty_unit_from_scale() {
        assert_eq!(Foo::unit_from_scale(AMNT_ONE), Some(A));
        assert_eq!(Foo::unit_from_scale(0.4), Some(B));
        assert_eq!(Foo::unit_from_scale(0.01), Some(C));
        assert_eq!(Foo::unit_from_scale(10.0), None);
    }

    #[test]
    fn test_qty() {
        let amnt = 17.4;
        let unit = FooUnit::B;
        let qty = Foo::new(amnt, unit);
        assert_eq!(qty.value(), amnt);
        assert_eq!(qty.unit(), unit);
        let qty = amnt * unit;
        assert_eq!(qty.value(), amnt);
        assert_eq!(qty.unit(), unit);
    }

    #[test]
    fn test_qty_to_string() {
        let qty = Foo::new(184.09, FooUnit::A);
        assert_eq!(qty.to_string(), "184.09 aaa");
    }

    #[test]
    fn test_unitless_fmt() {
        let qty = 184.09 * ONE;
        assert_eq!(format!("{}", qty), "184.09");
        assert_eq!(format!("{:+}", qty), "+184.09");
        assert_eq!(format!("{:>9.3}", qty), "  184.090");
    }

    #[test]
    fn test_qty_fmt() {
        let qty = Foo::new(184.09, FooUnit::A);
        assert_eq!(format!("{}", qty), "184.09 aaa");
        assert_eq!(format!("{:+}", qty), "+184.09 aaa");
        assert_eq!(format!("{:*<+15}", qty), "+184.09 aaa****");
        assert_eq!(format!("{:_>15.1}", qty), "______184.1 aaa");
        assert_eq!(format!("{:_>5.0}", qty), "184 aaa");
        let qty = Foo::new(189.0, FooUnit::B);
        assert_eq!(format!("{:_^15.1}", qty), "____189.0 b____");
    }

    #[test]
    fn test_convert() {
        let qty = Foo::new(17.4, FooUnit::B);
        let equiv = qty.convert(FooUnit::A);
        assert_almost_eq!(equiv.value(), 6.96_f64);
        assert_eq!(equiv.unit(), FooUnit::A);
        let qty = equiv.convert(FooUnit::B);
        assert_almost_eq!(qty.value(), 17.4_f64);
        assert_eq!(qty.unit(), FooUnit::B);
    }

    #[test]
    fn test_cmp_same_unit() {
        let qty1 = 17.4 * FooUnit::A;
        let qty2 = 0.37 * FooUnit::A;
        let qty3 = qty1;
        assert!(qty1 == qty3);
        assert!(qty3 == qty1);
        assert!(qty1 != qty2);
        assert!(qty2 != qty1);
        assert!(qty2 < qty1);
        assert!(qty1 > qty2);
        assert!(qty2 <= qty3);
        assert!(qty3 >= qty2);
    }

    #[test]
    fn test_cmp_diff_unit() {
        let qty1 = 17.4 * FooUnit::A;
        let qty2 = 0.37 * FooUnit::B;
        let qty3 = qty1.convert(FooUnit::C);
        assert!(qty1 == qty3);
        assert!(qty3 == qty1);
        assert!(qty1 != qty2);
        assert!(qty2 != qty1);
        assert!(qty2 < qty1);
        assert!(qty1 > qty2);
        assert!(qty2 <= qty3);
        assert!(qty3 >= qty2);
    }

    #[test]
    fn test_add_same_unit() {
        let amnt1 = 0.1;
        let unit1 = FooUnit::A;
        let amnt2 = -0.2;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 + qty2;
        assert_almost_eq!(res.value(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 + qty1;
        assert_almost_eq!(res.value(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
    }

    #[test]
    fn test_add_diff_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.37;
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let res = qty1 + qty2;
        assert_almost_eq!(res.value(), amnt1 + amnt2 * 0.4);
        assert_eq!(res.unit(), unit1);
        let res = qty2 + qty1;
        assert_almost_eq!(res.value(), amnt1 * 2.5 + amnt2);
        assert_eq!(res.unit(), unit2);
    }

    #[test]
    fn test_sub_same_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.37;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 - qty2;
        assert_almost_eq!(res.value(), amnt1 - amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 - qty1;
        assert_almost_eq!(res.value(), amnt2 - amnt1);
        assert_eq!(res.unit(), unit1);
    }

    #[test]
    fn test_sub_diff_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.3;
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let res = qty1 - qty2;
        assert_almost_eq!(res.value(), amnt1 - amnt2 * 0.4);
        assert_eq!(res.unit(), unit1);
        let res = qty2 - qty1;
        assert_almost_eq!(res.value(), amnt2 - amnt1 * 2.5);
        assert_eq!(res.unit(), unit2);
    }

    #[test]
    fn test_div_same_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.3;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 / qty2;
        assert_almost_eq!(res.value(), amnt1 / amnt2);
        assert_eq!(res.unit(), ONE);
    }

    #[test]
    fn test_div_diff_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.3;
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let res = qty1 / qty2;
        assert_almost_eq!(res.value(), amnt1 / (amnt2 * 0.4));
        assert_eq!(res.unit(), ONE);
    }

    #[test]
    fn test_mul_amnt() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.37;
        let qty1 = amnt1 * unit1;
        let res = qty1 * amnt2;
        assert_almost_eq!(res.value(), amnt1 * amnt2);
        assert_eq!(res.unit(), qty1.unit());
        let res = amnt2 * qty1;
        assert_almost_eq!(res.value(), amnt1 * amnt2);
        assert_eq!(res.unit(), qty1.unit());
    }

    #[test]
    fn test_div_amnt() {
        let amnt1 = 15.54;
        let unit1 = FooUnit::A;
        let amnt2 = 3.7;
        let qty1 = amnt1 * unit1;
        let res = qty1 / amnt2;
        assert_almost_eq!(res.value(), amnt1 / amnt2);
        assert_eq!(res.unit(), qty1.unit());
    }
}

#[cfg(test)]
mod quantity_without_ref_unit_tests {
    use quantities::{assert_almost_eq, prelude::*};

    /// Foo, a completely useless quantity
    #[quantity]
    #[unit(A, "aaa")]
    #[unit(B, "b")]
    #[unit(C, "c")]
    struct Foo {}

    #[test]
    fn test_unit() {
        let b = B;
        assert_eq!(b.name(), "B");
        assert_eq!(b.symbol(), "b");
        assert!(b.si_prefix().is_none());
    }

    #[test]
    fn test_unit_iter() {
        let mut iter_units = FooUnit::iter();
        assert_eq!(iter_units.next(), Some(&A));
        assert_eq!(iter_units.next(), Some(&B));
        assert_eq!(iter_units.next(), Some(&C));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_qty_iter_units() {
        let mut iter_units = Foo::iter_units();
        assert_eq!(iter_units.next(), Some(&A));
        assert_eq!(iter_units.next(), Some(&B));
        assert_eq!(iter_units.next(), Some(&C));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_cmp_same_unit() {
        let qty1 = 17.4 * FooUnit::A;
        let qty2 = 0.37 * FooUnit::A;
        let qty3 = qty1;
        assert!(qty1 == qty3);
        assert!(qty3 == qty1);
        assert!(qty1 != qty2);
        assert!(qty2 != qty1);
        assert!(qty2 < qty1);
        assert!(qty1 > qty2);
        assert!(qty2 <= qty3);
        assert!(qty3 >= qty2);
    }

    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    #[test]
    fn test_cmp_diff_unit() {
        let qty1 = 17.4 * FooUnit::A;
        let qty2 = 0.37 * FooUnit::B;
        let qty3 = 17.4000 * FooUnit::A;
        assert!(qty1 == qty3);
        assert!(qty3 == qty1);
        assert!(qty1 != qty2);
        assert!(qty2 != qty1);
        assert!(!(qty2 < qty1));
        assert!(!(qty1 > qty2));
        assert!(!(qty2 <= qty3));
        assert!(!(qty3 >= qty2));
    }

    #[test]
    fn test_add_same_unit() {
        let amnt1 = 5000.17;
        let unit1 = FooUnit::C;
        let amnt2 = -2.0;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 + qty2;
        assert_almost_eq!(res.value(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 + qty1;
        assert_almost_eq!(res.value(), amnt1 + amnt2);
        assert_eq!(res.unit(), unit1);
    }

    #[test]
    #[should_panic]
    fn test_add_diff_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.37;
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let _res = qty1 + qty2;
    }

    #[test]
    fn test_sub_same_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.37;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 - qty2;
        assert_almost_eq!(res.value(), amnt1 - amnt2);
        assert_eq!(res.unit(), unit1);
        let res = qty2 - qty1;
        assert_almost_eq!(res.value(), amnt2 - amnt1);
        assert_eq!(res.unit(), unit1);
    }

    #[test]
    #[should_panic]
    fn test_sub_diff_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.3;
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let _res = qty1 - qty2;
    }

    #[test]
    fn test_div_same_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.3;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit1;
        let res = qty1 / qty2;
        assert_almost_eq!(res, amnt1 / amnt2);
    }

    #[test]
    #[should_panic]
    fn test_div_diff_unit() {
        let amnt1 = 17.4;
        let unit1 = FooUnit::A;
        let amnt2 = 0.3;
        let unit2 = FooUnit::B;
        let qty1 = amnt1 * unit1;
        let qty2 = amnt2 * unit2;
        let _res = qty1 / qty2;
    }
}

#[cfg(test)]
mod quantity_single_unit_tests {
    use quantities::{assert_almost_eq, prelude::*};

    #[quantity]
    #[unit(Pop, "p")]
    struct Foo {}

    #[test]
    fn test_unit_iter() {
        let mut iter_units = FooUnit::iter();
        assert_eq!(iter_units.next(), Some(&POP));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_qty_iter_units() {
        let mut iter_units = Foo::iter_units();
        assert_eq!(iter_units.next(), Some(&POP));
        assert_eq!(iter_units.next(), None);
    }

    #[test]
    fn test_single_unit_qty() {
        let amnt = 17.4;
        let qty = Foo::new(amnt, POP);
        assert_eq!(qty.value(), amnt);
        let qty = amnt * POP;
        assert_eq!(qty.value(), amnt);
        assert_eq!(qty.unit(), POP);
        let qty = POP * amnt;
        assert_eq!(qty.value(), amnt);
        assert_eq!(qty.unit(), POP);
    }

    #[test]
    fn test_add() {
        let amnt1 = 517.04;
        let amnt2 = 14.3;
        let qty1 = amnt1 * POP;
        let qty2 = amnt2 * POP;
        let res = qty1 + qty2;
        assert_almost_eq!(res.value(), amnt1 + amnt2);
        assert_eq!(res.unit(), POP);
        let res = qty2 + qty1;
        assert_almost_eq!(res.value(), amnt1 + amnt2);
        assert_eq!(res.unit(), POP);
    }

    #[test]
    fn test_sub() {
        let amnt1 = 14.3;
        let amnt2 = 517.04;
        let qty1 = amnt1 * POP;
        let qty2 = amnt2 * POP;
        let res = qty1 - qty2;
        assert_almost_eq!(res.value(), amnt1 - amnt2);
        assert_eq!(res.unit(), POP);
        let res = qty2 - qty1;
        assert_almost_eq!(res.value(), amnt2 - amnt1);
        assert_eq!(res.unit(), POP);
    }

    #[test]
    fn test_div() {
        let amnt1 = 510.4;
        let amnt2 = 1.407;
        let qty1 = amnt1 * POP;
        let qty2 = amnt2 * POP;
        let res = qty1 / qty2;
        assert_almost_eq!(res, amnt1 / amnt2);
        let res = qty2 / qty1;
        assert_almost_eq!(res, amnt2 / amnt1);
    }
}

#[cfg(test)]
mod derived_quantity_tests {
    use quantities::{assert_almost_eq, prelude::*};

    #[quantity]
    #[ref_unit(Flop, "f")]
    #[unit(Kiloflop, "kf", 1000., "1000·f")]
    #[unit(Centiflop, "cf", 0.01, "0.01·f")]
    struct Foo {}

    #[quantity]
    #[ref_unit(Emil, "e")]
    #[unit(Milliemil, "me", 0.001, "0.001·e")]
    #[unit(Microemil, "µe", 0.000001, "0.000001·e")]
    #[unit(Kiloemil, "ke", 1000., "1000·e")]
    struct Bar {}

    #[quantity(Foo * Bar)]
    #[ref_unit(Bazoo, "b", "1·f·e")]
    #[unit(Millibazoo, "mb", 0.001, "0.001·b")]
    #[unit(Microbazoo, "µb", 0.000001, "0.000001·b")]
    #[unit(Kilobazoo, "kb", 1000., "1000·b")]
    struct Baz {}

    #[quantity(Foo / Bar)]
    #[ref_unit(Qoox, "Q", "1·f/e")]
    #[unit(Five_Flops_per_Emil, "ff/e", 5., "5·f/e")]
    #[unit(Milliqoox, "mQ", 0.001, "0.001·Q")]
    #[unit(Microqoox, "µQ", 0.000001, "0.000001·Q")]
    #[unit(Kiloqoox, "kQ", 1000., "1000·Q")]
    struct Qoo {}

    #[test]
    fn test_qty() {
        let amnt = 17.4;
        let unit = BazUnit::Microbazoo;
        let qty = Baz::new(amnt, unit);
        assert_eq!(qty.value(), amnt);
        assert_eq!(qty.unit(), unit);
        let qty = amnt * unit;
        assert_eq!(qty.value(), amnt);
        assert_eq!(qty.unit(), unit);
    }

    fn check_qty_mul_qty(x: Foo, y: Bar, r: Baz) {
        let z = x * y;
        assert_almost_eq!(z.value(), r.value());
        assert_eq!(z.unit(), r.unit());
        let z = y * x;
        assert_almost_eq!(z.value(), r.value());
        assert_eq!(z.unit(), r.unit());
        // reverse divs
        let z = (r / x).convert(y.unit());
        assert_almost_eq!(z.value(), y.value());
        assert_eq!(z.unit(), y.unit());
        let z = (r / y).convert(x.unit());
        assert_almost_eq!(z.value(), x.value());
        assert_eq!(z.unit(), x.unit());
    }

    #[test]
    fn test_qty_mul_qty() {
        check_qty_mul_qty(
            17.4 * FLOP,
            3. * EMIL,
            17.4 * 3. * BAZOO,
        );
        check_qty_mul_qty(
            14.52 * KILOFLOP,
            0.47 * MICROEMIL,
            14.52 * 0.47 * MILLIBAZOO,
        );
        check_qty_mul_qty(
            14.52 * CENTIFLOP,
            0.47 * MICROEMIL,
            14.52 * 0.47 * 0.01 * MICROBAZOO,
        );
    }

    fn check_qty_div_qty(x: Foo, y: Bar, r: Qoo) {
        let z = x / y;
        assert_almost_eq!(z.value(), r.value());
        assert_eq!(z.unit(), r.unit());
        // reverse mul
        let z = (r * y).convert(x.unit());
        assert_almost_eq!(z.value(), x.value());
        assert_eq!(z.unit(), x.unit());
    }

    #[test]
    fn test_qty_div_qty() {
        check_qty_div_qty(
            17.4 * FLOP,
            3. * EMIL,
            17.4 / 3. * QOOX,
        );
        check_qty_div_qty(
            14.52 * KILOFLOP,
            3.3 * MILLIEMIL,
            (14.52 / 3.3) * 1000. * KILOQOOX,
        );
        check_qty_div_qty(
            14.52 * CENTIFLOP,
            3.3 * KILOEMIL,
            14.52 / 3.3 * 10. * MICROQOOX,
        );
    }
}
