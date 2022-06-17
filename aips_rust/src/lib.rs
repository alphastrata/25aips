use chrono::{Duration, NaiveDate, NaiveDateTime};
use std::io::{BufRead, BufReader};

/// A struct repsesenting, and containing an entry from the dataset.
/// Fields:
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct CarCountEntry {
    pub timestamp: NaiveDateTime,
    pub count: usize,
    pub raw_data: String, // To facilitate reprocessing of data down the line if you need it...
}
impl CarCountEntry {
    /// Creates a new instance of CarCountEntry.
    // NOTE: Neither of the constructors are used currently, they are here for future use.
    #[inline]
    fn _new(timestamp: String, count: usize) -> Self {
        Self {
            timestamp: Self::parse_timestamp(&timestamp),
            count,
            raw_data: format!("{} {}", timestamp.to_owned(), count),
        }
    }
    /// Creates a new CarCountEntry, using a single line from dataset.
    fn _new_from_raw(raw_data: String) -> Self {
        let mut split = raw_data.split_whitespace();
        let timestamp = split.next().unwrap().to_owned();
        let count = split
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Unable to parse car count into an integer value due to incompatible data");
        Self::_new(timestamp, count)
    }

    /// Parses ISO8601 timestamp into a `NaiveDateTime` instance.
    fn parse_timestamp(timestamp: &str) -> NaiveDateTime {
        // "2021-12-01T05:00:00" -> chrono::NaiveDateTime
        let ts: Vec<&str> = timestamp.split("T").collect(); // LHS = YYYY-MM-DD, RHS = HH:MM:SS
        let ymd: Vec<u32> = ts[0]
            .split("-")
            .into_iter()
            .map(|i| i.trim().parse().unwrap_or(0))
            .collect();
        let hms: Vec<u32> = ts[1]
            .split(":")
            .into_iter()
            .map(|i| i.trim().parse().unwrap_or(0))
            .collect();

        NaiveDate::from_ymd(ymd[0] as i32, ymd[1], ymd[2]).and_hms(hms[0], hms[1], hms[2])
    }
}
/// A collection of CarCountEntry, wrapping them std::vec::Vec.
pub struct CarCountsCollection {
    pub collection: Vec<CarCountEntry>,
}

impl CarCountsCollection {
    /// Creates a CarCountsCollection (which just wrapping Vec<CarCountEntry>)
    pub fn new_from_disk(filename: &str) -> Self {
        let data = Self::read_from_disk(filename).expect("filename invalid");
        let mut v: Vec<CarCountEntry> = Vec::new();
        for entry in &data {
            let sp: Vec<&str> = entry.split_whitespace().collect();
            let t = CarCountEntry::parse_timestamp(sp[0].into());
            let c: String = sp[1].into();
            v.push(CarCountEntry {
                timestamp: t,
                count: c.trim().parse::<usize>().unwrap_or(0),
                raw_data: entry.clone(),
            }) //NOTE: Design descision here to take a count of 0 rather than a potential parse() error.
        }
        CarCountsCollection { collection: v }
    }
    /// Reads the raw data to feed CarCountEntry from a txt file on disk, assumes data is \n delimited.
    fn read_from_disk(filename: &str) -> anyhow::Result<Vec<String>> {
        let input = std::fs::File::open(filename)?;
        let buffered = BufReader::new(input).lines();
        let lines: Vec<String> = buffered.map(|e| e.unwrap()).collect();
        Ok(lines)
    }
    /// Counts the number of unique days in the dataset, returns Vec<NaiveDate> of those days.
    fn count_unique_days(&self) -> Vec<NaiveDate> {
        let mut unique_days: Vec<NaiveDate> = Vec::new();
        for entry in &self.collection {
            if !unique_days.contains(&entry.timestamp.date()) {
                unique_days.push(entry.timestamp.date());
            }
        }
        unique_days
    }
    /// Prints the total number of cars for a given day's entries.
    pub fn print_total_cars_counted_by_day(&self) {
        self.count_unique_days().into_iter().for_each(|day| {
            println!("{} {}", day.format("%Y-%m-%d"), self.total_for_day(day));
        });
    }
    /// Returns the total number of cars for a given day's entries.
    #[inline]
    fn total_for_day(&self, day: NaiveDate) -> usize {
        self.collection
            .iter()
            .filter(|e| e.timestamp.date() == day) // sanity check
            .map(|e| e.count)
            .sum()
    }
    /// Prints the total number of cars counted in the collection, as read from the dataset.
    #[inline]
    pub fn print_total_cars_counted(&self) {
        let total = self.collection.iter().fold(0, |acc, e| acc + e.count);
        println!("{}", total);
    }

