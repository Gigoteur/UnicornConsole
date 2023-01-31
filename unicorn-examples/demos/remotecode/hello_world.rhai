let x = 42;

fn _init() {
    print("hello from rhai");
    print(`hello${x}`);     // prints "hello42" to stdout
    circfill(10, 20, 10, 8);
}

fn _update() {
    x = x + 1;
    print(`UPDATE ${x}`);
}

fn _draw() {
    print(`DRAW ${x}`);
    cls(0);
    circfill(10+x, 20+x, 10+x, 8);
}