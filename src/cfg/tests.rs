use super::*;
use crate::cfg::sexpr::parse;

use std::sync::Mutex;

static CFG_PARSE_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn sizeof_action_is_two_usizes() {
    assert_eq!(
        std::mem::size_of::<KanataAction>(),
        std::mem::size_of::<usize>() * 2
    );
}

#[test]
fn span_works() {
    let s = "(hello world my oyster)\n(row two)";
    let tlevel = parse(s).unwrap();
    assert_eq!(
        &s[tlevel[0].span.start..tlevel[0].span.end],
        "(hello world my oyster)"
    );
    assert_eq!(&s[tlevel[1].span.start..tlevel[1].span.end], "(row two)");
}

#[test]
fn parse_simple() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    new_from_file(&std::path::PathBuf::from("./cfg_samples/simple.kbd")).unwrap();
}

#[test]
fn parse_minimal() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    new_from_file(&std::path::PathBuf::from("./cfg_samples/minimal.kbd")).unwrap();
}

#[test]
fn parse_default() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    new_from_file(&std::path::PathBuf::from("./cfg_samples/kanata.kbd")).unwrap();
}

#[test]
fn parse_jtroo() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let cfg = new_from_file(&std::path::PathBuf::from("./cfg_samples/jtroo.kbd")).unwrap();
    assert_eq!(cfg.layer_info.len(), 16);
}

#[test]
fn parse_f13_f24() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    new_from_file(&std::path::PathBuf::from("./cfg_samples/f13_f24.kbd")).unwrap();
}

#[test]
fn parse_action_vars() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let mut s = ParsedState::default();
    let source = r#"
(dofvar
  one 1
  two 2
  a a
  base base
  three ($a b c)
  chr C-S-v
  td ($a b $chr)
  four (lctl d)
)
(dofvar
  five (lsft e)
  rel release
  e  example
  e2 example2
  chord1 (chord $e $one)
  chord2 (chord $e2 $one)
  1 (1)
  full-action (tap-dance $one $three)
)
(dofalias
  tdl (tap-dance $two $td)
  tde (tap-dance-eager $two $td)
  unc (unicode $one)
  rlk (release-key $one)
  mul (multi $two $one)
  mwu (mwheel-up $one $two)
  mmu (movemouse-up $one $two)
  mau (movemouse-accel-up $one $two $one $two)
  ons (one-shot $one $two)
  thd (tap-hold $one $two $chr $two)
  tht (tap-hold-release-timeout $one $two $chr $two $one)
  thk (tap-hold-release-keys $one $two $chr $two $three)
  mac (macro $one $two $one $two $chr C-S-$three $one)
  rmc (macro-repeat $one $two $one $two $chr C-S-$three $one)
  dr1 (dynamic-macro-record $one)
  dp1 (dynamic-macro-play $one)
  abc (arbitrary-code $one)
  opf (on-press-fakekey $one $rel)
  orf (on-release-fakekey $one $rel)
  fla $full-action
  frk (fork $one $two $five)
  cpw (caps-word-custom $one $three $four)
  rst (dynamic-macro-record-stop-truncate $one)
  stm (setmouse $one $two)
)
(dofsrc a b c d)
(doflayer base $chord1 $chord2 $chr @tdl)
(dofoverrides
  ($two) ($one)
  ($one) $four
  $five ($two)
  $four $five
)
(doffakekeys
  $one $two
)
(dofseq $one $three)
(dofchords $e $one $1 $two)
(dofchords $e2 $one ($one) $two)
"#;
    s.cfg_text = source.into();
    parse_cfg_raw_string(source.into(), &mut s)
        .map_err(|e| {
            eprintln!("{:?}", error_with_source(e.into(), &s));
            ""
        })
        .unwrap();
}