    /// Prints the top three highest count entries.
    pub fn print_top_three_counts(&self) {
        let mut top_three = self.collection.clone();
        top_three.sort_by(|a, b| b.count.cmp(&a.count));
        top_three.truncate(3);
        for entry in &top_three {
            println!(
                "{} {}",
                entry.timestamp.format("%Y-%m-%dT%H:%M:%S"),
                entry.count
            );
        }
    }
    /// Returns the lowest sum of three entries, when said entries are a contiguous set of 90mins
    /// of unbroken, counting.
    fn lowest_90min_total(&self) -> Option<Vec<(usize, (NaiveDateTime, NaiveDateTime))>> {
        let mut window_of_90min: Vec<(usize, (NaiveDateTime, NaiveDateTime))> = Vec::new();
        let collection = self.collection.clone();
        for (n, entry) in collection.iter().enumerate() {
            if n + 2 < self.collection.len() {
                let start = entry.timestamp;
                let halfa = start + Duration::minutes(30);
                let hour = start + Duration::minutes(60);

                if n + 2 < self.collection.len()
                    && collection[n + 1].timestamp == halfa
                    && collection[n + 2].timestamp == hour
                {
                    let count =
                        collection[n].count + collection[n + 1].count + collection[n + 2].count;
                    window_of_90min.push((count, (start, hour)));
                }
            }
        }
        window_of_90min.sort_by(|a, b| a.0.cmp(&b.0));
        Some(window_of_90min)
    }
    /// Prints the lowest sum of three entries, where said entries are an unbroken set equal to
    /// 90mins.
    pub fn print_lowest_90min_total(&self) {
        if let Some(lowest) = self.lowest_90min_total() {
            lowest.iter().take(1).for_each(|e| {
                println!("{} {}", e.1 .0.format("%Y-%m-%d Starting:%H:%M"), e.0);
            });
        } else {
            println!(
                "No lowest 90min total found, not enough entries spanning a continious 90mins"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // You cannot use the rust Vec type as const, so this is a workaround to avoid code/dummy-data
    // duplication.
    fn provide_test_data_vec() -> Vec<&'static str> {
        //NOTE: It's kinda bad that the 'production' data is also the testing data.
        vec![
            "2021-12-01T05:00:00 5",
            "2021-12-01T05:30:00 12",
            "2021-12-01T06:00:00 14",
            "2021-12-01T06:30:00 15",
            "2021-12-01T07:00:00 25",
            "2021-12-01T07:30:00 46",
            "2021-12-01T08:00:00 42",
            "2021-12-01T15:00:00 9",
            "2021-12-01T15:30:00 11",
            "2021-12-01T23:30:00 0",
            //
            "2021-12-05T09:30:00 18",
            "2021-12-05T10:30:00 15",
            "2021-12-05T11:30:00 7",
            "2021-12-05T12:30:00 6",
            "2021-12-05T13:30:00 9",
            "2021-12-05T14:30:00 11",
            "2021-12-05T15:30:00 15",
            //
            "2021-12-08T18:00:00 33",
            "2021-12-08T19:00:00 28",
            "2021-12-08T20:00:00 25",
            "2021-12-08T21:00:00 21",
            "2021-12-08T22:00:00 16",
            "2021-12-08T23:00:00 11",
            //
            "2021-12-09T00:00:00 4",
        ]
    }

    #[test]
    fn test_totals_by_day() {
        let data = CarCountsCollection::new_from_disk("data.txt");
        let totals_8th = data.total_for_day(NaiveDate::from_ymd(2021, 12, 8)); //2021-12-08 134
        let totals_5th = data.total_for_day(NaiveDate::from_ymd(2021, 12, 5)); //2021-12-05 81
        assert_eq!(totals_8th, 134);
        assert_eq!(totals_5th, 81);
    }
    #[test]
    fn test_top_three() {
        let top_three_ans = vec![
            "2021-12-01T07:30:00 46",
            "2021-12-01T08:00:00 42",
            "2021-12-08T18:00:00 33",
        ];
        let data = CarCountsCollection::new_from_disk("data.txt");
        let mut top_three = data.collection.clone();
        top_three.sort_by(|a, b| b.count.cmp(&a.count));
        top_three.truncate(3);
        for (i, entry) in top_three.iter().enumerate() {
            assert_eq!(entry.raw_data, top_three_ans[i]);
        }
    }
    #[test]
    fn can_read_txt_data() {
        let res = CarCountsCollection::read_from_disk("data.txt").unwrap();
        let test_data = provide_test_data_vec();
        for i in res.into_iter().zip(test_data.into_iter()) {
            assert_eq!(i.0, i.1)
        }
    }
    #[test]
    fn can_create_entries_from_disk() {
        let test_data = provide_test_data_vec();
        assert_eq!(
            test_data.len(),
            CarCountsCollection::new_from_disk("data.txt")
                .collection
                .len()
        )
    }
    #[test]
    //NOTE: Tests here are bundled to reduce loc.
    fn test_datetime_parser() {
        let mut test_data = vec![
            "2021-12-01T05:00:00 5",
            "2021-12-01T08:00:00 42",
            "2021-12-01T15:00:00 9",
        ]
        .into_iter();
        assert_eq!(
            NaiveDate::from_ymd(2021, 12, 01).and_hms(5, 0, 0),
            CarCountEntry::parse_timestamp(test_data.next().unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd(2021, 12, 01).and_hms(8, 0, 0),
            CarCountEntry::parse_timestamp(test_data.next().unwrap())
        );
        assert_eq!(
            NaiveDate::from_ymd(2021, 12, 01).and_hms(15, 0, 0),
            CarCountEntry::parse_timestamp(test_data.next().unwrap())
        );
    }

    #[test]
    fn test_lowest_90min() {
        let data = CarCountsCollection::new_from_disk("data.txt");
        let ninety = data.lowest_90min_total();
        let answers = vec![31, 41, 54]; // 86 and 113 ommited, but are the other two options, if
                                        // the window were larger.
        for i in ninety.unwrap().iter().zip(answers.into_iter()) {
            assert_eq!(i.0 .0, i.1)
        }
    }

    #[test]
    fn total_cars_counted() {
        let data = CarCountsCollection::new_from_disk("data.txt");
        let total = data.collection.iter().fold(0, |acc, e| acc + e.count); //NOTE: testing logic
                                                                            //not function.
        assert_eq!(total, 398);
    }
}
