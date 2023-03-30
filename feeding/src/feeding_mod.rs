#[derive(Debug)]
pub struct Weight {
    pub boden: f32,
    pub zarge: f32,
    pub rahmen: f32,
    pub fuetterer: f32,
    pub kissen: f32,
    pub deckel: f32,
}

#[derive(Debug)]
pub struct Count {
    pub boden: i8,
    pub zarge: i8,
    pub rahmen: i8,
    pub fuetterer: i8,
    pub kissen: i8,
    pub deckel: i8,
}

fn init_warre_weights() -> Weight {
    let warre_weight = Weight {
        boden: 1.5,
        zarge: 2.44,
        rahmen: 0.115,
        fuetterer: 1.32,
        kissen: 2.0,
        deckel: 5.0,
    };
    return warre_weight;
}

fn init_normalmass_weight() -> Weight {
    let normalmass_weight = Weight {
        boden: 1.75,
        zarge: 2.08,
        rahmen: 0.130,
        fuetterer: 1.6,
        kissen: 0.0,
        deckel: 5.0,
    };
    return normalmass_weight;
}

pub fn init_dadant_weight() -> Weight {
    let dadant_weight = Weight {
        boden: 0.0,
        zarge: 3.7,
        rahmen: 0.275,
        fuetterer: 2.5,
        kissen: 2.0,
        deckel: 5.0,
    };
    return dadant_weight;
}

pub fn init_warre_count() -> Count {
    let warre_count = Count {
        boden: 1,
        zarge: 3,
        rahmen: 8,
        fuetterer: 1,
        kissen: 1,
        deckel: 1,
    };
    return warre_count;
}

pub fn init_normallmass_count() -> Count {
    let normalmass_count = Count {
        boden: 1,
        zarge: 2,
        rahmen: 11,
        fuetterer: 1,
        kissen: 0,
        deckel: 1,
    };
    return normalmass_count;
}

pub fn init_dadant_count() -> Count {
    let dadant_count = Count {
        boden: 1,
        zarge: 1,
        rahmen: 10,
        fuetterer: 1,
        kissen: 1,
        deckel: 1,
    };
    return dadant_count;
}

pub fn netto_weight(weight: Weight, count: Count, feeder: bool) -> f32 {
    let mut weight_ = weight.boden * count.boden as f32
        + weight.zarge * count.zarge as f32
        + weight.rahmen * count.rahmen as f32
        + weight.kissen * count.kissen as f32
        + weight.deckel * count.deckel as f32;

    if feeder == true {
        weight_ += weight.fuetterer * count.fuetterer as f32
    }
    return weight_;
}

pub fn feed_need(target: f32, current: f32) -> f32 {
    let feeding = 1.3 * (target - current);
    return feeding
}
