#![allow(unused)]

#[obake::versioned]
#[obake(version("0.1.0"))]
#[obake(version("0.2.0"))]
#[obake(version("0.3.0"))]
#[derive(Default)]
struct Foo {
    field_0: u32,
    #[obake(cfg("0.2.0"))]
    field_1: String,
    #[obake(cfg("0.1.0"))]
    #[obake(cfg("0.3.0"))]
    field_2: i64,
}

impl From<Foo!["0.1.0"]> for Foo!["0.2.0"] {
    fn from(from: Foo!["0.1.0"]) -> Self {
        Self {
            field_0: from.field_0,
            field_1: "default".to_owned(),
        }
    }
}

impl From<Foo!["0.2.0"]> for Foo!["0.3.0"] {
    fn from(from: Foo!["0.2.0"]) -> Self {
        Self {
            field_0: from.field_0,
            field_2: 42,
        }
    }
}

#[obake::versioned]
#[obake(version("0.1.0"))]
#[obake(version("0.2.0"))]
#[obake(version("0.3.0"))]
#[derive(Default)]
struct Bar {
    #[obake(inherit)]
    #[obake(cfg(">=0.2"))]
    field_0: Foo,
}

impl From<Bar!["0.1.0"]> for Bar!["0.2.0"] {
    fn from(from: Bar!["0.1.0"]) -> Self {
        Default::default()
    }
}

impl From<Bar!["0.2.0"]> for Bar!["0.3.0"] {
    fn from(from: Bar!["0.2.0"]) -> Self {
        Self {
            field_0: from.field_0.into(),
        }
    }
}

#[obake::versioned]
#[obake(version("0.1.0"))]
#[obake(version("0.2.0"))]
#[obake(version("0.3.0"))]
enum Baz {
    #[obake(cfg("<0.3"))]
    X(String),
    #[obake(cfg(">=0.2"))]
    Y {
        #[obake(inherit)]
        #[obake(cfg(">=0.2"))]
        foo: Foo,
        #[obake(inherit)]
        #[obake(cfg(">=0.2"))]
        bar: Bar,
    },
}

impl From<Baz!["0.1.0"]> for Baz!["0.2.0"] {
    fn from(from: Baz!["0.1.0"]) -> Self {
        type Baz = Baz!["0.1.0"];
        match from {
            Baz::X(x) => Self::X(x),
        }
    }
}

impl From<Baz!["0.2.0"]> for Baz!["0.3.0"] {
    fn from(from: Baz!["0.2.0"]) -> Self {
        type Baz = Baz!["0.2.0"];
        match from {
            Baz::X(_) => Self::Y {
                foo: Default::default(),
                bar: Default::default(),
            },
            Baz::Y { foo, bar } => Self::Y {
                foo: foo.into(),
                bar: bar.into(),
            },
        }
    }
}
