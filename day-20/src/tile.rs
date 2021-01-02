use std::{collections::VecDeque, fmt::Formatter};

use crate::{bitmap::Bitmap, util::reverse_string};

pub struct Tile {
    pub id: usize,
    pub edges: VecDeque<String>,
    pub bitmap: Bitmap,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        writeln!(f, "\tedges: {:?}", self.edges)
    }
}

static EDGE_TOP: usize = 0;
static EDGE_RIGHT: usize = 1;
static EDGE_BOTTOM: usize = 2;
static EDGE_LEFT: usize = 3;

impl Tile {
    pub fn flip_vertical(&mut self) {
        self.edges.swap(EDGE_RIGHT, EDGE_LEFT);
        self.edges[EDGE_TOP] = reverse_string(&self.edges[EDGE_TOP]);
        self.edges[EDGE_BOTTOM] = reverse_string(&self.edges[EDGE_BOTTOM]);
        self.bitmap.flip_vertical();
    }

    pub fn flip_horizontal(&mut self) {
        self.edges.swap(EDGE_TOP, EDGE_BOTTOM);
        self.edges[EDGE_LEFT] = reverse_string(&self.edges[EDGE_LEFT]);
        self.edges[EDGE_RIGHT] = reverse_string(&self.edges[EDGE_RIGHT]);
        self.bitmap.flip_horizontal();
    }

    pub fn rotate_left(&mut self) {
        self.edges.rotate_left(1);
        self.edges[EDGE_LEFT] = reverse_string(&self.edges[EDGE_LEFT]);
        self.edges[EDGE_RIGHT] = reverse_string(&self.edges[EDGE_RIGHT]);
        self.bitmap.rotate_left();
    }

    pub fn orient_left(&mut self, edge: &str) {
        let reversed = reverse_string(&edge);
        for _ in 0..4 {
            if self.edges[EDGE_LEFT] == reversed {
                self.flip_horizontal();
            }
            if self.edges[EDGE_LEFT] == edge {
                break;
            }
            self.rotate_left();
        }
    }

    pub fn orient_top(&mut self, edge: &str) {
        let reversed = reverse_string(&edge);
        for _ in 0..4 {
            if self.edges[EDGE_TOP] == reversed {
                self.flip_vertical();
            }
            if self.edges[EDGE_TOP] == edge {
                break;
            }
            self.rotate_left();
        }
    }

    pub fn top_edge(&self) -> &str {
        self.edges[EDGE_TOP].as_ref()
    }

    pub fn left_edge(&self) -> &str {
        self.edges[EDGE_LEFT].as_ref()
    }

    pub fn bottom_edge(&self) -> &str {
        self.edges[EDGE_BOTTOM].as_ref()
    }

    pub fn right_edge(&self) -> &str {
        self.edges[EDGE_RIGHT].as_ref()
    }
}