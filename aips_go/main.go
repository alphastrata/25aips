package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
	"time"

	"golang.org/x/exp/slices"
)

// Read data from assets/data.txt and return a slice of bytes.
func ReadData(p string) []byte {
	data, err := ioutil.ReadFile(p)
	if err != nil {
		log.Fatal(err)
	}

	return data
}

// CarCountEntry represents a single entry in the dataset, containing the counted cars and timestamp at indicating the start of a half hour period in which data was collected.
type CarCountEntry struct {
	Count     int
	Timestamp int64 // We're using unix time
}

type CarCountData struct {
	Entries []CarCountEntry
}

// Parse dataset strings of ISO8601 format into unix time.
func ParseTimestamp(iso string) int64 {
	iso = iso + "Z"
	t, err := time.Parse(time.RFC3339, iso)
	if err != nil {
		log.Fatal(err)
	}
	return t.Unix()
}

// Timestamp2String converts a unix timestamp to a string in the format of "YYYY-MM-DD HH:MM:SS".
func Timestamp2String(ts int64) []string {
	//1638340200 -> 2021-12-01T14:30:00 08:00
	t := time.Unix(ts, 0).In(time.FixedZone("UTC", 0))
	//return strings.Split(t.Format(time.RFC3339), "+")
	return strings.Split(t.Format(time.RFC3339), "+")
}

// NewCarCountEntry creates a new CarCountEntry.
func NewCarCountEntry(c int, ts int64) CarCountEntry {
	return CarCountEntry{Count: c, Timestamp: ts}
}

// Sort the dataset by timestamp, instead of implementing a custom sort function for our type.
func (dataset *CarCountData) SortByTimestamp() {
	for i := 0; i < len(dataset.Entries); i++ {
		for j := 0; j < len(dataset.Entries)-1; j++ {
			if dataset.Entries[j].Timestamp > dataset.Entries[j+1].Timestamp {
				dataset.Entries[j], dataset.Entries[j+1] = dataset.Entries[j+1], dataset.Entries[j]
			}
		}
	}

}

// Sort the dataset by count, instead of implementing a custom sort function.
func (dataset *CarCountData) SortByCount() {
	for i := 0; i < len(dataset.Entries); i++ {
		for j := 0; j < len(dataset.Entries)-1; j++ {
			if dataset.Entries[j].Count > dataset.Entries[j+1].Count {
				dataset.Entries[j], dataset.Entries[j+1] = dataset.Entries[j+1], dataset.Entries[j]
			}
		}
	}

}

// Builds a dataset of CarCountData
func BuildDataset(data []byte) CarCountData {
	sp := strings.Split(string(data), "\n")

	var dataset = CarCountData{}

	for i := 0; i < len(sp)-1; i++ {
		sp2 := strings.Split(sp[i], " ")

		c, err := strconv.Atoi(strings.TrimSpace(sp2[1]))
		if err != nil {
		}
		ts := ParseTimestamp(strings.TrimSpace(sp2[0]))

		dataset.Entries = append(dataset.Entries, NewCarCountEntry(c, ts))
	}
	return dataset
}

// Returns the top three half hour entries by count
func (c *CarCountData) GetTopThreeEntriesByCount() []CarCountEntry {
	c.SortByCount()
	var topThree []CarCountEntry
	for i := 0; i < 3; i++ {
		topThree = append(topThree, c.Entries[len(c.Entries)-1-i])
	}
	return topThree

}

// TODO: Use generics to make this type support any slice of time(y/m/d)
type DailyTotal struct {
	Day   string
	Count int
}

// Returns a [](string, int) where the string is the date in YYYY-MM-DD and the int is the number of cars counted on that day.
func (c *CarCountData) GetDailyTotals(days []string) []DailyTotal {
	var dailyTotals []DailyTotal
	for i := range days {
		var dailyTotalCount int
		for j := range c.Entries {
			if strings.Contains(Timestamp2String(c.Entries[j].Timestamp)[0], days[i]) {
				dailyTotalCount += c.Entries[j].Count
			}
		}
		dailyTotals = append(dailyTotals, DailyTotal{Day: days[i], Count: dailyTotalCount})
	}
	return dailyTotals
}

// Returns the total number of cars counted in the dataset
func (c *CarCountData) GetTotalCount() int {
	var total int
	for i := 0; i < len(c.Entries); i++ {
		total += c.Entries[i].Count
	}
	return total
}

// Returns a new timestamp that is ahead of the given timestamp by the given number of seconds supplied in `d`.
func AdvanceTimestampBy(ts int64, d int64) int64 {
	return ts + d
}

// Returns a bool to indicate whether a timestamp is within a calendar day.
func (c *CarCountEntry) IsWithinDay() bool {
	t := time.Unix(c.Timestamp, 0)
	t2 := time.Unix(AdvanceTimestampBy(c.Timestamp, 86400), 0)
	if t.Year() == t2.Year() && t.Month() == t2.Month() && t.Day() == t2.Day() {
		return true
	}
	return false
}

func (c *CarCountData) GetUniqueDays() []string {
	var uniqueDays []string
	for i := range c.Entries {
		toCheck := strings.Split(Timestamp2String(c.Entries[i].Timestamp)[0], "T")[0]
		if !slices.Contains(uniqueDays, toCheck) {
			uniqueDays = append(uniqueDays, toCheck)
		}
	}

	return uniqueDays // this is their timestamps
}

// Returns the total number of cars counted on a given day

func main() {
	// read the data
	data := ReadData("assets/data.txt")

	dataset := BuildDataset(data)

	// print total number of cars counted in dataset
	total := dataset.GetTotalCount()
	fmt.Println("\nTotal cars counted:", total)

	// print out the totals by day : one line per day with total
	fmt.Println("\nTotal cars counted by day:")
	//print 2016-11-23 289
	uniqueDays := dataset.GetUniqueDays()
	dailyTotals := dataset.GetDailyTotals(uniqueDays)
	for i := 0; i < len(dailyTotals); i++ {
		fmt.Println(dailyTotals[i].Day, ":", dailyTotals[i].Count)
	}

	dataset.SortByCount()
	topThree := dataset.Entries[(len(dataset.Entries) - 3):]
	//print 2016-11-23THH:MM:SS 289 // SAME FORMAT AS data.txt
	fmt.Println("\nTop three half hour entries:")
	for i := range topThree {
		ts := (topThree[i].Timestamp)
		tsString := Timestamp2String(ts)[0]
		c := (topThree[i].Count)
		fmt.Println((tsString)[0:len(tsString)-1], c)

	}

	// print the lowest 3 90min-contigious entries // NO FORMAT REQUIREMENTS SPECIFIED
	fmt.Println("\nLowest three 90min-contigious entries:")

}
