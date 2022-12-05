package main

import "testing"

func Test(t *testing.T) {
	input := [][]uint64{
		{
			1000, 2000, 3000,
		},
		{
			4000,
		},
		{
			5000, 6000,
		},
		{
			7000, 8000, 9000,
		},
		{
			10000,
		},
	}

	t.Run("most calories", func(t *testing.T) {
		got := MostCalories(input)

		want := uint64(24000)
		if got != want {
			t.Errorf("got %v, want %v", got, want)
		}
	})

	t.Run("top three most calories", func(t *testing.T) {
		got := TopThreeMostCalories(input)

		want := uint64(24000 + 11000 + 10000)
		if got != want {
			t.Errorf("got %v, want %v", got, want)
		}
	})
}
