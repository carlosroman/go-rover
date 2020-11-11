package planets

// Planet represents a mars
type Planet interface {
	// UpperRight returns the top right co-ordinate
	UpperRight() (x, y uint8)
	// IsValidLocation check to see if location valid
	IsValidLocation(x, y uint8) (valid bool)
	// MarkLocation returns true if never marked before
	MarkLocation(x, y uint8) (successful bool)
}
