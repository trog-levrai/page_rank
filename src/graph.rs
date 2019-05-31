pub struct Graph<'a> {
    id: usize,
    neighbors: Vec<&'a Graph<'a>>,
}
