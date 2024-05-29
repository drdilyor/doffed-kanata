use super::*;

#[test]
fn switch_layer() {
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
