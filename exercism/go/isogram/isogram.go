package isogram

import "unicode"

func IsIsogram(word string) bool {
	count := map[rune]int{}
	isogram := true

	for _, r := range word {
		if unicode.IsLetter(r) {
			count[unicode.ToLower(r)] += 1
		}
	}

	for _, c := range count {
		if c > 1 {
			isogram = false
		}
	}
	return isogram
}
