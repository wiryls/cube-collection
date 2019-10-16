package main

import (
	"log"
	"net/http"
	"os"
	"regexp"
)

func main() {
	addr := "23333"
	if len(os.Args) > 1 && os.Args[1] != "" {
		addr = os.Args[1]
	}
	if regexp.MustCompile(`[0-9]+$`).MatchString(addr) {
		addr = "localhost:" + addr
	}

	server := http.FileServer(http.Dir("./"))
	http.Handle("/", server)

	log.Printf("Server is running on http://%s\n", addr)
	err := http.ListenAndServe(addr, nil)
	if err != nil {
		log.Println(err)
	}
}
