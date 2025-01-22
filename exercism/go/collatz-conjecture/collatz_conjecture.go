package collatzconjecture

import "errors"

func CollatzConjecture(n int) (int, error) {
	steps := 0
	if n < 1 {
		return -1, errors.New("You must start with a positive integer")
	}
	for n > 1 {
		if n%2 == 0 {
			n = n / 2
		} else {
			n = n*3 + 1
		}
		steps += 1
	}
	return steps, nil
}
