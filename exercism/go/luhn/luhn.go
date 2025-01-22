package luhn

import (
	"strconv"
	"strings"
	"unicode"
)

// computes Luhn checksum
func Valid(id string) bool {
	id = strings.ReplaceAll(id, " ", "")

	id_len := len(id)
	if id_len < 2 {
		return false
	}

	total := 0
	for i := id_len - 1; i >= 0; i-- {
		if !unicode.IsDigit(rune(id[i])) {
			return false
		}

		d, _ := strconv.Atoi(string(id[i]))

		if (id_len-i)%2 == 1 { // odd number digits
			total += d
		} else { // even number digits
			d *= 2
			if d > 9 {
				d -= 9
			}
			total += d
		}
	}

	return total%10 == 0
}
