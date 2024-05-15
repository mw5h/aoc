package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type RoShamBo int

const (
	Rock     = 1
	Paper    = 2
	Scissors = 3
)

func determine_winner(them RoShamBo, us RoShamBo) int {
	if them == us {
		return 3
	}
	if us == them+1 || (us == Rock && them == Scissors) {
		return 6
	}
	return 0
}

func part1(data *string) int {
	scanner := bufio.NewScanner(strings.NewReader(*data))
	decoder := map[string]RoShamBo{"A": Rock, "B": Paper, "C": Scissors, "X": Rock, "Y": Paper, "Z": Scissors}
	points := 0

	for scanner.Scan() {
		line := scanner.Text()

		round := strings.Split(line, " ")
		them := decoder[round[0]]
		us := decoder[round[1]]
		points += determine_winner(them, us) + int(us)
	}
	return points
}

func decode(them RoShamBo, result string) RoShamBo {
	switch result {
	case "X":
		if them == Rock {
			return Scissors
		} else {
			return them - 1
		}
	case "Y":
		return them
	case "Z":
		if them == Scissors {
			return Rock
		} else {
			return them + 1
		}

	}
	panic("unexpected input")
}

func part2(data *string) int {
	scanner := bufio.NewScanner(strings.NewReader(*data))
	decoder := map[string]RoShamBo{"A": Rock, "B": Paper, "C": Scissors}
	points := 0

	for scanner.Scan() {
		line := scanner.Text()

		round := strings.Split(line, " ")
		them := decoder[round[0]]
		us := decode(them, round[1])
		points += determine_winner(them, us) + int(us)
	}
	return points
}

func main() {
	raw, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	data := string(raw[:])
	fmt.Printf("part1: %d\npart2: %d\n", part1(&data), part2(&data))
}
