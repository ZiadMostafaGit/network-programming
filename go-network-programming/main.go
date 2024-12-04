package main

import (
	"fmt"
	"net"
	"time"
)

// [fd64:6d6c:8763:7f00:c51b:b17d:e936:e37e]
func main() {

	listner, err := net.Listen("tcp", "[::]:5000")
	if err != nil {
		fmt.Printf("Error starting the server", err)
		return
	}

	defer listner.Close()
	fmt.Printf("the server is running on port 5000")
	for {

		req, err := listner.Accept()
		if err != nil {
			fmt.Printf("Error with the requiste it got droped")
			continue
		}
		fmt.Printf("New connection received from %s\n", req.RemoteAddr().String())
		go handleRequiste(req)

	}

}

func handleRequiste(req net.Conn) {

	defer req.Close()

	now := time.Now().Format("2006-01-02 15:04:05")

	body := fmt.Sprintf("Hi Iam Ziad Mostafa And This Is My Tcp Server \nThe Current time is %s", now)
	response := fmt.Sprintf(
		"HTTP/1.1 200 OK\r\n"+
			"Content-Type: text/plain; charset=UTF-8\r\n"+
			"Content-Length: %d\r\n"+
			"Connection: close\r\n\r\n"+
			"%s",
		len(body),
		body,
	)

	_, err := req.Write([]byte(response))
	if err != nil {
		fmt.Printf("Failed to send the response: %v\n", err)
	}

}
