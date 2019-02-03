// Layout
use glutin::EventsLoop;
use std::cell::RefCell;
use std::rc::Rc;

use drawer::{Drawer, PropertiesCollection, CursorComputed};
use yoga::Direction;
use dom::events::*;
use dom::node::*;
use dom::traits::*;
use dom::tree::*;

// Core contexts
use super::render::WebRenderContext;
use super::window::Window;

pub struct Ui {
    pub render: WebRenderContext,
    pub dom: DOMTree<BasicEvent>,
    pub window: Window,

    dom_props: PropertiesCollection<DOMNodeId<BasicEvent>>,
    event_loop: Rc<RefCell<EventsLoop>>,
    hovered: CursorComputed,
    needs_redraw: bool,
    should_close: bool,
}

impl Ui {
    pub fn new(
        event_loop: Rc<RefCell<EventsLoop>>,
        render: WebRenderContext,
        mut dom: DOMTree<BasicEvent>,
        window: Window,
    ) -> Ui {
        {
            let mut document = dom.document_mut();

            document.build_layout();
            document.value_mut().reflow_subtree(1000, 500, Direction::LTR);
        }
        
        Ui {
            dom_props: PropertiesCollection::default(),
            hovered: CursorComputed::default(),
            should_close: false,
            needs_redraw: true,
            event_loop,
            render,
            window,
            dom,
        }
    }

    pub fn redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn close_app(self) {
        self.render.deinit();
    }

    pub fn update(&mut self) {
        let mut builder_context = self.render.render_builder(self.window.size_dp());
        let mut document = self.dom.document_mut();

        document.calculate_styles(); // calculate inner styles with new layout props
        document.value_mut().reflow_subtree(1000, 500, Direction::LTR); // recalculate yoga

        let mut list_builder = Drawer::new(
            &mut self.dom_props,
            &mut builder_context.builder,

            self.render.pipeline_id,
            self.render.document_id,
        );

        list_builder.built_node(&mut document);

        // Render blocks
        self.render.set_display_list(
            builder_context.resources,
            builder_context.builder,
            self.window.size_dp(),
        );

        self.render.update(self.window.size_px());
        self.window.swap_buffers();
    }
}
