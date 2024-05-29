use super::*;

#[test]
fn override_with_unmod() {
    let result = simulate(
        "
(dofoverrides
 (a) (b)
 (b) (a)
)

(dofalias
 b (unshift b)
 a (unshift a)
)
(dofsrc a b)
(doflayer base @a @b)
        ",
        "d:lsft d:a t:50 u:a t:50 d:b t:50 u:b t:50 u:lsft",
    )
    .to_ascii()
    .no_time();
    assert_eq!(
        "dn:LShift up:LShift dn:B up:B dn:LShift up:LShift dn:A up:A dn:LShift",
        result
    );
}
