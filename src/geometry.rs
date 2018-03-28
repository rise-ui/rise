use std::f32;

use euclid;
use rusttype;
use webrender::api::*;

pub type Point = euclid::TypedPoint2D<f32, LayerPixel>;
pub type Size = euclid::TypedSize2D<f32, LayerPixel>;
pub type Rect = euclid::TypedRect<f32, LayerPixel>;
