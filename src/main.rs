/*
use csv::ReaderBuilder;
use std::path::Path;
use serde::{Deserialize,Serialize};
use burn::data::dataset::{Dataset,InMemDataset};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct WindData{
    #[serde(rename = "wind_speed_2m:ms")]
    pub speed2m: f64,
    #[serde(rename = "wind_speed_10m:ms")]
    pub speed10m: f64,
    #[serde(rename = "wind_speed_50m:ms")]
    pub speed50m: f64,
    #[serde(rename = "wind_speed_100m:ms")]
    pub speed100m: f64,
    #[serde(rename = "wind_dir_2m:d")]
    pub dir2m: f64,
    #[serde(rename = "wind_dir_10m:d")]
    pub dir10m: f64,
    #[serde(rename = "wind_dir_50m:d")]
    pub dir50m: f64,
    #[serde(rename = "wind_dir_100m:d")]
    pub dir100m: f64,
}

pub struct WindDataSet{
    dataset: InMemDataset<WindData>,
}

impl WindDataSet{
    pub const TRAIN: &'static str = "assets/train.csv";
    pub const TEST: &'static str = "assets/test.csv";

    pub fn load(path: &Path){
        let mut reader = ReaderBuilder::new();
        let reader = reader.delimiter(b'\t');
        let dataset = InMemDataset::from_csv(path, reader);
        if let Ok(dataset) = dataset{
            return Self{dataset};
        }
    }

}

impl Dataset<WindData> for WindDataSet{
    fn get(&self, index: usize) -> Option<&WindData>{
        self.dataset.get(index)
    }

    fn len(&self) -> usize{
        self.dataset.len()
    }
}
*/

fn main() {
    println!("Hello, world!")
}