#[test]
fn parse_transparent_default() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let mut s = ParsedState::default();
    let (_, _, layer_strings, layers, _, _) = parse_cfg_raw(
        &std::path::PathBuf::from("./cfg_samples/transparent_default.kbd"),
        &mut s,
    )
    .unwrap();

    assert_eq!(layer_strings.len(), 4);

    assert_eq!(
        layers[0][0][usize::from(OsCode::KEY_F13)],
        Action::KeyCode(KeyCode::F13)
    );
    assert_eq!(
        layers[0][0][usize::from(OsCode::KEY_F14)],
        Action::DefaultLayer(2)
    );
    assert_eq!(layers[0][0][usize::from(OsCode::KEY_F15)], Action::Layer(3));
    assert_eq!(layers[1][0][usize::from(OsCode::KEY_F13)], Action::Trans);
    assert_eq!(
        layers[1][0][usize::from(OsCode::KEY_F14)],
        Action::DefaultLayer(2)
    );
    assert_eq!(layers[1][0][usize::from(OsCode::KEY_F15)], Action::Layer(3));
    assert_eq!(
        layers[2][0][usize::from(OsCode::KEY_F13)],
        Action::DefaultLayer(0)
    );
    assert_eq!(layers[2][0][usize::from(OsCode::KEY_F14)], Action::Layer(1));
    assert_eq!(
        layers[2][0][usize::from(OsCode::KEY_F15)],
        Action::KeyCode(KeyCode::F15)
    );
    assert_eq!(
        layers[3][0][usize::from(OsCode::KEY_F13)],
        Action::DefaultLayer(0)
    );
    assert_eq!(layers[3][0][usize::from(OsCode::KEY_F14)], Action::Layer(1));
    assert_eq!(layers[3][0][usize::from(OsCode::KEY_F15)], Action::Trans);
}

#[test]
fn parse_all_keys() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    new_from_file(&std::path::PathBuf::from(
        "./cfg_samples/all_keys_in_defsrc.kbd",
    ))
    .unwrap();
}

#[test]
fn parse_multiline_comment() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    new_from_file(&std::path::PathBuf::from(
        "./test_cfgs/multiline_comment.kbd",
    ))
    .unwrap();
}

#[test]
fn disallow_nested_tap_hold() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    match new_from_file(&std::path::PathBuf::from("./test_cfgs/nested_tap_hold.kbd"))
        .map_err(|e| format!("{e:?}"))
    {
        Ok(_) => panic!("invalid nested tap-hold in tap action was Ok'd"),
        Err(e) => assert!(e.contains("tap-hold"), "real e: {e}"),
    }
}

#[test]
fn disallow_ancestor_seq() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    match new_from_file(&std::path::PathBuf::from("./test_cfgs/ancestor_seq.kbd"))
        .map_err(|e| format!("{e:?}"))
    {
        Ok(_) => panic!("invalid ancestor seq was Ok'd"),
        Err(e) => assert!(e.contains("is contained")),
    }
}

#[test]
fn disallow_descendent_seq() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    match new_from_file(&std::path::PathBuf::from("./test_cfgs/descendant_seq.kbd"))
        .map_err(|e| format!("{e:?}"))
    {
        Ok(_) => panic!("invalid descendant seq was Ok'd"),
        Err(e) => assert!(e.contains("contains")),
    }
}

#[test]
fn disallow_multiple_waiting_actions() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    match new_from_file(&std::path::PathBuf::from("./test_cfgs/bad_multi.kbd"))
        .map_err(|e| format!("{e:?}"))
    {
        Ok(_) => panic!("invalid multiple waiting actions Ok'd"),
        Err(e) => assert!(e.contains("Cannot combine multiple")),
    }
}

#[test]
fn chord_in_macro_dont_panic() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    new_from_file(&std::path::PathBuf::from(
        "./test_cfgs/macro-chord-dont-panic.kbd",
    ))
    .map(|_| ())
    .expect_err("graceful failure, no panic, also no success");
}

#[test]
fn unknown_defcfg_item_fails() {
    let _lk = match CFG_PARSE_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    new_from_file(&std::path::PathBuf::from(
        "./test_cfgs/unknown_defcfg_opt.kbd",
    ))
    .map(|_| ())
    .expect_err("graceful failure, no panic, also no success");
}

