use std::{io, io::prelude::*};
use std::collections::HashSet;

struct Extrema {
    minx: i16,
    maxx: i16,
    miny: i16,
    maxy: i16,
    minz: i16,
    maxz: i16,
}

fn get_extrema(actives: &HashSet<(i16, i16, i16)>) -> Extrema {
    let mut extrema = Extrema {
        minx: i16::MAX, maxx: i16::MIN,
        miny: i16::MAX, maxy: i16::MIN,
        minz: i16::MAX, maxz: i16::MIN,
    };

    for (x, y, z) in actives {
        if x < &extrema.minx { extrema.minx = *x};
        if y < &extrema.miny { extrema.miny = *y};
        if z < &extrema.minz { extrema.minz = *z};
        if x > &extrema.maxx { extrema.maxx = *x};
        if y > &extrema.maxy { extrema.maxy = *y};
        if z > &extrema.maxz { extrema.maxz = *z};
    }

    return extrema;
}

fn count_active_neighbours(actives: &HashSet<(i16, i16, i16)>, x: i16, y: i16, z: i16) -> u8 {
    let mut result = 0;
    for i in x-1..=x+1 {
        for j in y-1..=y+1 {
            for k in z-1..=z+1 {
                if (i, j, k) == (x, y, z) {continue;}
                if actives.contains(&(i,j,k)) {
                    result += 1;
                }
            }
        }
    }
    return result;
}

fn step(actives: &HashSet<(i16, i16, i16)>) -> HashSet<(i16, i16, i16)> {
    let extrema = get_extrema(actives);
    let mut new_actives = HashSet::new();
    for x in extrema.minx-1..=extrema.maxx+1 {
        for y in extrema.miny-1..=extrema.maxy+1 {
            for z in extrema.minz-1..=extrema.maxz+1 {
                if actives.contains(&(x,y,z)) {
                    match count_active_neighbours(actives, x, y, z) {
                        2 | 3 => { new_actives.insert((x, y, z)); },
                        _ => {},
                    }
                } else {
                    if count_active_neighbours(actives, x, y, z) == 3 {
                        new_actives.insert((x, y, z));
                    }
                }
            }
        }
    }

    return new_actives;
}

fn main() {
    let stdin = io::stdin();

    let mut actives: HashSet<(i16, i16, i16)> = HashSet::new();

    for (x, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                actives.insert((x as i16, y as i16, 0));
            }
        }
    }

    for _ in 0..6 {
        actives = step(&actives);
    }

    println!("{}", actives.len());
}
