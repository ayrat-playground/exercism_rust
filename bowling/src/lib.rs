#[derive(Clone, Debug)]
pub struct Frame {
    idx: usize,
    throws: Vec<usize>
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>
}

impl Frame {
    fn new(idx: usize, throw: usize) -> Result<Self, &'static str> {
        if idx > 9 || throw > 10 { return Err("Wrong idx or throw for a frame") }

        Ok(Frame{ idx: idx, throws: vec!(throw) })
    }

    fn add(&mut self, throw: usize) -> Result<(), &'static str> {
        if self.is_full() { return Err("Frame is full") }
        if throw > 10 { return Err("Wrong throw") }

        self.throws.push(throw);

        if (self.throws.iter().sum::<usize>() > 10 && self.idx != 9) ||
            (self.throws.len() == 3 && self.idx == 9 && self.throws[0] == 10 && self.throws[1] < 10 && self.throws[1] + self.throws[2] > 10) {
            Err("Wrong throw sum")
        } else {
            Ok(())
        }
    }

    fn is_full(&self) -> bool {
        if (self.throws.len() == 2 && self.idx < 9) ||
            (self.idx == 9 && self.throws.len() == 3) ||
            (self.idx == 9 && self.throws.len() == 2 && (self.throws[0] + self.throws[1]) < 10)
        {
            return true
        }
        if self.idx != 9 && self.throws[0] == 10 { return true }

        false
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: Vec::new() }
    }

    pub fn roll(&mut self, throw: usize) -> Result<(), &'static str> {
        if self.frames.len() == 0 {
            let frame = Frame::new(0, throw)?;

            self.frames.push(frame);
        } else if self.frames.last().unwrap().is_full() {
            let frame = Frame::new(self.frames.len(), throw)?;

            self.frames.push(frame);
        } else {
            let last_idx = self.frames.len() - 1;
            let mut frame = self.frames[last_idx].clone();

            frame.add(throw)?;

            self.frames[last_idx] = frame;
        }

        Ok(())
    }

    pub fn score(&self) -> Result<usize, ()> {
        if (self.frames.len() != 10) ||
            (self.frames.len() == 10 && self.frames[9].throws.len() == 1 && self.frames[9].throws[0] == 10) ||
            (self.frames.len() == 10 && self.frames[9].throws.len() == 2 && self.frames[9].throws[1] == 10) ||
            (self.frames.len() == 10 && self.frames[9].throws.len() == 2 && self.frames[9].throws.iter().sum::<usize>() == 10)
        { return Err(()) }

        let mut result = 0;

        for frame in &self.frames {
            if frame.idx != 9 {
                if frame.throws[0] == 10 {
                    if self.frames[frame.idx + 1].throws[0] == 10 {
                        if frame.idx + 1 == 9 {
                            result += 10 + self.frames[frame.idx + 1].throws[0] + self.frames[frame.idx + 1].throws[1];
                        } else {
                            result += 10 + self.frames[frame.idx + 1].throws[0] + self.frames[frame.idx + 2].throws[0];
                        }
                    } else {
                        result += 10 + self.frames[frame.idx + 1].throws.iter().sum::<usize>();
                    }
                } else if frame.throws.iter().sum::<usize>() == 10 {
                    result += 10 + self.frames[frame.idx + 1].throws[0];
                } else {
                    result += frame.throws.iter().sum::<usize>();
                }
            } else {
                result += frame.throws.iter().sum::<usize>();
            }
        }

        Ok(result)
    }
}
