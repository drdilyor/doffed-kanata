use super::*;

#[test]
fn transparent_base() {
    let result = simulate(
        "(dofcfg process-unmapped-keys yes concurrent-tap-hold yes) \
         (dofsrc a) \
         (doflayer base _)",
        "d:a t:50 u:a t:50",
    );
    assert_eq!("out:↓A\nt:50ms\nout:↑A", result);
}

#[test]
fn delegate_base() {
    let result = simulate(
        "(dofcfg process-unmapped-keys   yes \
                 delegate-to-first-layer yes)
         (dofsrc a b) \
         (doflayer base c (layer-switch 2)) \
         (doflayer 2 _ _)",
        "d:b t:50 u:b t:50 d:a t:50 u:a t:50",
    );
    assert_eq!("t:100ms\nout:↓C\nt:50ms\nout:↑C", result);
}

#[test]
fn delegate_base_but_base_is_transparent() {
    let result = simulate(
        "(dofcfg process-unmapped-keys   yes \
                 delegate-to-first-layer yes)
         (dofsrc a b) \
         (doflayer base _ (layer-switch 2)) \
         (doflayer 2 _ _)",
        "d:b t:50 u:b t:50 d:a t:50 u:a t:50",
    );
    assert_eq!("t:100ms\nout:↓A\nt:50ms\nout:↑A", result);
}

#[test]
fn layer_switching() {
    let result = simulate(
        "(dofcfg process-unmapped-keys   yes
                 delegate-to-first-layer yes)
         (dofsrc a b c d)
         (doflayer base x y z (layer-switch 2))
         (doflayer 2 e f _ (layer-switch 3))
         (doflayer 3 g _ _ (layer-switch 4))
         (doflayer 4 _ _ _ XX)
        ",
        "d:c t:20 u:c t:20 d:d t:20 u:d t:20
         d:b t:20 u:b t:20
         d:c t:20 u:c t:20
         d:d t:20 u:d t:20
         d:a t:20 u:a t:20
         d:b t:20 u:b t:20
         d:d t:20 u:d t:20
         d:a t:20 u:a t:20",
    );
    assert_eq!(
        "out:↓Z\nt:20ms\nout:↑Z\nt:60ms\nout:↓F\nt:20ms\nout:↑F\nt:20ms\nout:↓Z\nt:20ms\nout:↑Z\nt:60ms\nout:↓G\nt:20ms\nout:↑G\nt:20ms\nout:↓Y\nt:20ms\nout:↑Y\nt:60ms\nout:↓X\nt:20ms\nout:↑X",
        result
    );
}

#[test]
fn layer_holding() {
    let result = simulate(
        "(dofcfg process-unmapped-keys   yes
                 delegate-to-first-layer no)
         (dofsrc a b c d e f)
         (doflayer base x y z (layer-while-held 2) XX XX)
         (doflayer 2 e f _ XX (layer-while-held 3) XX)
         (doflayer 3 g _ _ XX XX (layer-while-held 4))
         (doflayer 4 _ _ _ XX XX XX)
        ",
        "d:c t:20 u:c t:20
         d:d t:20
         d:a t:20 u:a t:20
         d:b t:20 u:b t:20
         d:c t:20 u:c t:20
         d:e t:20
         d:a t:20 u:a t:20
         d:b t:20 u:b t:20
         d:c t:20 u:c t:20
         d:f t:20
         d:a t:20 u:a t:20
         d:b t:20 u:b t:20
         d:c t:20 u:c t:20",
    );
    assert_eq!(
        "out:↓Z\nt:20ms\nout:↑Z\nt:40ms\nout:↓E\nt:20ms\nout:↑E\nt:20ms\nout:↓F\nt:20ms\nout:↑F\nt:20ms\nout:↓Z\nt:20ms\nout:↑Z\nt:40ms\nout:↓G\nt:20ms\nout:↑G\nt:20ms\nout:↓F\nt:20ms\nout:↑F\nt:20ms\nout:↓Z\nt:20ms\nout:↑Z\nt:40ms\nout:↓G\nt:20ms\nout:↑G\nt:20ms\nout:↓F\nt:20ms\nout:↑F\nt:20ms\nout:↓Z\nt:20ms\nout:↑Z",
        result
    );
}
