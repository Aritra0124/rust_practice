use std::cmp::min;
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frame: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self{
            frame: Vec::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        
        let rolls = self.frame_calculation();
        if !rolls.is_empty(){
            if rolls.last().unwrap().len() == 1{
                if *rolls.last().unwrap().last().unwrap() != 10 && 10 - rolls.last().unwrap().last().unwrap() < pins{
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
            if rolls.len() == 10 && rolls.last().unwrap().len() == 3{
                return Err(Error::GameComplete);
            }else if rolls.len() == 10 && rolls.last().unwrap().len() == 2 && rolls.last().unwrap().iter().sum::<u16>() < 10 {
                return Err(Error::GameComplete);
            }else if rolls.len() == 10 && rolls.last().unwrap().len() == 2 && rolls.last().unwrap().iter().sum::<u16>() > 10 {
                if rolls.last().unwrap()[0] == 10 && rolls.last().unwrap()[1] < pins{
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
        }
        if pins >= 0 && pins<= 10{
                self.frame.push(pins);
                return Ok(());
        }
        return Err(Error::NotEnoughPinsLeft);
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame.is_empty(){
            return None;   
        }
        let mut val = 0; 
        let rolls: Vec<Vec<u16>> = self.frame_calculation();
        for i in rolls.iter(){
            if i.len() == 3 && (i[0] + i[1] == 10 || i[0] == 10){
                val += i.iter().sum::<u16>();
            }else if i.len() == 2 && i[0] + i[1] < 10{
                val += i.iter().sum::<u16>();
            }else{
                return None;
            }
        }
        if rolls.len() < 10 && val == 0{
            return None;
        }else if rolls.len() == 10 && val == 0{
            return Some(0);
        }
        Some(val as u16)
    }
    pub fn frame_calculation(&self) -> Vec<Vec<u16>>{
        let mut rolls: Vec<Vec<u16>> = Vec::new();
        let mut frame_count = 0;
        let mut i = 0;
        while i < self.frame.len()  && frame_count < 10{
            if frame_count == 9 {
                rolls.push(self.frame[i..].to_vec());
                break;
            }
            if self.frame[i] == 10 {
                let end = min(i + 3, self.frame.len());
                rolls.push(self.frame[i..end].to_vec());
                i += 1;
            } else if i + 1 < self.frame.len() {
                let frame_sum = self.frame[i] + self.frame[i + 1];
                if frame_sum == 10 {
                    let end = min(i + 3, self.frame.len());
                    rolls.push(self.frame[i..end].to_vec());
                } else {
                    rolls.push(vec![self.frame[i], self.frame[i + 1]]);
                }
                i += 2;
            }
    
            frame_count += 1;
            if frame_count == 10 {
            break;
            }
        }
        rolls
    }
}
