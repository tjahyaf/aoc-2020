use std::fmt::Formatter;

pub struct Bitmap(pub Vec<Vec<bool>>);

impl std::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            writeln!(f, "{}", row.iter().map(|c| if *c { '#' } else { '.' }).collect::<String>())?;
        }

        Ok(())
    }
}

impl Bitmap {
    pub fn new(bits: Vec<Vec<bool>>) -> Self {
        if let Some(len) = bits.first().and_then(|row| Some(row.len())) {
            assert!(bits.iter().skip(1).all(|row| row.len() == len));
        }

        Self(bits)
    }

    pub fn from_str_rows(strings: &[&str]) -> Self {
        Self::new(strings
            .iter()
            .map(|row| row.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>()
        )
    }

    pub fn from_row_bitmaps<'a>(bitmaps: impl Iterator<Item=&'a Bitmap>) -> Self {
        Self::new(bitmaps.fold(vec![], |mut rows, bitmap| {
            if rows.len() < bitmap.0.len() {
                rows.extend((0..bitmap.0.len()-rows.len()).map(|_| vec![]));
            }

            bitmap.0.iter().enumerate().for_each(|(i, row)| {
                if let Some(bitrow) = rows.get_mut(i) {
                    bitrow.extend(row);
                }
            });

            rows
        }))
    }

    pub fn from_column_bitmaps<'a>(bitmaps: impl Iterator<Item=&'a Bitmap>) -> Self {
        Self::new(bitmaps.map(|bitmap| bitmap.0.iter()).flatten().cloned().collect())
    }

    pub fn width(&self) -> usize {
        match self.0.first() {
            Some(row) => row.len(),
            _ => 0
        }
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn rotate_left(&mut self) {
        let length = self.0.first().unwrap().len();
        let mut rotated = vec![vec![]; length];
        
        for bits in self.0.iter() {
            for (i, value) in bits.iter().enumerate() {
                rotated[length - i - 1].push(value.to_owned());
            }
        }

        self.0 = rotated;
    }

    pub fn flip_horizontal(&mut self) {
        self.0.reverse();
    }

    pub fn flip_vertical(&mut self) {
        for row in self.0.iter_mut() {
            row.reverse();
        }
    }
}