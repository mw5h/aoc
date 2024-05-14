package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func maxN(m []int, v int) {
	for i, num := range m {
		if v > num {
			m[i] = v
			v = num
		}
	}
}

func sumN(m []int) int {
	v := 0
	for _, num := range m {
		v += num
	}
	return v
}

func partN(data *string, n int) int {
	scanner := bufio.NewScanner(strings.NewReader(*data))

	max := make([]int, n)
	this := 0

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			maxN(max, this)
			this = 0
			continue
		}

		v, e := strconv.Atoi(line)
		if e != nil {
			panic(e)
		}
		this += v
	}
	maxN(max, this)

	return sumN(max)
}

func part1(data *string) int {
	return partN(data, 1)
}

func part2(data *string) int {
	return partN(data, 3)
}

func main() {
	raw, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	data := string(raw[:])
	fmt.Printf("part1: %d\npart2: %d\n", part1(&data), part2(&data))
}
