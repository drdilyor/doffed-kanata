use super::*;

#[test]
fn sim_switch_layer() {
    let result = simulate(
        "
         (dofcfg)
         (dofsrc a b)
         (dofalias b (switch
            ((layer base)) x break
            ((layer other)) y break))
         (doflayer base (layer-while-held other) @b)
         (doflayer other XX @b)
        ",
        "d:b u:b t:10 d:a d:b u:b u:a t:10",
    )
    .no_time();
    assert_eq!("out:↓X out:↑X out:↓Y out:↑Y", result);
}

#[test]
fn sim_switch_base_layer() {
    let result = simulate(
        "
         (dofcfg)
         (dofsrc a b c)
         (dofalias b (switch
            ((base-layer base)) x break
            ((base-layer other)) y break))
         (doflayer base (layer-switch other) @b c)
         (doflayer other XX @b (layer-while-held base))
        ",
        "d:b u:b t:10 d:a d:b u:b u:a t:10 d:c t:10 d:b t:10 u:c u:b t:10",
    )
    .no_time();
    assert_eq!("out:↓X out:↑X out:↓Y out:↑Y out:↓Y out:↑Y", result);
}

#[test]
fn sim_switch_noop() {
    let result = simulate(
        "
         (dofsrc)
         (doflayermap (-) a XX b (switch
          ((input real a)) c break
          () d break
         ))
        ",
        "d:a d:b t:10 u:a u:b t:10 d:b u:b t:10",
    )
    .no_time();
    assert_eq!("out:↓C out:↑C out:↓D out:↑D", result);
}

#[test]
fn sim_switch_trans_not_top_layer() {
    let result = simulate(
        "
        (dofalias init (multi (layer-while-held l1) (layer-while-held l2) (layer-while-held l3) (layer-while-held l4)))
        (dofsrc a b)
        (doflayer l0 c @init)
        (doflayer l1 b @init)
        (doflayer l2 (switch () _ break) @init)
        (doflayer l3 _ @init)
        (doflayer l4 _ @init)
        ",
        "d:b t:20 d:a t:10 u:a t:100 d:a t:10 u:a t:100",
    )
    .to_ascii();
    assert_eq!("t:21ms dn:B t:9ms up:B t:101ms dn:B t:9ms up:B", result);
}
