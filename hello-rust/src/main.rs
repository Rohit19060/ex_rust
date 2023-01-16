fn main() {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    write!(t, "Hello, ").unwrap();
    t.fg(term::color::RED).unwrap();
    writeln!(t, "world!").unwrap();
    t.reset().unwrap();
}
