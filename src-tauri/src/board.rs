use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::ops::Not;
use std::ops::{Index, IndexMut};
use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum FieldState {
    Blank = 0,
    Cross = 1,
    Circle = -1,
}

impl Not for FieldState {
    type Output = FieldState;

    fn not(self) -> Self::Output {
        match self {
            FieldState::Circle => FieldState::Cross,
            FieldState::Cross => FieldState::Circle,
            FieldState::Blank => self,
        }
    }
}

impl fmt::Display for FieldState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FieldState::Blank => "_ ",
                FieldState::Cross => "X ",
                FieldState::Circle => "O ",
            }
        )
    }
}

pub trait InternalArray<T> {
    fn internal_array(&self) -> &[T];
    fn internal_array_mut(&mut self) -> &mut [T];
    fn size(&self) -> usize;
    fn ind(&self, i: usize, j: usize) -> &T {
        &self.internal_array()[i * self.size() + j]
    }
    fn ind_mut(&mut self, i: usize, j: usize) -> &mut T {
        let size = self.size();
        &mut self.internal_array_mut()[i * size + j]
    }
}

pub trait GridDisplay<T: fmt::Display>: InternalArray<T> {
    fn display(&self) {
        for i in 0..self.size() {
            for j in 0..self.size() {
                print!("{} ", self.ind(i, j));
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WeightMap {
    array: FlatArray<usize>,
    size: usize,
}

impl InternalArray<usize> for WeightMap {
    fn internal_array(&self) -> &[usize] {
        &self.array.0
    }

    fn internal_array_mut(&mut self) -> &mut [usize] {
        &mut self.array.0
    }

    fn size(&self) -> usize {
        self.size
    }
}   

impl GridDisplay<usize> for WeightMap {}

impl WeightMap {
    pub fn new(size: usize) -> WeightMap {
        let mut w = WeightMap {
            array: FlatArray([0; 100]),
            size,
        };
        w.generate();
        w
    }

    pub fn clear(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                *self.ind_mut(i, j) = 0;
            }
        }
    }

    pub fn generate(&mut self) {
        if self.size < 6 {
            for i in 0..self.size {
                for j in 0..self.size {
                    *self.ind_mut(i, j) = 2;
                }
            }

            for i in 0..self.size {
                *self.ind_mut(i, i) += 1;
                *self.ind_mut(i, self.size - i - 1) += 1;
            }
        } else {
            for sx in 0..self.size - 4 {
                for sy in 0..self.size - 4 {
                    for i in 0..5 {
                        for j in 0..5 {
                            *self.ind_mut(i + sx, j + sy) += 1;
                        }
                    }

                    for i in 0..5 {
                        for j in 0..5 {
                            *self.ind_mut(j + sx, i + sy) += 1;
                        }
                    }

                    for i in 0..5 {
                        *self.ind_mut(i + sx, i + sy) += 1;
                        *self.ind_mut(i + sx, self.size - i - 1 - sy) += 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlatArray<T>([T; 100]);
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct BoardArray {
    pub array: FlatArray<FieldState>,
    pub size: usize,
}

impl InternalArray<FieldState> for BoardArray {
    fn internal_array(&self) -> &[FieldState] {
        &self.array.0
    }

    fn internal_array_mut(&mut self) -> &mut [FieldState] {
        &mut self.array.0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl GridDisplay<FieldState> for BoardArray {}

impl Index<(usize, usize)> for BoardArray {
    type Output = FieldState;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.array.0[index.0 * self.size + index.1]
    }
}

impl IndexMut<(usize, usize)> for BoardArray {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.array.0[index.0 * self.size + index.1]
    }
}

impl BoardArray {
    pub unsafe fn get_unchecked(&self, index: (usize, usize)) -> &FieldState {
        self.array.0.get_unchecked(index.0 * self.size + index.1)
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: (usize, usize)) -> &mut FieldState {
        self.array
            .0
            .get_unchecked_mut(index.0 * self.size + index.1)
    }

    pub fn new(size: usize) -> BoardArray {
        BoardArray {
            array: FlatArray([FieldState::Blank; 100]),
            size,
        }
    }

    pub fn rotate_right(&self) -> Self {
        let mut new = *self;

        for i in 0..self.size {
            for j in 0..self.size {
                unsafe {
                    *new.get_unchecked_mut((i, j)) = *self.get_unchecked((self.size - j - 1, i));
                }
            }
        }
        new
    }

    pub fn flip_horizontal(&self) -> Self {
        let mut new = *self;

        for i in 0..self.size {
            for j in 0..self.size {
                unsafe {
                    *new.get_unchecked_mut((i, j)) = *self.get_unchecked((self.size - i - 1, j));
                }
            }
        }
        new
    }

    pub fn flip_vertical(&self) -> Self {
        let mut new = *self;

        for i in 0..self.size {
            for j in 0..self.size {
                unsafe {
                    *new.get_unchecked_mut((i, j)) = *self.get_unchecked((i, self.size - j - 1));
                }
            }
        }
        new
    }

    pub fn get_symmetries(&self) -> [BoardArray; 6] {
        let mut symmetries = [BoardArray::new(self.size); 6];
        symmetries[0] = *self;

        for i in 0..3 {
            symmetries[i + 1] = symmetries[i].rotate_right();
        }

        symmetries[4] = self.flip_horizontal();
        symmetries[5] = self.flip_vertical();

        symmetries
    }

    pub fn jsonable(&self) -> [[i8; 10]; 10] {
        let mut matrix = [[0i8; 10]; 10];

        for i in 0..self.size {
            for j in 0..self.size {
                matrix[i][j] = match *self.ind(i, j) {
                    FieldState::Blank => 0,
                    FieldState::Circle =>  1,
                    FieldState::Cross => -1,
                };
            }
        }

        matrix
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Board {
    pub array: BoardArray,
    pub current_player: FieldState,
    pub weight_map: WeightMap,
}

#[derive(Debug, Clone, Copy)]
pub struct Block(usize, (usize, usize));

#[derive(Debug, Clone, Copy)]
pub struct WeightedBlock {
    block: Block,
    weight: usize,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            array: BoardArray::new(size),
            current_player: FieldState::Cross,
            weight_map: WeightMap::new(size),
        }
    }

    pub fn _display(&self) {
        self.array.display();
    }

    pub fn blank_fields(&self) -> Vec<(usize, usize)> {
        let mut empty = vec![];
        for x in 0..self.array.size {
            for y in 0..self.array.size {
                if self.array[(x, y)] == FieldState::Blank {
                    empty.push((x, y))
                }
            }
        }
        empty
    }

    pub fn ordered_moves(&self) -> Vec<(usize, usize)> {
        let mut blank = self.blank_fields();
        blank.sort_by(|a, b| {
            self.weight_map
                .ind(b.0, b.1)
                .partial_cmp(self.weight_map.ind(a.0, a.1))
                .unwrap()
        });
        blank
    }

    // Positive values for Cross, negative values for Circle
    pub fn evaluate(&self) -> i32 {
        let condition = if self.array.size < 5 {
            self.array.size
        } else {
            5
        };

        for x in 0..self.array.size {
            let mut cross_count = 0;
            let mut circle_count = 0;

            for y in 0..self.array.size {
                let cell = unsafe { self.array.get_unchecked((x, y)) };
                match *cell {
                    FieldState::Cross => {
                        cross_count += 1;
                        circle_count = 0;
                    }
                    FieldState::Circle => {
                        circle_count += 1;
                        cross_count = 0;
                    }
                    _ => {
                        cross_count = 0;
                        circle_count = 0;
                    }
                }

                if cross_count >= condition {
                    return 1000;
                } else if circle_count >= condition {
                    return -1000;
                }
            }
        }

        for x in 0..self.array.size {
            let mut cross_count = 0;
            let mut circle_count = 0;

            for y in 0..self.array.size {
                let cell = unsafe { self.array.get_unchecked((y, x)) };
                match *cell {
                    FieldState::Cross => {
                        cross_count += 1;
                        circle_count = 0;
                    }
                    FieldState::Circle => {
                        circle_count += 1;
                        cross_count = 0;
                    }
                    _ => {
                        cross_count = 0;
                        circle_count = 0;
                    }
                }

                if cross_count >= condition {
                    return 1000;
                } else if circle_count >= condition {
                    return -1000;
                }
            }
        }

        if self.array.size < 6 {
            let mut cross_count = 0;
            let mut circle_count = 0;

            for x in 0..self.array.size {
                let cell = unsafe { self.array.get_unchecked((x, x)) };
                match *cell {
                    FieldState::Cross => {
                        cross_count += 1;
                        circle_count = 0;
                    }
                    FieldState::Circle => {
                        circle_count += 1;
                        cross_count = 0;
                    }
                    _ => {
                        cross_count = 0;
                        circle_count = 0;
                    }
                }

                if cross_count >= condition {
                    return 1000;
                } else if circle_count >= condition {
                    return -1000;
                }
            }
        } else {
            for sx in 0..=self.array.size - 5 {
                let mut cross_count = 0;
                let mut circle_count = 0;

                for x in 0..self.array.size - sx {
                    let cell = unsafe { self.array.get_unchecked((x + sx, x)) };
                    match *cell {
                        FieldState::Cross => {
                            cross_count += 1;
                            circle_count = 0;
                        }
                        FieldState::Circle => {
                            circle_count += 1;
                            cross_count = 0;
                        }
                        _ => {
                            cross_count = 0;
                            circle_count = 0;
                        }
                    }

                    if cross_count >= condition {
                        return 1000;
                    } else if circle_count >= condition {
                        return -1000;
                    }
                }
            }

            for sy in 1..=self.array.size - 5 {
                let mut cross_count = 0;
                let mut circle_count = 0;

                for x in 0..self.array.size - sy {
                    let cell = unsafe { self.array.get_unchecked((x, x + sy)) };
                    match *cell {
                        FieldState::Cross => {
                            cross_count += 1;
                            circle_count = 0;
                        }
                        FieldState::Circle => {
                            circle_count += 1;
                            cross_count = 0;
                        }
                        _ => {
                            cross_count = 0;
                            circle_count = 0;
                        }
                    }

                    if cross_count >= condition {
                        return 1000;
                    } else if circle_count >= condition {
                        return -1000;
                    }
                }
            }
        }

        if self.array.size < 6 {
            let mut cross_count = 0;
            let mut circle_count = 0;

            for x in 0..self.array.size {
                let cell = unsafe { self.array.get_unchecked((x, self.array.size - x - 1)) };
                match *cell {
                    FieldState::Cross => {
                        cross_count += 1;
                        circle_count = 0;
                    }
                    FieldState::Circle => {
                        circle_count += 1;
                        cross_count = 0;
                    }
                    _ => {
                        cross_count = 0;
                        circle_count = 0;
                    }
                }

                if cross_count >= condition {
                    return 1000;
                } else if circle_count >= condition {
                    return -1000;
                }
            }
        } else {
            for sx in 0..=self.array.size - 5 {
                let mut cross_count = 0;
                let mut circle_count = 0;

                for x in 0..self.array.size - sx {
                    let cell =
                        unsafe { self.array.get_unchecked((self.array.size - x - 1, x + sx)) };
                    match *cell {
                        FieldState::Cross => {
                            cross_count += 1;
                            circle_count = 0;
                        }
                        FieldState::Circle => {
                            circle_count += 1;
                            cross_count = 0;
                        }
                        _ => {
                            cross_count = 0;
                            circle_count = 0;
                        }
                    }

                    if cross_count >= condition {
                        return 1000;
                    } else if circle_count >= condition {
                        return -1000;
                    }
                }
            }

            for sy in 1..=self.array.size - 5 {
                let mut cross_count = 0;
                let mut circle_count = 0;

                for x in 0..self.array.size - sy {
                    let cell =
                        unsafe { self.array.get_unchecked((self.array.size - x - 1 - sy, x)) };
                    match *cell {
                        FieldState::Cross => {
                            cross_count += 1;
                            circle_count = 0;
                        }
                        FieldState::Circle => {
                            circle_count += 1;
                            cross_count = 0;
                        }
                        _ => {
                            cross_count = 0;
                            circle_count = 0;
                        }
                    }

                    if cross_count >= condition {
                        return 1000;
                    } else if circle_count >= condition {
                        return -1000;
                    }
                }
            }
        }

        let mut combined_score = 0;

        for i in 0..self.array.size {
            for j in 0..self.array.size {
                match self.array[(i, j)] {
                    FieldState::Cross => combined_score += *self.weight_map.ind(i, j) as i32,
                    FieldState::Circle => combined_score -= *self.weight_map.ind(i, j) as i32,
                    FieldState::Blank => continue,
                }
            }
        }

        combined_score
    }

    pub fn heuristic_blocks(&self, sign: FieldState) -> Vec<Block> {
        let mut blocks: Vec<Block> = vec![];

        for x in 0..self.array.size {
            let mut count = 0;

            for y in 0..self.array.size {
                let cell = unsafe { self.array.get_unchecked((x, y)) };

                if *cell == sign {
                    count += 1;
                } else {
                    let mut local_blocks = vec![];

                    if *unsafe { self.array.get_unchecked((x, y)) } == FieldState::Blank {
                        local_blocks.push(Block(count, (x, y)));
                    }

                    count += 1;
                    if y as isize - count as isize >= 0
                        && *unsafe { self.array.get_unchecked((x, y - count)) } == FieldState::Blank
                    {
                        local_blocks.push(Block(count - 1, (x, y - count)));
                    }

                    if !(count - 1 == 3 && local_blocks.len() == 1) && count - 1 > 2 {
                        blocks.extend(local_blocks);
                    }

                    count = 0;
                }
            }
        }

        for x in 0..self.array.size {
            let mut count = 0;

            for y in 0..self.array.size {
                let cell = unsafe { self.array.get_unchecked((y, x)) };

                if *cell == sign {
                    count += 1;
                } else {
                    let mut local_blocks = vec![];

                    if *unsafe { self.array.get_unchecked((y, x)) } == FieldState::Blank {
                        local_blocks.push(Block(count, (y, x)));
                    }

                    count += 1;

                    if y as isize - count as isize >= 0
                        && *unsafe { self.array.get_unchecked((y - count, x)) } == FieldState::Blank
                    {
                        local_blocks.push(Block(count, (y - count, x)));
                    }

                    if !(count - 1 == 3 && local_blocks.len() == 1) && count - 1 > 2 {
                        blocks.extend(local_blocks);
                    }

                    count = 0;
                }
            }
        }

        if self.array.size < 6 {
            let mut count = 0;

            for x in 0..self.array.size {
                let cell = unsafe { self.array.get_unchecked((x, x)) };

                if *cell == sign {
                    count += 1;
                } else {
                    let mut local_blocks = vec![];

                    if *unsafe { self.array.get_unchecked((x, x)) } == FieldState::Blank {
                        local_blocks.push(Block(count, (x, x)));
                    }

                    count += 1;

                    if x as isize - count as isize >= 0
                        && *unsafe { self.array.get_unchecked((x - count, x - count)) }
                            == FieldState::Blank
                    {
                        local_blocks.push(Block(count, (x - count, x - count)));
                    }

                    if !(count - 1 == 3 && local_blocks.len() == 1) && count - 1 > 2 {
                        blocks.extend(local_blocks);
                    }

                    count = 0;
                }
            }
        } else {
            for sx in 0..=self.array.size - 5 {
                let mut count = 0;

                for x in 0..self.array.size - sx {
                    let cell = unsafe { self.array.get_unchecked((x + sx, x)) };
                    if *cell == sign {
                        count += 1;
                    } else {
                        let mut local_blocks = vec![];

                        if *unsafe { self.array.get_unchecked((x + sx, x)) } == FieldState::Blank {
                            local_blocks.push(Block(count, (x + sx, x)));
                        }

                        count += 1;

                        if x as isize - count as isize >= 0
                            && *unsafe { self.array.get_unchecked((x + sx - count, x - count)) }
                                == FieldState::Blank
                        {
                            local_blocks.push(Block(count, (x + sx - count, x - count)));
                        }

                        if !(count - 1 == 3 && local_blocks.len() == 1) && count - 1 > 2 {
                            blocks.extend(local_blocks);
                        }

                        count = 0;
                    }
                }
            }

            for sy in 1..=self.array.size - 5 {
                let mut count = 0;

                for x in 0..self.array.size - sy {
                    let cell = unsafe { self.array.get_unchecked((x, x + sy)) };

                    if *cell == sign {
                        count += 1;
                    } else {
                        let mut local_blocks = vec![];

                        if *unsafe { self.array.get_unchecked((x, x + sy)) } == FieldState::Blank {
                            local_blocks.push(Block(count, (x, x + sy)));
                        }

                        count += 1;

                        if x as isize - count as isize >= 0
                            && *unsafe { self.array.get_unchecked((x - count, x + sy - count)) }
                                == FieldState::Blank
                        {
                            local_blocks.push(Block(count, (x - count, x + sy - count)));
                        }

                        if !(count - 1 == 3 && local_blocks.len() == 1) && count - 1 > 2 {
                            blocks.extend(local_blocks);
                        }

                        count = 0;
                    }
                }
            }
        }

        if self.array.size < 6 {
            let mut count = 0;

            for x in 0..self.array.size {
                let cell = unsafe { self.array.get_unchecked((x, self.array.size - x - 1)) };

                if *cell == sign {
                    count += 1;
                } else {
                    let mut local_blocks = vec![];

                    if *unsafe { self.array.get_unchecked((x, self.array.size - x - 1)) }
                        == FieldState::Blank
                    {
                        local_blocks.push(Block(count, (x, self.array.size - x - 1)));
                    }

                    count += 1;

                    if x as isize - count as isize >= 0
                        && self.array.size as isize - x as isize - 1 + count as isize >= 0
                        && x as isize - count as isize >= 0
                        && *unsafe {
                            self.array
                                .get_unchecked((x - count, self.array.size - x - 1 + count))
                        } == FieldState::Blank
                    {
                        local_blocks
                            .push(Block(count, (x - count, self.array.size - x - 1 + count)));
                    }

                    if !(count - 1 == 3 && local_blocks.len() == 1) && count - 1 > 2 {
                        blocks.extend(local_blocks);
                    }

                    count = 0;
                }
            }
        } else {
            for sx in 0..=self.array.size - 5 {
                let mut count = 0;

                for x in 0..self.array.size - sx {
                    let cell =
                        unsafe { self.array.get_unchecked((self.array.size - x - 1, x + sx)) };
                    if *cell == sign {
                        count += 1;
                    } else {
                        let mut local_blocks = vec![];

                        if *unsafe { self.array.get_unchecked((self.array.size - x - 1, x + sx)) }
                            == FieldState::Blank
                        {
                            local_blocks.push(Block(count, (self.array.size - x - 1, x + sx)));
                        }

                        count += 1;

                        if self.array.size as isize - x as isize - 1 + count as isize >= 0
                            && x as isize + sx as isize - count as isize >= 0
                            && *unsafe {
                                self.array.get_unchecked((
                                    self.array.size - x - 1 + count,
                                    x + sx - count,
                                ))
                            } == FieldState::Blank
                        {
                            local_blocks.push(Block(
                                count,
                                (self.array.size - x - 1 + count, x + sx - count),
                            ));
                        }

                        if !(count - 1 == 3 && local_blocks.len() == 1) && count - 1 > 2 {
                            blocks.extend(local_blocks);
                        }

                        count = 0;
                    }
                }
            }

            for sy in 1..=self.array.size - 5 {
                let mut count = 0;

                for x in 0..self.array.size - sy {
                    let cell =
                        unsafe { self.array.get_unchecked((self.array.size - x - 1 - sy, x)) };
                    if *cell == sign {
                        count += 1;
                    } else {
                        let mut local_blocks = vec![];

                        if *unsafe { self.array.get_unchecked((self.array.size - x - 1 - sy, x)) }
                            == FieldState::Blank
                        {
                            local_blocks.push(Block(count, (self.array.size - x - 1 - sy, x)));
                        }

                        count += 1;

                        if self.array.size as isize - x as isize - 1 - sy as isize + count as isize
                            >= 0
                            && x as isize - count as isize >= 0
                            && *unsafe {
                                self.array.get_unchecked((
                                    self.array.size - x - 1 - sy + count,
                                    x - count,
                                ))
                            } == FieldState::Blank
                        {
                            local_blocks.push(Block(
                                count,
                                (self.array.size - x - 1 - sy + count, x - count),
                            ));
                        }

                        if !(count - 1 == 3 && local_blocks.len() == 1) && count - 1 > 2 {
                            blocks.extend(local_blocks);
                        }

                        count = 0;
                    }
                }
            }
        }

        blocks
    }

    pub fn weighted_blocks(&self) -> Vec<WeightedBlock> {
        let blocks = self.heuristic_blocks(FieldState::Cross);

        let mut weighted = vec![];

        for i in blocks {
            weighted.push(WeightedBlock {
                block: i,
                weight: *self.weight_map.ind(i.1 .0, i.1 .1),
            });
        }

        weighted
    }

    pub fn best_block(&self) -> Option<(usize, usize)> {
        let weighted = self.weighted_blocks();

        if let Some(max) = weighted.iter().max_by(|&a, &b| a.block.0.cmp(&b.block.0)) {
            let best_by_strikes = weighted
                .iter()
                .filter(|x| x.block.0 == max.block.0)
                .max_by(|a, b| a.weight.cmp(&b.weight))
                .unwrap()
                .block
                .1;

            Some(best_by_strikes)
        } else {
            None
        }
    }

    pub fn reweigh(&mut self) {
        self.weight_map.clear();

        self.weight_map.generate();

        let blocks = self.heuristic_blocks(FieldState::Circle);

        for block in blocks {
            *self.weight_map.ind_mut(block.1 .0, block.1 .1) += 5 * block.0;
        }
    }

    pub fn winner_sign(&self) -> Option<FieldState> {
        let eval = self.evaluate();

        if self.blank_fields().is_empty() && eval.abs() != 1000 {
            Some(FieldState::Blank)
        } else {
            match eval {
                1000 => Some(FieldState::Cross),
                -1000 => Some(FieldState::Circle),
                _ => None,
            }
        }
    }

    pub fn switch_player(&mut self) {
        self.current_player = !self.current_player;
    }

    pub fn make_move(&mut self, coords: (usize, usize)) -> Result<(), ()> {
        if self.array[coords] == FieldState::Blank
            && coords.0 < self.array.size
            && coords.1 < self.array.size
        {
            self.array[coords] = self.current_player;
            self.switch_player();
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn make_move_unchecked(&mut self, coords: (usize, usize)) {
        unsafe {
            let cell = self.array.get_unchecked_mut(coords);
            *cell = self.current_player;
        }
        self.switch_player();
    }
}

pub fn minimax(
    board: Board,
    alpha: i32,
    beta: i32,
    depth: u8,
    //memo: &HashMap<BoardArray, i32>,
    //saver: Sender<(BoardArray, i32)>
) -> i32 {
    let mut passed_alpha = alpha;
    let mut passed_beta = beta;

    let eval = board.evaluate();

    if depth == 0 || board.blank_fields().is_empty() || eval.abs() == 1000 {
        eval
    } 
    
    else if board.current_player == FieldState::Cross {
        let mut max = -10000;
        for i in board.ordered_moves() {
            let mut passed = board;
            passed.make_move_unchecked(i);
            let mut position = 0;

            let mut found = false;
            for sym in &passed.array.get_symmetries() {
                let memo = MEMO.read().unwrap();
                if memo.contains_key(&(*sym, depth)) {
                    position = memo[&(*sym, depth)];
                    found = true;
                    break;
                }
            }

            if !found {
                position = minimax(passed, alpha, beta, depth - 1);
                MEMO.write()
                    .unwrap()
                    .insert((passed.array, depth), position);
            }

            if position > max {
                max = position;
            }
            if passed_alpha < position {
                passed_alpha = position;
            }
            if passed_beta <= passed_alpha {
                break;
            }
        }
        max
    } else {
        let mut min = 10000;
        for i in board.blank_fields() {
            let mut passed = board;
            passed.make_move_unchecked(i);
            let mut position = 0;

            let mut found = false;

            for sym in &passed.array.get_symmetries() {
                let memo = MEMO.read().unwrap();
                if memo.contains_key(&(*sym, depth)) {
                    position = memo[&(*sym, depth)];
                    found = true;
                    break;
                }
            }

            if !found {
                position = minimax(passed, alpha, beta, depth - 1);
                MEMO.write()
                    .unwrap()
                    .insert((passed.array, depth), position);
            }

            if position < min {
                min = position;
            }
            if passed_beta > position {
                passed_beta = position;
            }
            if passed_beta <= passed_alpha {
                break;
            }
        }
        min
    }
}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MEMO: RwLock<HashMap<(BoardArray, u8), i32>> = RwLock::new(HashMap::new());
}

#[derive(Debug)]
struct ComputedBest {
    min: i32,
    x: usize,
    y: usize,
}

pub fn computer_move(board: &mut Board, general_depth: u8) -> (usize, usize) {
    board.reweigh();

    // Immediate win heuristic
    for i in board.blank_fields() {
        let mut new = *board;
        new.make_move_unchecked(i);

        if new.evaluate() == -1000 {
            return i;
        }
    }

    // Immediate loss heuristic
    for i in board.blank_fields() {
        let mut new = *board;
        new.current_player = FieldState::Cross;
        new.make_move_unchecked(i);

        if new.evaluate() == 1000 {
            return i;
        }
    }

    // Basic fork heuristic
    for i in board.blank_fields() {
        let mut level_one = *board;
        level_one.current_player = FieldState::Cross;
        level_one.make_move_unchecked(i);

        let mut outer_wins = 0;

        for j in level_one.blank_fields() {
            let mut level_two = level_one;
            level_two.current_player = FieldState::Cross;
            level_two.make_move_unchecked(j);
            
            if level_two.evaluate() == 1000 {
                outer_wins += 1;
            }

            if outer_wins > 1 {
                return i;
            }                
        }  
    }

    //Advanced fork heuristic
    for i in board.blank_fields() {
        let mut level_one = *board;
        level_one.current_player = FieldState::Cross;
        level_one.make_move_unchecked(i);

        let mut outer_wins = 0;

        for j in level_one.blank_fields() {
            let mut level_two = level_one;
            level_two.current_player = FieldState::Cross;
            level_two.make_move_unchecked(j);

            if level_two.evaluate() == 1000 {
                outer_wins += 1;
                continue;
            }

            let mut inner_wins = 0;

            for k in level_two.blank_fields() {
                let mut level_three = level_two;
                level_three.current_player = FieldState::Cross;
                level_three.make_move_unchecked(k);
                
                if level_three.evaluate() == 1000 {
                    inner_wins += 1;
                }

                if inner_wins > 1 {
                    outer_wins += 1;
                }
            }               
            
            if outer_wins > 1 {
                return i;
            }

        }  
    }

    // Block heuristic, else minimax
    if let Some(block) = board.best_block() {
        block
    } else {
        let best = Arc::new(Mutex::new(ComputedBest {
            min: 10000,
            x: 10,
            y: 10,
        }));

        let best_clone = best.clone();

        let mut pool = Vec::new();

        for i in board.blank_fields() {
            let depth = if board.array.size > 3 {
                general_depth
            } else {
                9
            };

            let mut new = *board;
            new.make_move_unchecked(i);

            let local_best = best_clone.clone();
            pool.push(thread::spawn(move || {
                let position = minimax(new, -10000, 10000, depth);

                let mut local_best = local_best.lock().unwrap();
                if position < local_best.min || (local_best.x == 10 && local_best.y == 10) {
                    local_best.min = position;
                    local_best.x = i.0;
                    local_best.y = i.1;
                }
            }));
        }

        for t in pool {
            t.join().unwrap();
        }

        let result = best.lock().unwrap();

        (result.x, result.y)
    }
}
