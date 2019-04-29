package main

import (
	"fmt"
	"os"
	"strings"
)

// TODO Add documentation.
func main() {
	path, isPathPresent := os.LookupEnv("PATH")

	if isPathPresent {
		pathVar := PathVar{path, strings.Split(path, ":")}
		fmt.Println(pathVar.pretty())
	}

}

// A PathVar represents a Unix $PATH environment variable and its component directories.
type PathVar struct {
	path                string
	directoryComponents []string
}

// Returns the pretty-printed version of the input PATH parameter.
func (path *PathVar) pretty() string {
	var output strings.Builder
	for _, dir := range path.directoryComponents {
		fmt.Fprintln(&output, dir)
	}
	return output.String()
}
