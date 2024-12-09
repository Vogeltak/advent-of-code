use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Clone)]
struct Block {
    id: usize,
    len: usize,
    kind: BlockKind,
}

#[derive(Clone)]
enum BlockKind {
    File,
    Free,
}

fn compact(disk: &mut Vec<Block>, fragment: bool) {
    let mut i = disk.len() - 1;
    while i > 0 {
        if matches!(disk[i].kind, BlockKind::Free) {
            i -= 1;
            continue;
        }

        let (id, mut len) = (disk[i].id, disk[i].len);

        while let Some(i_next_free) = disk[0..i].iter().position(|b| {
            matches!(b.kind, BlockKind::Free)
                    // If we don't allow fragmentation, we only want to find
                    // blocks of free space that are large enough to contain
                    // our file in its entirety.
                    && match fragment {
                        true => true,
                        false => b.len >= len,
                    }
        }) {
            let next_free = disk.get_mut(i_next_free).unwrap();

            // 1. file block fits exactly (easiest case), or
            // 2. file block exceeds continuous free space
            //    -> fill and adjust length
            // 3. file block is smaller than available free space
            //    -> change len of free space block we're using, and
            //       insert file block at its position

            match len.cmp(&next_free.len) {
                // (3)
                // We should never get to this option if we don't allow fragmentation,
                // because our find operation wouldn't return Some(Block) if it
                // doesn't have enough space to fit our file.
                Ordering::Less => {
                    next_free.len -= len;
                    disk.insert(
                        i_next_free,
                        Block {
                            id,
                            len,
                            kind: BlockKind::File,
                        },
                    );

                    // Update the index since we inserted a new block
                    i += 1;

                    disk[i].kind = BlockKind::Free;

                    // We've finished processing the file block, so
                    // we can move on to the next one
                    break;
                }
                // (1)
                Ordering::Equal => {
                    next_free.id = id;
                    next_free.kind = BlockKind::File;
                    disk[i].kind = BlockKind::Free;

                    // We've processed the complete file block,
                    // therefore we can move on to the next one
                    break;
                }
                // (2)
                Ordering::Greater => {
                    next_free.id = id;
                    next_free.kind = BlockKind::File;
                    len -= next_free.len;

                    // We're not yet finished with this file block
                }
            }
        }
        i -= 1;
    }
}

fn checksum(disk: &[Block]) -> usize {
    disk.iter()
        .fold((0, 0), |(mut sum, pos), b| {
            if matches!(b.kind, BlockKind::File) {
                sum += (pos..pos + b.len)
                    .map(|offset| offset * b.id)
                    .sum::<usize>();
            }
            (sum, pos + b.len)
        })
        .0
}

#[aoc::main(09)]
fn main(input: &str) -> (usize, usize) {
    let mut fid = 0;
    let mut disk = input
        .chars()
        .enumerate()
        .map(|(id, value)| {
            let len = value.to_digit(10).unwrap() as usize;
            match id % 2 {
                0 => {
                    fid += 1;
                    Block {
                        id: fid - 1,
                        len,
                        kind: BlockKind::File,
                    }
                }
                _ => Block {
                    id: 0,
                    len,
                    kind: BlockKind::Free,
                },
            }
        })
        .collect_vec();

    println!("there are {} entries on the disk", disk.len());
    println!(
        "-> {} of them are contiguous file blocks",
        disk.iter()
            .filter(|e| matches!(e.kind, BlockKind::File))
            .count()
    );
    println!(
        "-> {} of them are blocks of free space",
        disk.iter()
            .filter(|e| matches!(e.kind, BlockKind::Free))
            .count()
    );

    let mut disk2 = disk.clone();

    compact(&mut disk, true);
    let p1 = checksum(&disk);

    compact(&mut disk2, false);
    let p2 = checksum(&disk2);

    (p1, p2)
}
