package main

import "testing"

const TEST_DATA = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`

func TestPart1(t *testing.T) {
	test_data := string(TEST_DATA)
	if r := part1(&test_data); r != 24000 {
		t.Fatalf("Part 1 test expected result of 24000, but got %d", r)
	}
}

func TestPart2(t *testing.T) {
	test_data := string(TEST_DATA)
	if r := part2(&test_data); r != 45000 {
		t.Fatalf("Part 2 test expected result of 45000, but got %d", r)
	}
}
