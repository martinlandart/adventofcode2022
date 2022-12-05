package main

import (
	"os"
	"strconv"
	"strings"
)

func main() {
	content, err := os.ReadFile("./data")
	if err != nil {
		panic(err)
	}

	elves := strings.Split(string(content), "\n\n")

	var elvesSlice [][]uint64
	for _, elf := range elves {
		calories := strings.Split(elf, "\n")

		var caloriesSlice []uint64
		for _, calorie := range calories {
			if len(calorie) == 0 {
				continue
			}
			c, err := strconv.ParseUint(calorie, 10, 64)
			if err != nil {
				panic(err)
			}
			caloriesSlice = append(caloriesSlice, c)
		}
		elvesSlice = append(elvesSlice, caloriesSlice)
	}

	println("most calories:", MostCalories(elvesSlice))
	println("top three most calories:", TopThreeMostCalories(elvesSlice))
}

func MostCalories(elves [][]uint64) uint64 {
	most := uint64(0)
	for _, elf := range elves {
		elfSum := sum(elf)
		if elfSum > most {
			most = uint64(elfSum)
		}
	}
	return most
}

func TopThreeMostCalories(elves [][]uint64) uint64 {
	top := [3]uint64{0, 0, 0}
	lowBound := 0

	for _, elf := range elves {
		elfSum := sum(elf)
		println(elfSum)

		if elfSum > top[lowBound] {
			top[lowBound] = elfSum
		}

		lowBound = minIndex(top)
	}
	println("top", top[0], top[1], top[2])

	return top[0] + top[1] + top[2]
}

func minIndex(arr [3]uint64) int {
	lowest := 0

	for i, v := range arr {
		if v < arr[lowest] {
			lowest = i
		}
	}
	return lowest
}

func sum(elf []uint64) uint64 {
	sum := uint64(0)
	for _, calories := range elf {
		sum += calories
	}
	return sum
}
