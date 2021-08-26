#[obake::versioned]
#[obake(version(not_a_sem_str))]
struct Foo {}

#[obake::versioned]
#[obake(version("not a semver"))]
struct Bar {}

#[obake::versioned]
#[obake(version("0.1.0"))]
struct Baz {
    #[obake(cfg(not_a_ver_str))]
    field_0: u32,
}

#[obake::versioned]
#[obake(version("0.1.0"))]
struct Flim {
    #[obake(cfg("not a semver constraint"))]
    field_0: u32,
}

fn main() {}
