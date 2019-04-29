package main

import "os"
import "strings"
import "fmt"

func main() {
	path, isPathPresent := os.LookupEnv("PATH")

	if isPathPresent {
		values := strings.Split(path, ":")
		fmt.Println("$PATH=", values)
	}

}
