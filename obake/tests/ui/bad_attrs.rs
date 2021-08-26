#[obake::versioned]
#[obake(version("0.1.0", extra_nonsense))]
struct Foo {}

#[obake::versioned]
#[obake(version("0.1.0"))]
struct Bar {
    #[obake(cfg("*"), extra_nonsense)]
    field_0: u32,
}

#[obake::versioned]
#[obake(version("0.1.0"))]
enum Baz {
    Variant {
        #[obake(not_an_obake_helper)]
        field_0: u32,
    },
}

#[obake::versioned]
#[obake(version("0.1.0"))]
enum Flim {
    Variant {
        #[obake(inherit, extra_nonsense)]
        field_0: u32,
    },
}

#[obake::versioned]
#[obake(version("0.1.0"))]
enum Flam {
    Variant {
        #[obake(cfg("*", extra_nonsense))]
        field_0: u32,
    },
}

fn main() {}
