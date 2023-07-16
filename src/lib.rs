use layout::{
    backends::svg::SVGWriter,
    gv::{self, GraphBuilder},
    topo::layout::VisualGraph,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn example_graph() -> String {
    "digraph { a -> b [label=\"foo\"]; }".to_string()
}

#[wasm_bindgen]
pub fn process(input: &str) -> String {
    let mut parser = gv::DotParser::new(input);

    let tree = parser.process().unwrap();

    let mut gb = GraphBuilder::new();
    gb.visit_graph(&tree);
    let mut vg = gb.get();
    generate_svg(&mut vg)
}

fn generate_svg(graph: &mut VisualGraph) -> String {
    let mut svg = SVGWriter::new();
    graph.do_it(false, true, true, &mut svg);
    svg.finalize()
}
