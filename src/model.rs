use crate::{
    linalg::{Vector2, Vector3},
    texture::Texture,
    triangle::Triangle,
};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    rc::Rc,
};

type TriInd = (usize, usize, usize);

pub struct Model {
    vertices: Vec<Vector3>,
    tex_coords: Vec<Vector2>,
    norms: Vec<Vector3>,
    tris: Vec<(TriInd, TriInd, TriInd)>,
    texture: Option<Rc<Texture>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            tex_coords: Vec::new(),
            norms: Vec::new(),
            tris: Vec::new(),
            texture: None,
        }
    }

    pub fn open<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let mut ret = Self::new();
        for line in reader.lines() {
            let tl = line?;
            let strs = tl.split_whitespace().collect::<Vec<&str>>();
            match strs[0] {
                "v" => {
                    let (x, y, z): (f32, f32, f32) =
                        (strs[1].parse()?, strs[2].parse()?, strs[3].parse()?);
                    if let Some(w) = strs.get(4) {
                        ret.vertices.push(Vector3::new(x, y, z) / w.parse::<f32>()?);
                    } else {
                        ret.vertices.push(Vector3::new(x, y, z));
                    }
                }
                "vt" => {
                    let (u, v): (f32, f32) = (
                        strs[1].parse()?,
                        if let Some(s) = strs.get(2) {
                            s.parse()?
                        } else {
                            0.
                        },
                    );
                    ret.tex_coords.push(Vector2::new(u, v));
                }
                "vn" => {
                    let (x, y, z): (f32, f32, f32) =
                        (strs[1].parse()?, strs[2].parse()?, strs[3].parse()?);
                    ret.vertices.push(Vector3::new(x, y, z));
                }
                "f" => {
                    let mut tris = [(0usize, 0usize, 0usize); 3];
                    for i in 0..3 {
                        let inds = strs[i + 1].split('/').collect::<Vec<&str>>();
                        tris[i].0 = inds[0].parse::<usize>()? - 1;
                        tris[i].1 = inds[1].parse::<usize>().map_or(usize::MAX, |u| u - 1);
                        tris[i].2 = inds[2].parse::<usize>().map_or(usize::MAX, |u| u - 1);
                    }
                }
                _ => {}
            }
        }
        Ok(ret)
    }

    pub fn load_texture<P>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        self.texture = Some(Rc::new(Texture::open(path)?));
        Ok(())
    }

    pub fn iter(&self) -> IterModel {
        IterModel { i: 0, model: self }
    }

    pub fn len_tri(&self) -> usize {
        self.tris.len()
    }

    pub fn get_tri(&self, i: usize) -> Triangle {
        let (v1, t1, n1) = self.tris[i].0;
        let (v2, t2, n2) = self.tris[i].1;
        let (v3, t3, n3) = self.tris[i].2;
        Triangle {
            v: [self.vertices[v1], self.vertices[v2], self.vertices[v3]],
            n: [
                if n1 != usize::MAX {
                    self.norms[n1]
                } else {
                    Vector3::new(0., 0., 0.)
                },
                if n2 != usize::MAX {
                    self.norms[n2]
                } else {
                    Vector3::new(0., 0., 0.)
                },
                if n3 != usize::MAX {
                    self.norms[n3]
                } else {
                    Vector3::new(0., 0., 0.)
                },
            ],
            uv: [
                if t1 != usize::MAX {
                    self.tex_coords[t1]
                } else {
                    Vector2::new(0., 0.)
                },
                if t2 != usize::MAX {
                    self.tex_coords[t2]
                } else {
                    Vector2::new(0., 0.)
                },
                if t3 != usize::MAX {
                    self.tex_coords[t3]
                } else {
                    Vector2::new(0., 0.)
                },
            ],
            texture: self.texture.clone(),
        }
    }
}

pub struct IterModel<'a> {
    i: usize,
    model: &'a Model,
}

impl<'a> Iterator for IterModel<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.model.tris.len() {
            None
        } else {
            let ret = self.model.get_tri(self.i);
            self.i += 1;
            Some(ret)
        }
    }
}
