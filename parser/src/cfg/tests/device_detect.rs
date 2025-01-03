#[cfg(target_os = "linux")]
mod linux {
    use super::super::*;

    #[test]
    fn linux_device_parses_properly() {
        let source = r#"
(dofcfg linux-device-detect-mode any)
(dofsrc) (doflayer base)"#;
        let icfg = parse_cfg(source)
            .map_err(|e| log::info!("{:?}", miette::Error::from(e)))
            .expect("no error");
        assert_eq!(
            icfg.options.linux_opts.linux_device_detect_mode,
            Some(DeviceDetectMode::Any)
        );

        let source = r#"
(dofcfg linux-device-detect-mode keyboard-only)
(dofsrc) (doflayer base)"#;
        let icfg = parse_cfg(source)
            .map_err(|e| log::info!("{:?}", miette::Error::from(e)))
            .expect("no error");
        assert_eq!(
            icfg.options.linux_opts.linux_device_detect_mode,
            Some(DeviceDetectMode::KeyboardOnly)
        );

        let source = r#"
(dofcfg linux-device-detect-mode keyboard-mice)
(dofsrc) (doflayer base)"#;
        let icfg = parse_cfg(source)
            .map_err(|e| log::info!("{:?}", miette::Error::from(e)))
            .expect("no error");
        assert_eq!(
            icfg.options.linux_opts.linux_device_detect_mode,
            Some(DeviceDetectMode::KeyboardMice)
        );

        let source = r#"(dofsrc mmid) (doflayer base 1)"#;
        let icfg = parse_cfg(source)
            .map_err(|e| log::info!("{:?}", miette::Error::from(e)))
            .expect("no error");
        assert_eq!(
            icfg.options.linux_opts.linux_device_detect_mode,
            Some(DeviceDetectMode::Any)
        );

        let source = r#"(dofsrc a) (doflayer base b)"#;
        let icfg = parse_cfg(source)
            .map_err(|e| log::info!("{:?}", miette::Error::from(e)))
            .expect("no error");
        assert_eq!(
            icfg.options.linux_opts.linux_device_detect_mode,
            Some(DeviceDetectMode::KeyboardMice)
        );

        let source = r#"
(dofcfg linux-device-detect-mode not an opt)
(dofsrc) (doflayer base)"#;
        parse_cfg(source)
            .map(|_| ())
            .map_err(|e| log::info!("{:?}", miette::Error::from(e)))
            .expect_err("error should happen");
    }
}
