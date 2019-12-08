use std::collections::HashMap;
use std::ops::Index;

pub struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    pub fn new(width: usize, height: usize, data: &[u8]) -> Self {
        Image {
            width,
            height,
            layers: data.chunks(width * height).map(|c| Layer::new(c)).collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Layer> {
        self.layers.iter()
    }

    pub fn image(&self) -> Vec<u8> {
        (0..(self.width * self.height))
            .flat_map(|i| {
                let x = self.pixel_at(i);
                if i % self.width == (self.width - 1) {
                    vec![x, b'\n']
                } else {
                    vec![x]
                }
            })
            .collect()
    }

    fn pixel_at(&self, i: usize) -> u8 {
        self.layers
            .iter()
            .map(|l| l[i])
            .filter(|&c| c != b'2')
            .nth(0)
            .map(|c| if c == b'0' { b' ' } else { c })
            .unwrap()
    }
}

pub struct Layer {
    contents: Vec<u8>,
}

impl Layer {
    fn new(data: &[u8]) -> Self {
        Layer {
            contents: data.to_vec(),
        }
    }

    pub fn summary(&self) -> HashMap<u8, usize> {
        let mut map = HashMap::new();
        for b in &self.contents {
            if let Some(count) = map.get_mut(b) {
                *count += 1;
            } else {
                map.insert(*b, 1);
            }
        }
        map
    }
}

impl Index<usize> for Layer {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        self.contents.index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::Image;

    #[test]
    fn layer() {
        let img = Image::new(3, 2, b"123456789012");
        let summary = img.iter().map(|l| l.summary()).collect::<Vec<_>>();
        assert_eq!(summary[0].get(&b'1'), Some(&1));
        assert_eq!(summary[0].get(&b'6'), Some(&1));
        assert_eq!(summary[0].get(&b'7'), None);

        assert_eq!(summary[1].get(&b'0'), Some(&1));
        assert_eq!(summary[1].get(&b'6'), None);
        assert_eq!(summary[1].get(&b'7'), Some(&1));
    }

    #[test]
    fn image() {
        let img = Image::new(2, 2, b"0222112222120000");
        assert_eq!(img.image(), b" 1\n1 \n");
    }
}
