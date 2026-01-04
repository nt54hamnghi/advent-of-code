use std::cmp::Reverse;
use std::collections::BinaryHeap;

use itertools::Itertools;

const INPUT: &str = include_str!("../input/part2.txt");

#[allow(clippy::map_flatten)]
pub fn run() -> color_eyre::Result<()> {
    // let v = with_binary_heap(INPUT);
    // dbg!(v);

    let v = with_k_largest(INPUT);
    dbg!(v);

    Ok(())
}

pub fn with_k_largest(input: &str) -> u64 {
    let group_sums = input.lines().map(|n| n.parse::<u64>()).batching(|it| {
        let mut sum = 0;
        while let Some(Ok(n)) = it.next() {
            sum += n;
        }
        if sum == 0 { None } else { Some(sum) }
    });

    group_sums.k_largest(3).sum()
}

fn with_binary_heap(input: &str) -> u64 {
    let mut group_sums = input
        .lines()
        .map(|n| n.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .map(Reverse);

    // We use a BinaryHeap to maintain the sums in order: we push the first three,
    // and any time we push another one, we also pop from the heap.
    // That way, the heap never grows beyond 4 items
    let mut heap = (&mut group_sums).take(3).collect::<BinaryHeap<_>>();
    // group_sums.take(3) consume the iterator, but we'd like to reuse it later.
    // Since a mutable reference also implements Iterator, we can simply do (&mut group_sums).take(3).

    for i in group_sums {
        heap.push(i);
        // everytime we pop, it will always be the smallest item, thanks to cmp::Reverse
        // when the loop completes, we're left with 3 largest items
        heap.pop();
    }

    heap.iter().map(|v| v.0).sum()
}
