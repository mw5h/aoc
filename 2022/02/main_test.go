package main

import "testing"

const TEST_DATA = `A Y
B X
C Z`

func TestMain(t *testing.T) {
	var tests = []struct {
		name     string
		f        func(*string) int
		expected int
	}{
		{"part1", part1, 15},
		{"part2", part2, 12},
	}

	test_data := string(TEST_DATA)
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if r := tt.f(&test_data); r != tt.expected {
				t.Errorf("got %d, expected %d", r, tt.expected)
			}
		})
	}
}
