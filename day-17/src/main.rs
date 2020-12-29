use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use  std::fs;

fn main() {
    let lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let num = part_one(&lines);
    println!("{}", num);

    let num = part_two(&lines);
    println!("{}", num);
}

fn part_one(lines: &[String]) -> usize {
    let mut active_volume = parse(lines);

    for _ in 0..6 {
        let mut counts : Volume<CountRow> = Default::default();

        for (z, active_plane) in active_volume.iter() {
            for (y, active_row) in active_plane.iter() {
                let rows = active_row
                    .iter()
                    .map(|x| vec![*x-1, *x, *x+1])
                    .flatten()
                    .collect::<Vec<_>>();

                // At z+1
                let count_plane = counts.plane_at(*z+1);
                count_plane.row_at(*y-1).increment_all(rows.iter().cloned());
                count_plane.row_at(*y).increment_all(rows.iter().cloned());
                count_plane.row_at(*y+1).increment_all(rows.iter().cloned());

                // At z
                let count_plane = counts.plane_at(*z);
                count_plane.row_at(*y-1).increment_all(rows.iter().cloned());
                count_plane.row_at(*y).increment_all(rows.iter().cloned());
                count_plane.row_at(*y+1).increment_all(rows.iter().cloned());
                count_plane.row_at(*y).decrement_all(active_row.iter().cloned());   // no double count

                // at z-1
                let count_plane = counts.plane_at(*z-1);
                count_plane.row_at(*y-1).increment_all(rows.iter().cloned());
                count_plane.row_at(*y).increment_all(rows.iter().cloned());
                count_plane.row_at(*y+1).increment_all(rows.iter().cloned());
            }
        }

        let mut next_volume : Volume<ActiveRow> = Default::default();

        // Activate when 3
        for (z, count_plane) in counts.iter() {
            for (y, count_row) in count_plane.iter() {
                next_volume.plane_at(*z).row_at(*y)
                    .activate_all(count_row.iter().filter_map(|(x, count)| {
                    if *count == 3 {
                        Some(*x)
                    } else {
                        None
                    }
                }));
            }
        }

        for (z, active_plane) in active_volume.iter() {
            let count_plane = counts.plane_at(*z);
            let next_plane = next_volume.plane_at(*z);

            for (y, active_row) in active_plane.iter() {
                let count_row = count_plane.row_at(*y);
                let next_row = next_plane.row_at(*y);

                next_row.activate_all(active_row.iter().filter(|x| {
                    let count = count_row.get(**x);
                    count == 2
                }).cloned());

                next_row.deactivate_all(active_row.iter().filter(|x| {
                    let count = count_row.get(**x);
                    count != 2 && count != 3
                }).cloned());
            }
        }

        active_volume = next_volume;
    }

    active_volume
        .iter()
        .map(|(_, plane)| plane.iter())
        .flatten()
        .map(|(_, row)| row.iter())
        .flatten()
        .count()
}

fn part_two(lines: &[String]) -> usize {
    let mut active_dimension: Dimension<ActiveRow> = Default::default();
    *active_dimension.volume_at(0) = parse(lines);

    for _ in 0..6 {
        let mut counts : Dimension<CountRow> = Default::default();

        for (w, active_volume) in active_dimension.iter() {
            for (z, active_plane) in active_volume.iter() {
                for (y, active_row) in active_plane.iter() {
                    let rows = active_row
                        .iter()
                        .map(|x| vec![*x-1, *x, *x+1])
                        .flatten()
                        .collect::<Vec<_>>();
    
                    for w in w-1..=w+1 {
                        let count_volume = counts.volume_at(w);

                        // At z+1
                        let count_plane = count_volume.plane_at(*z+1);
                        count_plane.row_at(*y-1).increment_all(rows.iter().cloned());
                        count_plane.row_at(*y).increment_all(rows.iter().cloned());
                        count_plane.row_at(*y+1).increment_all(rows.iter().cloned());
        
                        // At z
                        let count_plane = count_volume.plane_at(*z);
                        count_plane.row_at(*y-1).increment_all(rows.iter().cloned());
                        count_plane.row_at(*y).increment_all(rows.iter().cloned());
                        count_plane.row_at(*y+1).increment_all(rows.iter().cloned());
        
                        // at z-1
                        let count_plane = count_volume.plane_at(*z-1);
                        count_plane.row_at(*y-1).increment_all(rows.iter().cloned());
                        count_plane.row_at(*y).increment_all(rows.iter().cloned());
                        count_plane.row_at(*y+1).increment_all(rows.iter().cloned());
                    }

                    // No double count
                    counts.volume_at(*w).plane_at(*z).row_at(*y).decrement_all(active_row.iter().cloned());
                }
            }
        }
        
        let mut next_dimension : Dimension<ActiveRow> = Default::default();

        // Activate when 3
        for (w, count_volume) in counts.iter() {
            for (z, count_plane) in count_volume.iter() {
                for (y, count_row) in count_plane.iter() {
                    next_dimension.volume_at(*w).plane_at(*z).row_at(*y)
                        .activate_all(count_row.iter().filter_map(|(x, count)| {
                        if *count == 3 {
                            Some(*x)
                        } else {
                            None
                        }
                    }));
                }
            }
        }

        for (w, active_volume) in active_dimension.iter() {
            let count_volume = counts.volume_at(*w);
            let next_volume = next_dimension.volume_at(*w);

            for (z, active_plane) in active_volume.iter() {
                let count_plane = count_volume.plane_at(*z);
                let next_plane = next_volume.plane_at(*z);
    
                for (y, active_row) in active_plane.iter() {
                    let count_row = count_plane.row_at(*y);
                    let next_row = next_plane.row_at(*y);
    
                    next_row.activate_all(active_row.iter().filter(|x| {
                        let count = count_row.get(**x);
                        count == 2
                    }).cloned());
    
                    next_row.deactivate_all(active_row.iter().filter(|x| {
                        let count = count_row.get(**x);
                        count != 2 && count != 3
                    }).cloned());
                }
            }
        }
        
        active_dimension = next_dimension;
    }

    active_dimension
        .iter()
        .map(|(_, vol)| vol.iter())
        .flatten()
        .map(|(_, plane)| plane.iter())
        .flatten()
        .map(|(_, row)| row.iter())
        .flatten()
        .count()
}

