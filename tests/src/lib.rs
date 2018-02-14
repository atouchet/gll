use std::fs::File;

pub mod gll10_g0 {
    include!(concat!(env!("OUT_DIR"), "/gll10_g0.rs"));
}

pub mod gll13_g1 {
    include!(concat!(env!("OUT_DIR"), "/gll13_g1.rs"));
}

pub mod gll15_g0 {
    include!(concat!(env!("OUT_DIR"), "/gll15_g0.rs"));
}

#[test]
fn gll10_g0() {
    gll10_g0::Parser::with(b"aad", |mut parser| {
        assert_eq!(
            gll10_g0::S::parse(&mut parser).unwrap().span.0,
            parser.input.range()
        );
        parser
            .gss
            .print(&mut File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/gll10-g0-gss.dot")).unwrap())
            .unwrap();
        parser
            .sppf
            .print(&mut File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/gll10-g0-sppf.dot")).unwrap())
            .unwrap();
    });
}

#[test]
fn gll13_g1() {
    gll13_g1::Parser::with(b"adb", |mut parser| {
        assert_eq!(
            gll13_g1::S::parse(&mut parser).unwrap().span.0,
            parser.input.range()
        );
        parser
            .gss
            .print(&mut File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/gll13-g1-gss.dot")).unwrap())
            .unwrap();
        parser
            .sppf
            .print(&mut File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/gll13-g1-sppf.dot")).unwrap())
            .unwrap();
    });
}

#[test]
fn gll15_g0() {
    gll15_g0::Parser::with(b"aac", |mut parser| {
        assert_eq!(
            gll15_g0::A::parse(&mut parser).unwrap().span.0,
            parser.input.range()
        );
        parser
            .gss
            .print(&mut File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/gll15-g0-gss.dot")).unwrap())
            .unwrap();
        parser
            .sppf
            .print(&mut File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/gll15-g0-sppf.dot")).unwrap())
            .unwrap();
    });
}
