// modul for calculating the feedings etc.
pub mod calculations {
    use round::round;

    use crate::hivetypes::calculations::Types::{dadant, deutschnormal, warre};

    #[derive(Clone, Copy)]
    pub enum Types {
        warre,
        dadant,
        deutschnormal,
    }

    pub trait SetZargeCount {
        fn set_zarge_count(&mut self, cnt: i8) -> &Self;
    }

    pub trait SetFeeder {
        fn set_feeder(&mut self, fd: bool) -> &Self;
    }

    pub trait New {
        fn new(types: Types) -> Self;
    }

    pub trait NettoWeight {
        fn netto_weight(&self) -> f32;
    }

    pub trait BruttoWeight {
        fn brutto_weight(&self) -> f32;
    }

    pub trait FeedPresent {
        fn feed_present(&self) -> f32;
    }

    pub trait SetCurrentWeight {
        fn set_current_weight(&mut self, current: f32) -> ();
    }

    pub trait FeedNeed {
        fn feed_need(&self) -> f32;
    }

    pub trait HiveName {
        fn return_hive_name(&self) -> String;
    }

    pub struct HiveTypes {
        pub boden: (f32, i8),
        pub zarge: (f32, i8),
        pub rahmen: (f32, i8),
        pub fuetterer: (f32, i8),
        pub kissen: (f32, i8),
        pub deckel: (f32, i8),
        pub current_weight: f32,
        pub types: Types,
    }

    impl SetZargeCount for HiveTypes {
        fn set_zarge_count(&mut self, cnt: i8) -> &Self {
            self.zarge.1 = cnt;
            return self;
        }
    }

    impl SetFeeder for HiveTypes {
        fn set_feeder(&mut self, fd: bool) -> &Self {
            if fd {
                self.fuetterer.1 = 1;
            } else {
                self.fuetterer.1 = 0;
            }
            self
        }
    }

    impl New for HiveTypes {
        fn new(actual_type: Types) -> Self {
            match actual_type {
                Types::warre => Self {
                    boden: (1.5, 1),
                    zarge: (2.44, 3),
                    rahmen: (0.150, 8),
                    fuetterer: (1.32, 1),
                    kissen: (2.0, 1),
                    deckel: (5.0, 1),
                    current_weight: 0.0,
                    types: warre,
                },

                Types::dadant => Self {
                    boden: (0.0, 1),
                    zarge: (3.7, 1),
                    rahmen: (0.275, 10),
                    fuetterer: (2.50, 1),
                    kissen: (2.0, 1),
                    deckel: (5.0, 1),
                    current_weight: 0.0,
                    types: dadant,
                },

                Types::deutschnormal => Self {
                    boden: (2.0, 1),
                    zarge: (2.66, 2),
                    rahmen: (0.190, 11),
                    fuetterer: (1.6, 1),
                    kissen: (0.0, 1),
                    deckel: (1.5, 1),
                    current_weight: 0.0,
                    types: deutschnormal,
                },
            }
        }
    }

    impl NettoWeight for HiveTypes {
        fn netto_weight(&self) -> f32 {
            let weight_ = self.boden.0 * self.boden.1 as f32
                + self.zarge.0 * self.zarge.1 as f32
                + self.rahmen.0 * self.rahmen.1 as f32 * self.zarge.1 as f32
                + self.kissen.0 * self.kissen.1 as f32
                + self.fuetterer.0 * self.fuetterer.1 as f32
                + self.deckel.0 * self.deckel.1 as f32;
            let weight_ = round(weight_ as f64, 2);
            return weight_ as f32;
        }
    }

    impl BruttoWeight for HiveTypes {
        fn brutto_weight(&self) -> f32 {
            let weight = round(HiveTypes::netto_weight(&self) as f64 + 22.0, 2);
            return weight as f32;
        }
    }

    impl FeedPresent for HiveTypes {
        fn feed_present(&self) -> f32 {
            let feed = self.current_weight - HiveTypes::netto_weight(&self) - 2.0; // 2.0 bees or so
            return if feed > 0.0 {
                let feed = round(feed as f64, 2);
                feed as f32
            } else {
                let feed = 0.0;
                feed as f32
            };
        }
    }

    impl SetCurrentWeight for HiveTypes {
        fn set_current_weight(&mut self, current: f32) -> () {
            self.current_weight = current;
        }
    }

    impl FeedNeed for HiveTypes {
        fn feed_need(&self) -> f32 {
            let target = self.brutto_weight();
            let feeding = 1.3 * (target - self.current_weight);
            return if feeding > 0.0 {
                let feeding = round(feeding as f64, 2);
                feeding as f32
            } else {
                let feeding = 0.0;
                feeding as f32
            };
        }
    }

    impl HiveName for HiveTypes {
        fn return_hive_name(&self) -> String {
            match self.types {
                Types::warre => return "Warré".to_string(),
                Types::deutschnormal => return "Deutsch Normalmass".to_string(),
                Types::dadant => return "Dadant".to_string(),
            }
        }
    }
}
