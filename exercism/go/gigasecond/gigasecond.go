package gigasecond

import (
	"time"
)

const gigaSeconds = 1_000_000_000 * time.Second

// Adds 1 gigasecond to any given time and returns time
func AddGigasecond(t time.Time) time.Time {
	return t.Add(gigaSeconds)
}
