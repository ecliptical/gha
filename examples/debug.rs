use gha::debug;

fn main() {
    debug!("single line");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct LongStruct {
        a: &'static str,
        b: usize,
        c: Option<Box<LongStruct>>,
    }

    let root = LongStruct {
        a: "test",
        b: 12,
        c: Some(Box::new(LongStruct {
            a: "another test",
            b: 34,
            c: None,
        })),
    };

    debug!("{root:#?}");
}
