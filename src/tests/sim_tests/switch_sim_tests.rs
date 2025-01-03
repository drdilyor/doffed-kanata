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
