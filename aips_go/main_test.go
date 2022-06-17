package main

import (
	"fmt"
	"strings"
	"testing"
)

var testData = []string{
	"5 1638334800", "12 1638336600", "14 1638338400", "15 1638340200", "25 1638342000", "46 1638343800", "42 1638345600", "9 1638370800", "11 1638372600", "0 1638401400", "18 1638696600", "15 1638700200", "7 1638703800", "6 1638707400", "9 1638711000", "11 1638714600", "15 1638718200", "33 1638986400", "28 1638990000", "25 1638993600", "21 1638997200", "16 1639000800", "11 1639004400", "4 1639008000"}

func TestReadDataIntoCarCountData(t *testing.T) {
	data := ReadData("assets/data.txt")
	dataset := BuildDataset(data).Entries
	for i := range dataset {
		testCount := fmt.Sprintf("%d", dataset[i].Count)
		testTs := fmt.Sprintf("%d", dataset[i].Timestamp)
		truth_count := strings.Split(testData[i], " ")[0]
		truth_ts := strings.Split(testData[i], " ")[1]
		if testCount != truth_count {
			t.Errorf("counts do not match: %s != %s", testCount, testData[i])
		}
		if testTs != truth_ts {
			t.Errorf("timestamps do not match: %s != %s", testTs, testData[i])
		}

	}

}

func TestGetTopThreeEntriesByCount(t *testing.T) {
	data := ReadData("assets/data.txt")
	dataset := BuildDataset(data)
	topThree := dataset.GetTopThreeEntriesByCount() //NOTE: add a reverse func for []CarCountEntry
	t.Logf("%+v", topThree)
	if topThree[0].Count != int(46) {
		t.Errorf("top three count does not match: %d != %d", topThree[0].Count, 46)
	}
	if topThree[1].Count != int(42) {
		t.Errorf("top three count does not match: %d != %d", topThree[1].Count, 42)
	}
	if topThree[2].Count != int(33) {
		t.Errorf("top three count does not match: %d != %d", topThree[2].Count, 33)
	}

}

func TestTotalCarsCount(t *testing.T) {
	// 398
	data := ReadData("assets/data.txt")
	dataset := BuildDataset(data)
	total_cars := dataset.GetTotalCount()
	if total_cars != 398 {
		t.Errorf("total cars count does not match: %d != %d", total_cars, 398)
	}
}

func TestGetUniqueDays(t *testing.T) {
	// 4
	data := ReadData("assets/data.txt")
	dataset := BuildDataset(data)
	uniqueDays := dataset.GetUniqueDays()
	if len(uniqueDays) != 4 {
		t.Errorf("unique days count does not match: %d != %d", len(uniqueDays), 5)
	}

}
func TestGetDailyTotals(t *testing.T) {
	// 1st dec: 179
	// 5th dec: 81
	// 8th dec: 134
	// 9th dec: 4
	data := ReadData("assets/data.txt")
	dataset := BuildDataset(data)
	uniqueDays := dataset.GetUniqueDays()
	dataset.SortByTimestamp()
	totalsByDay := dataset.GetDailyTotals(uniqueDays)
	for i := range totalsByDay {
		t.Logf("Day: %s on %d", totalsByDay[i].Day, totalsByDay[i].Count)
	}
	if totalsByDay[0].Count != int(179) {
		t.Errorf("Should be 179 but was:%d", totalsByDay[0].Count)
	}

	if totalsByDay[1].Count != int(81) {
		t.Errorf("Should be  81 but was:%d", totalsByDay[1].Count)
	}
	if totalsByDay[2].Count != int(134) {
		t.Errorf("Should be 134 but was:%d", totalsByDay[2].Count)
	}
	if totalsByDay[3].Count != int(4) {
		t.Errorf("should be   4 but was:%d", totalsByDay[3].Count)
	}
}

func TestGetUniqueDayStrings(t *testing.T) {
	day0 := "2021-12-01"
	day1 := "2021-12-05"
	day2 := "2021-12-08"

	day3 := "2021-12-09"
	data := ReadData("assets/data.txt")
	dataset := BuildDataset(data)
	uniqueDays := dataset.GetUniqueDays()

	if uniqueDays[0] != day0 {
		t.Errorf("unique days string does not match: %s != %s", uniqueDays[0], day0)
	}
	if uniqueDays[1] != day1 {
		t.Errorf("unique days string does not match: %s != %s", uniqueDays[1], day1)
	}
	if uniqueDays[2] != day2 {
		t.Errorf("unique days string does not match: %s != %s", uniqueDays[2], day2)
	}
	if uniqueDays[3] != day3 {
		t.Errorf("unique days string does not match: %s != %s", uniqueDays[3], day3)
	}

}

func TestTopThreeEntries(t *testing.T) {
	// "2021-12-01T07:30:00 46"
	// "2021-12-01T08:00:00 42"
	// "2021-12-08T18:00:00 33"
	t.Fail()
}
func TestLowest90minWindow(t *testing.T) {
	// 31, 41, 54, all from the 5th (as it's the only one with unbroken 90min windows)
	t.Fail()

}