#[test]
fn recursive_multi_is_flattened() {
    macro_rules! atom {
        ($e:expr) => {
            SExpr::Atom(Spanned::new($e.into(), Span::default()))
        };
    }
    macro_rules! list {
        ($e:tt) => {
            SExpr::List(Spanned::new(vec! $e, Span::default()))
        };
    }
    use sexpr::*;
    let params = [
        atom!("a"),
        atom!("mmtp"),
        list!([
            atom!("multi"),
            atom!("b"),
            atom!("mltp"),
            list!([atom!("multi"), atom!("c"), atom!("mrtp"),])
        ]),
    ];
    let s = ParsedState::default();
    if let KanataAction::MultipleActions(parsed_multi) = parse_multi(&params, &s).unwrap() {
        assert_eq!(parsed_multi.len(), 4);
        assert_eq!(parsed_multi[0], Action::KeyCode(KeyCode::A));
        assert_eq!(parsed_multi[1], Action::KeyCode(KeyCode::B));
        assert_eq!(parsed_multi[2], Action::KeyCode(KeyCode::C));
        assert_eq!(
            parsed_multi[3],
            Action::Custom(
                &&[
                    &CustomAction::MouseTap(Btn::Mid),
                    &CustomAction::MouseTap(Btn::Left),
                    &CustomAction::MouseTap(Btn::Right),
                ][..]
            )
        );
    } else {
        panic!("multi did not parse into multi");
    }
}

#[test]
fn test_parse_sequence_a_b() {
    let seq = parse_sequence_keys(
        &parse("(a b)").expect("parses")[0].t,
        &ParsedState::default(),
    )
    .expect("parses");
    assert_eq!(seq.len(), 2);
    assert_eq!(&seq[0], &u16::from(OsCode::KEY_A));
    assert_eq!(&seq[1], &u16::from(OsCode::KEY_B));
}

#[test]
fn test_parse_sequence_sa_b() {
    let seq = parse_sequence_keys(
        &parse("(S-a b)").expect("parses")[0].t,
        &ParsedState::default(),
    )
    .expect("parses");
    assert_eq!(seq.len(), 3);
    assert_eq!(&seq[0], &(u16::from(OsCode::KEY_LEFTSHIFT) | 0x8000));
    assert_eq!(&seq[1], &(u16::from(OsCode::KEY_A) | 0x8000));
    assert_eq!(&seq[2], &u16::from(OsCode::KEY_B));
}

#[test]
fn test_parse_sequence_sab() {
    let seq = parse_sequence_keys(
        &parse("(S-(a b))").expect("parses")[0].t,
        &ParsedState::default(),
    )
    .expect("parses");
    assert_eq!(seq.len(), 3);
    assert_eq!(&seq[0], &(u16::from(OsCode::KEY_LEFTSHIFT) | 0x8000));
    assert_eq!(&seq[1], &(u16::from(OsCode::KEY_A) | 0x8000));
    assert_eq!(&seq[2], &(u16::from(OsCode::KEY_B) | 0x8000));
}

#[test]
fn test_parse_sequence_bigchord() {
    let seq = parse_sequence_keys(
        &parse("(AG-A-M-C-S-(a b) c)").expect("parses")[0].t,
        &ParsedState::default(),
    )
    .expect("parses");
    assert_eq!(seq.len(), 8);
    assert_eq!(&seq[0], &(u16::from(OsCode::KEY_RIGHTALT) | 0x1000));
    assert_eq!(&seq[1], &(u16::from(OsCode::KEY_LEFTALT) | 0x3000));
    assert_eq!(&seq[2], &(u16::from(OsCode::KEY_LEFTMETA) | 0x3800));
    assert_eq!(&seq[3], &(u16::from(OsCode::KEY_LEFTCTRL) | 0x7800));
    assert_eq!(&seq[4], &(u16::from(OsCode::KEY_LEFTSHIFT) | 0xF800));
    assert_eq!(&seq[5], &(u16::from(OsCode::KEY_A) | 0xF800));
    assert_eq!(&seq[6], &(u16::from(OsCode::KEY_B) | 0xF800));
    assert_eq!(&seq[7], &(u16::from(OsCode::KEY_C)));
}

