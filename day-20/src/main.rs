use std::{collections::{HashMap, HashSet, VecDeque}, fs, io::{self, BufRead}};

mod bitmap;
mod tile;
mod util;

use bitmap::*;
use tile::*;

fn main() {
    let lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten();

    let mut tiles = parse(lines);
    
    let num = part_one(&tiles);
    println!("{}", num);

    let num = part_two(&mut tiles);
    println!("{}", num);
}

fn parse(lines: impl Iterator<Item = String>) -> Vec<Tile> {
    let mut lines = lines.peekable();
    let mut tiles = vec![];

    let _ = lines.by_ref().take_while(|l| !l.starts_with("Tile "));

    while lines.peek().is_some() {
        let id = lines
            .next()
            .as_ref()
            .and_then(|s| s.strip_prefix("Tile "))
            .and_then(|s| s.strip_suffix(":"))
            .and_then(|s| s.parse::<usize>().map_or(None, |num| Some(num)))
            .unwrap();
        let image = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>();

        let top = image.first().unwrap().clone();
        let bottom = image.last().unwrap().clone();
        let (left, right) = image
            .iter()
            .map(|row| (row.chars().next().unwrap(), row.chars().last().unwrap()))
            .unzip();
        
        let mut edges = VecDeque::new();
        edges.push_back(top);
        edges.push_back(right);
        edges.push_back(bottom);
        edges.push_back(left);

        let pixels = image
            .iter()
            .skip(1)
            .take(image.len() - 2)
            .map(|row| {
                row.chars()
                    .skip(1)
                    .take(row.len() - 2)
                    .map(|c| c == '#')
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        tiles.push(Tile {
            id,
            edges,
            bitmap: Bitmap::new(pixels),
        });

        let _ = lines.by_ref().take_while(|l| !l.starts_with("Tile "));
    }

    tiles
}

#[allow(dead_code)]
fn part_one(tiles: &[Tile]) -> usize {
    find_corners(&create_edge_map(tiles))
        .iter()
        .product()
}

fn part_two(tiles: &mut [Tile]) -> usize {
    let image = stitch_images(tiles);
    let mut dragon = Bitmap::from_str_rows(&vec![
        "                  # ", 
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",  
    ]);

    let total_bit_count = image.0
        .iter()
        .flatten()
        .filter(|cell| **cell)
        .count();
    let dragon_bit_count = dragon.0
        .iter()
        .flatten()
        .filter(|c| **c)
        .count();

    let mut dragons = 0usize;
    'convolution: for _ in 0..2 {
        for _ in 0..4 {
            dragons = count_pattern(&image, &dragon, dragon_bit_count);
            if dragons > 0 {
                break 'convolution;
            }

            dragon.rotate_left();
        }

        dragon.flip_horizontal();
    }

    total_bit_count - dragons * dragon_bit_count
}

fn count_pattern(image: &Bitmap, pattern: &Bitmap, threshold: usize) -> usize {
    let pattern_width = pattern.width();
    let pattern_height = pattern.height();
    let mut count = 0;

    for y in 0..image.height() - pattern_height {
        for x in 0..image.width() - pattern_width {
            let found : usize = (0..pattern_height).map(|i| {
                pattern.0[i]
                    .iter()
                    .zip(image.0[y+i][x..x+pattern_width].iter())
                    .filter(|(l, r)| **l && **r)
                    .count()
            })
            .sum();

            if found >= threshold {
                count += 1;
            }
        }
    }

    count
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn create_edge_map(tiles: &[Tile]) -> HashMap<String, HashSet<usize>> {
    let mut edges = HashMap::<String, HashSet<usize>>::with_capacity(tiles.len()*8);
    
    for tile in tiles {
        for edge in tile.edges.iter() {
            let set = edges.entry((*edge).to_owned()).or_default();
            set.insert(tile.id);

            let reversed = reverse_string(edge);
            let set = edges.entry(reversed).or_default();
            set.insert(tile.id);
        }
    }

    edges
}

fn find_corners(edges: &HashMap<String, HashSet<usize>>) -> Vec<usize> {
    edges
        .iter()
        .filter_map(|(_, ids)| match ids.len() {
            1 => Some(ids.iter().next().unwrap()),
            _ => None,
        })
        .fold(HashMap::<usize, usize>::new(), |mut map, id| {
            let total = map.entry(*id).or_default();
            *total += 1;

            map
        })
        .iter()
        .filter_map(|(id, count)| match count {
            4 => Some(id),
            _ => None,
        })
        .cloned()
        .collect()
}

fn stitch_images(tiles: &mut [Tile]) -> Bitmap {
    let edges = create_edge_map(tiles);
    let mut tiles = tiles.iter_mut().map(|t| (t.id, t)).collect::<HashMap<_,_>>();

    let mut grid: Vec<Vec<&Tile>> = vec![Vec::new()];

    // Top row
    {
        let corner_id = find_corners(&edges).first().unwrap().clone();
        let top_left = tiles.remove(&corner_id).unwrap();

        while !is_aligned_top_left(top_left, &edges) {
            top_left.rotate_left();
        }

        grid[0].push(top_left);
        
        let mut prev_tile = grid[0][0];
        while let Some(next) = get_facing_tile_id(prev_tile.id, prev_tile.right_edge(), &edges) {
            let next_tile = tiles.remove(&next).unwrap();
            next_tile.orient_left(prev_tile.right_edge());
            grid[0].push(next_tile);

            prev_tile = next_tile;
        }
    }

    // The rest
    while tiles.len() != 0 {
        grid.push(
            grid
                .last()
                .unwrap()
                .iter()
                .fold(vec![], |mut row, tile| {
                    if let Some(next) = get_facing_tile_id(tile.id, tile.bottom_edge(), &edges) {
                        let next_tile = tiles.remove(&next).unwrap();
                        next_tile.orient_top(tile.bottom_edge());
                        row.push(next_tile);
                    } else {
                        panic!("Shouldn't happen!")
                    }

                    row
                })
        );
    }

    // Create Full bitmap
    let bitmap_list =  grid
        .iter()
        .map(|row| {
            Bitmap::from_row_bitmaps(row.iter().map(|tile| &tile.bitmap))
        })
        .collect::<Vec<_>>();

    Bitmap::from_column_bitmaps(bitmap_list.iter())
}

fn is_aligned_top_left(tile: &Tile, edges: &HashMap::<String, HashSet::<usize>>) -> bool {
    edges.get(tile.top_edge()).map_or(false, |edge| edge.len() == 1) && 
    edges.get(tile.left_edge()).map_or(false, |edge| edge.len() == 1)
}

fn get_facing_tile_id(tile_id: usize, edge: &str, edges: &HashMap::<String, HashSet::<usize>>) -> Option<usize> {
    match edges.get(edge) {
        Some(set) if set.len() == 2 => {
            let next = set.iter().filter(|id| **id != tile_id).next().unwrap();
            Some(*next)
        },
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_dragons() {
        let lines = io::BufReader::new(fs::File::open("./input2.txt").unwrap())
        .lines()
        .flatten();

        let mut tiles = parse(lines);
        assert_eq!(tiles.len(), 9);

        let num = part_two(&mut tiles);
        println!("{}", num);
    }
}