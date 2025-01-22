package raindrops

import "fmt"

var factors = [3]int{3, 5, 7}
var sounds = [3]string{"Pling", "Plang", "Plong"}
var sound map[int]string

func Convert(number int) string {
	result := ""
	sound = make(map[int]string)

	for i := range factors {
		sound[factors[i]] = sounds[i]
	}

	for i := range factors {
		if number%factors[i] == 0 {
			result += sound[factors[i]]
		}
	}

	if result == "" {
		result = fmt.Sprint(number)
	}

	return result
}
