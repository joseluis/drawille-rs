use std::char;
use std::cmp;

use fnv::FnvHashMap;

static PIXEL_MAP: [[u8; 2]; 4] = [[0x01, 0x08],
                                   [0x02, 0x10],
                                   [0x04, 0x20],
                                   [0x40, 0x80]];

/// A canvas object that can be used to draw to the terminal using Braille characters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Canvas {
    chars: FnvHashMap<(u16, u16), (u8, char)>,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl Canvas {
    /// Creates a new `Canvas` with the given width and height.
    ///
    /// Note that the `Canvas` can still draw outside the given dimensions (expanding the canvas)
    /// if a pixel is set outside the dimensions.
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            chars: FnvHashMap::default(),
            width: (width / 2) as u16,
            height: (height / 4) as u16,
        }
    }

    /// Clears the canvas.
    pub fn clear(&mut self) {
        self.chars.clear();
    }

    /// Sets a pixel at the specified coordinates.
    pub fn set(&mut self, x: u32, y: u32) {
        let (row, col) = ((x / 2) as u16, (y / 4) as u16);
        let a = self.chars.entry((row, col)).or_insert((0,' '));
        a.0 |= PIXEL_MAP[y as usize % 4][x as usize % 2];
        a.1 = ' ';
    }

    /// Sets a letter at the specified coordinates.
    pub fn set_char(&mut self, x: u32, y: u32, c: char) {
        let (row, col) = ((x / 2) as u16, (y / 4) as u16);
        let a = self.chars.entry((row, col)).or_insert((0,' '));
        a.0 = 0;
        a.1 = c;
    }

    /// Draws text at the specified coordinates (top-left of the text) up to max_width length
    pub fn text(&mut self, x: u32, y: u32, max_width: u32, text: &str) {
        for (i, c) in text.chars().enumerate() {
            let w = i as u32 * 2;
            if w > max_width {
                return;
            }
            self.set_char(x + w, y, c);
        }
    }

    /// Deletes a pixel at the specified coordinates.
    pub fn unset(&mut self, x: u32, y: u32) {
        let (row, col) = ((x / 2) as u16, (y / 4) as u16);
        let a = self.chars.entry((row, col)).or_insert((0,' '));
        a.0 &= !PIXEL_MAP[y as usize % 4][x as usize % 2];
    }

    /// Toggles a pixel at the specified coordinates.
    pub fn toggle(&mut self, x: u32, y: u32) {
        let (row, col) = ((x / 2) as u16, (y / 4) as u16);
        let a = self.chars.entry((row, col)).or_insert((0,' '));
        a.0 ^= PIXEL_MAP[y as usize % 4][x as usize % 2];
    }

    /// Detects whether the pixel at the given coordinates is set.
    pub fn get(&self, x: u32, y: u32) -> bool {
        let (row, col) = ((x / 2) as u16, (y / 4) as u16);
        self.chars.get(&(row, col)).map_or(false, |a| {
            let dot_index = PIXEL_MAP[y as usize % 4][x as usize % 2];
            a.0 & dot_index != 0
        })
    }

    /// Returns a `Vec` of each row of the `Canvas`.
    ///
    /// Note that each row is actually four pixels high due to the fact that a single Braille
    /// character spans two by four pixels.
    pub fn rows(&self) -> Vec<String> {
        let mut maxrow = self.width;
        let mut maxcol = self.height;
        for &(x, y) in self.chars.keys() {
            if x > maxrow {maxrow = x;}
            if y > maxcol {maxcol = y;}
        }

        let mut result = Vec::with_capacity(maxcol as usize + 1);
        for y in 0..=maxcol {
            let mut row = String::with_capacity(maxrow as usize + 1);
            for x in 0..=maxrow {
                let cell = self.chars.get(&(x, y)).cloned().unwrap_or((0,' '));
                row.push(if cell.0 == 0 {
                    cell.1
                } else {
                    char::from_u32(0x2800 + cell.0 as u32).unwrap()
                })
            }
            result.push(row);
        }
        result
    }

    /// Draws the canvas to a `String` and returns it.
    pub fn frame(&self) -> String {
        self.rows().join("\n")
    }

    /// Draws a line from `(x1, y1)` to `(x2, y2)` onto the `Canvas`.
    pub fn line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        let xdiff = cmp::max(x1, x2) - cmp::min(x1, x2);
        let ydiff = cmp::max(y1, y2) - cmp::min(y1, y2);
        let xdir = if x1 <= x2 { 1 } else { -1 };
        let ydir = if y1 <= y2 { 1 } else { -1 };

        let r = cmp::max(xdiff, ydiff);

        for i in 0..=r {
            let mut x = x1 as i32;
            let mut y = y1 as i32;

            if ydiff != 0 {
                y += ((i * ydiff) / r) as i32 * ydir;
            }
            if xdiff != 0 {
                x += ((i * xdiff) / r) as i32 * xdir;
            }

            self.set(x as u32, y as u32);
        }
    }
}
