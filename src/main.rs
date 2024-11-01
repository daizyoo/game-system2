struct Query<C: Component> {
    content: C,
}

struct Res<R: Resource> {
    content: R,
}

impl<T: Component> Query<T> {
    fn new(content: T) -> Self {
        Self { content }
    }

    fn get(&self) -> &T {
        &self.content
    }
}

trait Component {}

trait Resource {}

trait Arg {}

impl<C: Component> Arg for Query<C> {}
impl<R: Resource> Arg for Res<R> {}

trait Args {}

impl Args for () {}
impl<A1: Arg> Args for A1 {}
impl<A1: Arg, A2: Arg> Args for (A1, A2) {}
impl<A1: Arg, A2: Arg, A3: Arg> Args for (A1, A2, A3) {}
impl<A1: Arg, A2: Arg, A3: Arg, A4: Arg> Args for (A1, A2, A3, A4) {}

trait System {
    type Args: Args;

    fn run(&self, arg: Self::Args);
}

impl System for fn() {
    type Args = ();
    fn run(&self, _: Self::Args) {
        self()
    }
}

impl<A: Arg> System for fn(A) {
    type Args = A;
    fn run(&self, arg: Self::Args) {
        self(arg)
    }
}

impl<A1: Arg, A2: Arg> System for fn(A1, A2) {
    type Args = (A1, A2);
    fn run(&self, arg: Self::Args) {
        self(arg.0, arg.1)
    }
}

impl<A1: Arg, A2: Arg, A3: Arg> System for fn(A1, A2, A3) {
    type Args = (A1, A2, A3);
    fn run(&self, arg: Self::Args) {
        self(arg.0, arg.1, arg.2)
    }
}

/// # Example
/// ```
/// fn system(Query<Data>) {
///     println!("{:?}", q.get());
/// }
/// let app = App::new()
/// app.add_system(system)
/// app.add_data(Data { value: "Hello World".to_string() })
/// app.run()
/// ```
///
/// # Output
/// Hello World
///
fn main() {
    let sys = system2 as fn(Query<Data>, Query<Data>);
    sys.run((
        Query::new(Data {
            value: "Data1".to_string(),
        }),
        Query::new(Data {
            value: "Data2".to_string(),
        }),
    ));
}

#[derive(Debug)]
#[allow(unused)]
struct Data {
    value: String,
}

fn system(q: Query<Data>) {
    println!("{:?}", q.get());
}

fn system2(q: Query<Data>, q2: Query<Data>) {
    println!("{:?}\n{:?}", q.get(), q2.get());
}

impl Component for Data {}
