// modul for calculating the feedings etc.

use round::round;
use std::arch::x86_64::_mm_loadl_epi64;

pub trait SetZargeCount {
    fn set_zarge_count(self, cnt: i8) -> Self;
}

pub trait SetFeeder {
    fn set_feeder(self, fd: bool) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Weight {
    pub boden: f32,
    pub zarge: f32,
    pub rahmen: f32,
    pub fuetterer: f32,
    pub kissen: f32,
    pub deckel: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Count {
    pub boden: i8,
    pub zarge: i8,
    pub rahmen: i8,
    pub fuetterer: i8,
    pub kissen: i8,
    pub deckel: i8,
}

impl SetZargeCount for Count {
    fn set_zarge_count(mut self, cnt: i8) -> Count {
        self.zarge = cnt;
        return self;
    }
}

impl SetFeeder for Count {
    fn set_feeder(mut self, fd: bool) -> Count {
        self.fuetterer = match fd {
            true => 1,
            false => 0,
        };
        return self;
    }
}

pub struct WarreWeights {}

impl WarreWeights {
    pub fn new() -> Weight {
        Weight {
            boden: 1.5,
            zarge: 2.44,
            rahmen: 0.115,
            fuetterer: 1.32,
            kissen: 2.0,
            deckel: 5.0,
        }
    }
}

pub struct NormalmassWeights {}

impl NormalmassWeights {
    pub fn new() -> Weight {
        Weight {
            boden: 1.75,
            zarge: 2.08,
            rahmen: 0.130,
            fuetterer: 1.6,
            kissen: 0.0,
            deckel: 5.0,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DadantWeights {}

impl DadantWeights {
    pub fn new() -> Weight {
        Weight {
            boden: 0.0,
            zarge: 3.7,
            rahmen: 0.275,
            fuetterer: 2.5,
            kissen: 2.0,
            deckel: 5.0,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct WarreCounts {}

impl WarreCounts {
    pub fn new() -> Count {
        Count {
            boden: 1,
            zarge: 3,
            rahmen: 8,
            fuetterer: 1,
            kissen: 1,
            deckel: 1,
        }
    }
}

pub struct NormalmassCounts {}

impl NormalmassCounts {
    pub fn new() -> Count {
        Count {
            boden: 1,
            zarge: 2,
            rahmen: 11,
            fuetterer: 1,
            kissen: 0,
            deckel: 1,
        }
    }
}

pub struct DadantCounts {}

impl DadantCounts {
    pub fn new() -> Count {
        Count {
            boden: 1,
            zarge: 1,
            rahmen: 10,
            fuetterer: 1,
            kissen: 1,
            deckel: 1,
        }
    }
}

pub fn netto_weight(weight: Weight, count: Count) -> f32 {
    let mut weight_ = weight.boden * count.boden as f32
        + weight.zarge * count.zarge as f32
        + weight.rahmen * count.rahmen as f32 * count.zarge as f32
        + weight.kissen * count.kissen as f32
        + weight.fuetterer * count.fuetterer as f32
        + weight.deckel * count.deckel as f32;
    let weight_ = round(weight_ as f64, 2);
    return weight_ as f32;
}

pub fn brutto_weight(weight: Weight, count: Count) -> f32 {
    let weight = round(netto_weight(weight, count) as f64 + 22.0, 2);
    return weight as f32;
}

pub fn feed_need(target: f32, current: f32) -> f32 {
    let feeding = 1.3 * (target - current);
    return if feeding > 0.0 {
        let feeding = round(feeding as f64, 2);
        feeding as f32
    } else {
        let feeding = 0.0;
        feeding as f32
    };
}