#[test]
fn test_parse_sequence_inner_chord() {
    let seq = parse_sequence_keys(
        &parse("(S-(a b C-c) d)").expect("parses")[0].t,
        &ParsedState::default(),
    )
    .expect("parses");
    assert_eq!(seq.len(), 6);
    assert_eq!(&seq[0], &(u16::from(OsCode::KEY_LEFTSHIFT) | 0x8000));
    assert_eq!(&seq[1], &(u16::from(OsCode::KEY_A) | 0x8000));
    assert_eq!(&seq[2], &(u16::from(OsCode::KEY_B) | 0x8000));
    assert_eq!(&seq[3], &(u16::from(OsCode::KEY_LEFTCTRL) | 0xC000));
    assert_eq!(&seq[4], &(u16::from(OsCode::KEY_C) | 0xC000));
    assert_eq!(&seq[5], &(u16::from(OsCode::KEY_D)));
}

#[test]
fn test_parse_sequence_earlier_inner_chord() {
    let seq = parse_sequence_keys(
        &parse("(S-(a C-b c) d)").expect("parses")[0].t,
        &ParsedState::default(),
    )
    .expect("parses");
    assert_eq!(seq.len(), 6);
    assert_eq!(&seq[0], &(u16::from(OsCode::KEY_LEFTSHIFT) | 0x8000));
    assert_eq!(&seq[1], &(u16::from(OsCode::KEY_A) | 0x8000));
    assert_eq!(&seq[2], &(u16::from(OsCode::KEY_LEFTCTRL) | 0xC000));
    assert_eq!(&seq[3], &(u16::from(OsCode::KEY_B) | 0xC000));
    assert_eq!(&seq[4], &(u16::from(OsCode::KEY_C) | 0x8000));
    assert_eq!(&seq[5], &(u16::from(OsCode::KEY_D)));
}

#[test]
fn test_parse_sequence_numbers() {
    let seq = parse_sequence_keys(
        &parse("(0 1 2 3 4 5 6 7 8 9)").expect("parses")[0].t,
        &ParsedState::default(),
    )
    .expect("parses");
    assert_eq!(seq.len(), 10);
    assert_eq!(&seq[0], &u16::from(OsCode::KEY_0));
    assert_eq!(&seq[1], &u16::from(OsCode::KEY_1));
    assert_eq!(&seq[2], &u16::from(OsCode::KEY_2));
    assert_eq!(&seq[3], &u16::from(OsCode::KEY_3));
    assert_eq!(&seq[4], &u16::from(OsCode::KEY_4));
    assert_eq!(&seq[5], &u16::from(OsCode::KEY_5));
    assert_eq!(&seq[6], &u16::from(OsCode::KEY_6));
    assert_eq!(&seq[7], &u16::from(OsCode::KEY_7));
    assert_eq!(&seq[8], &u16::from(OsCode::KEY_8));
    assert_eq!(&seq[9], &u16::from(OsCode::KEY_9));
}

#[test]
fn test_parse_macro_numbers() {
    // Note, can't test zero in this way because a delay of 0 isn't allowed by the parsing.
    let exprs = parse("(1 2 3 4 5 6 7 8 9)").expect("parses")[0].t.clone();
    let mut expr_rem = exprs.as_slice();
    let mut i = 1;
    while expr_rem.len() > 0 {
        let (macro_events, expr_rem_tmp) =
            parse_macro_item(expr_rem, &ParsedState::default()).expect("parses");
        expr_rem = expr_rem_tmp;
        assert_eq!(macro_events.len(), 1);
        match &macro_events[0] {
            SequenceEvent::Delay { duration } => assert_eq!(duration, &i),
            ev => panic!("expected delay, {ev:?}"),
        }
        i += 1;
    }

    let exprs = parse("(0)").expect("parses")[0].t.clone();
    parse_macro_item(exprs.as_slice(), &ParsedState::default()).expect_err("errors");
}
