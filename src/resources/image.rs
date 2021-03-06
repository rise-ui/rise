use std::collections::HashMap;
use std::path::PathBuf;

use failure::Error;

use image::{self, DynamicImage, GenericImage, ImageError};
use jss::webrender::api::{
  ExternalImageData,
  ExternalImageId,
  ImageDescriptor,
  ResourceUpdate,
  UpdateImage,
  ImageFormat,
  ImageData,
  DirtyRect,
  ImageKey,
  AddImage,
  RenderApi,
};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ImageSource {
    AbsolutePath(PathBuf),
    AssetPath(PathBuf),
    Bundled(String),
}

impl ImageSource {
    pub fn absolute<P: Into<PathBuf>>(path: P) -> Self {
        ImageSource::AbsolutePath(path.into())
    }
    pub fn asset<P: Into<PathBuf>>(path: P) -> Self {
        ImageSource::AssetPath(path.into())
    }
    pub fn bundled<P: Into<String>>(name: P) -> Self {
        ImageSource::Bundled(name.into())
    }
}

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub key: ImageKey,
    pub descriptor: ImageDescriptor,
}

#[derive(Debug, Fail)]
#[fail(display = "missing bundled image {}", name)]
struct BundledImageMissingError {
    name: String,
}

#[derive(Default)]
pub struct ImageLoader {
    pub render: Option<RenderApi>,
    pub assets_path: PathBuf,
    pub bundled_images: HashMap<ImageSource, ImageInfo>,
    pub images: HashMap<ImageSource, ImageInfo>,
    pub texture_descriptors: HashMap<u64, ImageDescriptor>,
}

impl ImageLoader {
    pub fn new() -> Self {
        ImageLoader::default()
    }

    pub fn get_image(&mut self, source: &ImageSource) -> Result<&ImageInfo, Error> {
        let image = self.get_image_internal(source);
        if let Err(ref error) = image {
            error!("Failed to load image from source {:?}. {}", source, error);
        }
        image
    }

    fn get_image_internal(&mut self, source: &ImageSource) -> Result<&ImageInfo, Error> {
        if self.images.contains_key(source) {
            Ok(&self.images[source])
        } else {
            let (data, descriptor) = match *source {
                ImageSource::AbsolutePath(ref path) => prepare_image(image::open(&path)?)?,
                ImageSource::AssetPath(ref relative_path) => {
                    let mut path = PathBuf::from(&self.assets_path);
                    path.push(relative_path);
                    prepare_image(image::open(&path)?)?
                }
                ImageSource::Bundled(ref name) => {
                    return Err(BundledImageMissingError {
                        name: name.to_owned(),
                    }
                    .into());
                }
            };

            Ok(self.put_image(source, data, descriptor))
        }
    }

    fn put_image(
        &mut self,
        source: &ImageSource,
        data: ImageData,
        descriptor: ImageDescriptor,
    ) -> &ImageInfo {
        let image_info = self.create_image_resource(data, descriptor);
        self.images.insert(source.clone(), image_info);
        &self.images[source]
    }

    pub fn create_image_resource(&mut self, data: ImageData, descriptor: ImageDescriptor) -> ImageInfo {
        let key = self.render_api().generate_image_key();
        let image = ResourceUpdate::AddImage(AddImage {
            tiling: None,
            descriptor,
            data,
            key,
        });

        self.render_api().update_resources(vec![ image ]);

        ImageInfo {
            key: key,
            descriptor: descriptor,
        }
    }

    pub fn update_texture(&mut self, key: ImageKey, descriptor: ImageDescriptor, data: ExternalImageData) {
        let image = ResourceUpdate::UpdateImage(UpdateImage {
            data: ImageData::External(data),
            dirty_rect: DirtyRect::All,
            descriptor,
            key,
        });

        self.render_api().update_resources(vec![ image ]);

        let ExternalImageData {
            id: ExternalImageId(texture_id),
            ..
        } = data;

        self.texture_descriptors.insert(texture_id, descriptor);
    }

    pub fn load_image(&mut self, name: &str, data: Vec<u8>) {
        if let Err(error) = self.load_image_internal(name, data) {
            error!("Failed to load image from raw data {}", error);
        }
    }

    fn load_image_internal(&mut self, name: &str, data: Vec<u8>) -> Result<(), Error> {
        let (data, descriptor) = prepare_image(image::load_from_memory(&data)?)?;
        let image_info = self.create_image_resource(data, descriptor);
        self.images.insert(ImageSource::bundled(name), image_info);
        Ok(())
    }

    fn render_api(&self) -> &RenderApi {
        self.render.as_ref().unwrap()
    }
}

fn prepare_image(image: DynamicImage) -> Result<(ImageData, ImageDescriptor), Error> {
    let (format, image_dims) = match &image {
        DynamicImage::ImageLuma8(buffer) => (ImageFormat::R8, buffer.dimensions()),
        DynamicImage::ImageRgba8(buffer) => (ImageFormat::BGRA8, buffer.dimensions()),
        _ => {
            return Err(ImageError::UnsupportedError("ImageFormat unsupported".to_string()).into());
        }
    };

    let mut bytes = image.raw_pixels();
    if format == ImageFormat::BGRA8 {
        premultiply(bytes.as_mut_slice());
    }

    let opaque = is_image_opaque(format, &bytes[..]);
    let descriptor = ImageDescriptor::new(image_dims.0 as i32, image_dims.1 as i32, format, opaque, true);
    let data = ImageData::new(bytes);
    Ok((data, descriptor))
}

fn is_image_opaque(format: ImageFormat, bytes: &[u8]) -> bool {
    match format {
        ImageFormat::BGRA8 => {
            let mut is_opaque = true;
            for i in 0..(bytes.len() / 4) {
                if bytes[i * 4 + 3] != 255 {
                    is_opaque = false;
                    break;
                }
            }
            is_opaque
        }
        ImageFormat::R8 => true,
        _ => unreachable!(),
    }
}

// From webrender/wrench
// These are slow. Gecko's gfx/2d/Swizzle.cpp has better versions
pub fn premultiply(data: &mut [u8]) {
    for pixel in data.chunks_mut(4) {
        let a = u32::from(pixel[3]);
        let r = u32::from(pixel[2]);
        let g = u32::from(pixel[1]);
        let b = u32::from(pixel[0]);

        pixel[3] = a as u8;
        pixel[2] = ((r * a + 128) / 255) as u8;
        pixel[1] = ((g * a + 128) / 255) as u8;
        pixel[0] = ((b * a + 128) / 255) as u8;
    }
}
