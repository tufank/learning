package diffsquares

import "math"

// sum of first n numbers is n*(n+1)/2
func SquareOfSum(n int) int {
	return int(math.Pow(float64(n*(n+1)/2), 2))
}

// sum of squares of first n numbers is n^3/3 + n^2/2 + n/6
func SumOfSquares(n int) int {
	return int(math.Pow(float64(n), 3)/3 + math.Pow(float64(n), 2)/2 + float64(n)/6)
}

func Difference(n int) int {
	return int(math.Abs(float64(SquareOfSum(n)) - float64(SumOfSquares(n))))
}