fn parse(lines: &[String]) -> Volume<ActiveRow> {
    let mut volume : Volume<ActiveRow> = Default::default();
    let plane = volume.plane_at(0);
    let mut y = 0isize;

    for line in lines.iter().rev() {
        let row = plane.row_at(y);

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                row.activate(x as isize);
            }
        }

        y += 1;
    }

    volume
}

#[derive(Default)]
struct Dimension<T>(HashMap<isize, Volume<T>>) where T: Default;

impl<T> Dimension<T> where T: Default {
    fn volume_at(&mut self, w: isize) -> &mut Volume<T> {
        self.0.entry(w).or_default()
    }

    fn iter(&self) -> impl Iterator<Item=(&isize, &Volume<T>)> {
        self.0.iter()
    }
}

#[derive(Default)]
struct Volume<T>(HashMap<isize, Plane<T>>) where T: Default;

impl<T> Volume<T> where T: Default {
    fn plane_at(&mut self, z: isize) -> &mut Plane<T> {
        self.0.entry(z).or_default()
    }

    fn iter(&self) -> impl Iterator<Item=(&isize, &Plane<T>)> {
        self.0.iter()
    }
}

#[derive(Debug, Default)]
struct Plane<T>(HashMap<isize, T>) where T: Default;

impl<T> Plane<T> where T: Default {
    fn row_at(&mut self, y: isize) -> &mut T {
        self.0.entry(y).or_default()
    }

    fn iter(&self) -> impl Iterator<Item=(&isize, &T)> {
        self.0.iter()
    }
}

#[derive(Debug, Default)]
struct CountRow(HashMap<isize, u8>);

impl CountRow {
    fn iter(&self) -> impl Iterator<Item=(&isize, &u8)> {
        self.0.iter()
    }

    fn increment_all<'a>(&mut self, positions: impl Iterator<Item=isize>) {
        for i in positions {
            let curr = self.0.entry(i).or_default();
            *curr += 1;
        }
    }

    fn decrement_all(&mut self, positions: impl Iterator<Item=isize>) {
        for i in positions {
            let curr = self.0.entry(i).or_default();
            *curr -= 1;
        }
    }

    fn get(&self, x: isize) -> u8 {
        self.0.get(&x).unwrap_or(&0u8).to_owned()
    }
}

#[derive(Debug, Default)]
struct ActiveRow(HashSet<isize>);

impl ActiveRow {
    fn iter(&self) -> impl Iterator<Item=&isize> {
        self.0.iter()
    }

    fn activate(&mut self, x: isize) {
        self.0.insert(x);
    }

    fn activate_all(&mut self, positions: impl Iterator<Item=isize>) {
        self.0.extend(positions);
    }

    fn deactivate_all(&mut self, positions: impl Iterator<Item=isize>) {
        for x in positions {
            self.0.remove(&x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let num = part_two(&[
            ".#.".into(),
            "..#".into(),
            "###".into()
        ]);

        println!("{}", num);
    }

    #[test]
    fn test_part_one() {
        let num = part_one(&[
            ".#.".into(),
            "..#".into(),
            "###".into()
        ]);

        println!("{}", num);
    }
}

impl std::fmt::Debug for Volume<ActiveRow> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut z_list = self.0.keys().collect::<Vec<_>>();
        z_list.sort_unstable();

        for z in z_list {
            writeln!(f, "z={}", *z)?;
            
            let mut rows = self.0.get(z).unwrap().iter().collect::<Vec<_>>();
            rows.sort_unstable_by(|l, r| l.0.cmp(r.0));

            for (y, row) in rows.iter().rev() {
                let mut sorted_row = row.iter().collect::<Vec<_>>();

                if sorted_row.is_empty() {
                    continue;
                }

                sorted_row.sort_unstable();
                let sorted_row = sorted_row.iter().map(|x| format!("{},", **x)).collect::<String>();
                writeln!(f, "y={}: {}", y, sorted_row)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}