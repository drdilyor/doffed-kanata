use super::*;

#[test]
fn use_defsrc_deflayer() {
    let result = simulate(
        r##"
         (dofcfg)
         (dofsrc a b c d)
         (doflayer base
            1 2 3 (layer-while-held other)
         )
         (doflayer other
            4 5 (layer-while-held src) XX
         )
         (doflayer src
            use-dofsrc use-dofsrc XX XX
         )
        "##,
        "d:d d:c d:b d:a t:100",
    )
    .to_ascii();
    assert_eq!("t:2ms dn:B t:1ms dn:A", result);
}

#[test]
fn use_defsrc_deflayermap() {
    const CFG: &str = "
         (dofcfg process-unmapped-keys yes)
         (dofsrc a b c d)
         (doflayer base
            1
            (layer-while-held othermap1)
            (layer-while-held othermap2)
            (layer-while-held othermap3)
         )
         (doflayermap (othermap1)
            a 5
            ___ use-dofsrc
         )
         (doflayermap (othermap2)
            a 6
            __ use-dofsrc
            _ x
         )
         (doflayermap (othermap3)
            a 7
            _ use-dofsrc
            __ x
         )
        ";
    let result = simulate(CFG, "d:b d:a d:c d:e t:10").to_ascii();
    assert_eq!("t:1ms dn:Kb5 t:1ms dn:C t:1ms dn:E", result);
    let result = simulate(CFG, "d:c d:a d:c d:e t:10").to_ascii();
    assert_eq!("t:1ms dn:Kb6 t:1ms dn:X t:1ms dn:E", result);
    let result = simulate(CFG, "d:d d:a d:c d:e t:10").to_ascii();
    assert_eq!("t:1ms dn:Kb7 t:1ms dn:C t:1ms dn:X", result);
}
