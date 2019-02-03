#[macro_use]
pub mod id;
pub mod font;
pub mod image;

use std::collections::HashMap;
use std::default::Default;
use std::sync::{Mutex, MutexGuard};

use super::webrender::api::RenderApiSender;

use self::font::FontLoader;
use self::id::{Id, IdGen};
use self::image::ImageLoader;

lazy_static! {
    static ref RES: Mutex<Resources> = Mutex::new(Resources::new());
}

pub fn init_resources(render_api: RenderApiSender) {
    RES.try_lock().unwrap().set_render_api(render_api);
}
// Allow global access to Resources
pub fn resources() -> MutexGuard<'static, Resources> {
    RES.try_lock().unwrap()
}

named_id!(WidgetId);

/// Map for a given `Id` and resource type.
pub struct Map<I, T> {
    id_gen: IdGen<I>,
    map: HashMap<I, T>,
}

impl<I: Id, T> Default for Map<I, T> {
    #[inline]
    fn default() -> Self {
        Map {
            id_gen: IdGen::new(),
            map: HashMap::new(),
        }
    }
}
impl<I: Id, T> Map<I, T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Borrow the resource associated with the given `Id`.
    pub fn get(&self, id: I) -> Option<&T> {
        self.map.get(&id)
    }
    /// Adds the given resource to the `Map` and returns a unique `Id` for it.
    pub fn insert(&mut self, resource: T) -> I {
        let id = self.id_gen.next_id();
        self.map.insert(id, resource);
        id
    }
}

pub struct Resources {
    pub font_loader: FontLoader,
    pub image_loader: ImageLoader,
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            font_loader: FontLoader::new(),
            image_loader: ImageLoader::new(),
        }
    }
}

impl Resources {
    /// Creates a new `Resources` struct, same as calling `default()`
    pub fn new() -> Self {
        Self::default()
    }

    fn set_render_api(&mut self, render: RenderApiSender) {
        self.font_loader.render = Some(render.create_api());
        self.image_loader.render = Some(render.create_api());
    }
}
