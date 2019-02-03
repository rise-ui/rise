extern crate glutin;
extern crate jss;
extern crate dom;
extern crate rise;

#[path = "common/utils.rs"]
mod utils;

use rise::{
    app::{
        WindowPosition,
        WindowOptions,
        App,
    },
};

fn main() {
    let options = WindowOptions {
        position: WindowPosition::Center,
        title: "sample".to_string(),
        window_size: (1000, 500),
    };

    let dom = utils::get_sample_dom_tree();
    let app = App::new(options, dom);
    app.run();
}