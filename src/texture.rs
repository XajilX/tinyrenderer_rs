use image::{ImageResult, RgbImage};

pub struct Texture {
    img: RgbImage,
}

impl Texture {
    #[allow(unused)]
    pub fn new(img: RgbImage) -> Self {
        Self { img }
    }

    pub fn open<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<std::path::Path>,
    {
        Ok(Self {
            img: image::open(path)?.into(),
        })
    }

    pub fn at_uv(&self, u: f32, v: f32) -> [u8; 3] {
        let (w, h) = self.img.dimensions();
        let (x, y) = (u * w as f32, v * h as f32);
        let (xf, yf) = (x.floor(), y.floor());
        let (x1, y1) = (xf.max(0.), yf.max(0.));
        let (x2, y2) = ((x1 + 1.).min(w as f32 - 1.), (y1 + 1.).min(h as f32 - 1.));
        let (s, t) = (x - xf, y - yf);
        let px1 = self.img.get_pixel(x1 as u32, y1 as u32).0;
        let px2 = self.img.get_pixel(x2 as u32, y1 as u32).0;
        let px3 = self.img.get_pixel(x1 as u32, y2 as u32).0;
        let px4 = self.img.get_pixel(x2 as u32, y2 as u32).0;
        let pxx1 = [
            px1[0] as f32 * (1. - s) + px2[0] as f32 * s,
            px1[1] as f32 * (1. - s) + px2[1] as f32 * s,
            px1[2] as f32 * (1. - s) + px2[2] as f32 * s,
        ];
        let pxx2 = [
            px3[0] as f32 * (1. - s) + px4[0] as f32 * s,
            px3[1] as f32 * (1. - s) + px4[1] as f32 * s,
            px3[2] as f32 * (1. - s) + px4[2] as f32 * s,
        ];
        [
            (pxx1[0] * (1. - t) + pxx2[0] * t) as u8,
            (pxx1[1] * (1. - t) + pxx2[1] * t) as u8,
            (pxx1[2] * (1. - t) + pxx2[2] * t) as u8,
        ]
    }
}
