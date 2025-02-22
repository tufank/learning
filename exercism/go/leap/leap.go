package leap

// IsLeapYear determines if a year is leap or not
//   - In every year that is evenly divisible by 4.
//   - Unless the year is evenly divisible by 100, in which case
//     it's only a leap year if the year is also evenly divisible by 400.
func IsLeapYear(year int) bool {
	if (year%400 == 0) || (year%4 == 0 && year%100 != 0) {
		return true
	} else {
		return false
	}

}
