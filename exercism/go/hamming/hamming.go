package hamming

import "errors"

func Distance(a, b string) (int, error) {
	hamDistance := 0

	if len(a) != len(b) {
		return -1, errors.New("Strings have different lengths")
	}
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			hamDistance += 1
		}
	}
	return hamDistance, nil
}
