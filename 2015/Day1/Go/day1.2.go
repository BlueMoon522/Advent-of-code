package main
import (
	"fmt"
	"io"
	"log"
	"os"
)

func getInputStrings() string {
	content, err := os.Open("input.txt") //read a file
	if err != nil {
		log.Fatal(err) //if file doesnot exist
	}
	defer content.Close()

	//creating a buffer to store file content

	var result string

	buffer := make([]byte, 1024) //buffersize
	//reading the file content into a buffer
	for {
		n, err := content.Read(buffer)
		if err != nil && err != io.EOF {
			fmt.Println("Error", err)
			break
		}
		if n == 0 {
			break
		}
		result += string(buffer[:n]) //convert bytes to string and append
	}

	return result

}

//changing the string to array of strings
func getStringChar() []string {

  result := getInputStrings()
	var chars []string
	for _, r := range result {
		chars = append(chars, string(r))
	}
	return chars
}

func main() {
	input := getStringChar()
  basementIndex:= 0
	currentFloor := 0
	for i := 0; i < len(input); i++ {
    fmt.Println(currentFloor)
    if(currentFloor != -1){
		if input[i] == "(" {
			currentFloor = currentFloor + 1
		} else if input[i] == ")" {
			currentFloor = currentFloor - 1
		}
    }else{
      //actual answer is from i not i+1
      // i have no idea why
        basementIndex= i+1
        fmt.Println(basementIndex)
      break
    }
	}
	fmt.Println(currentFloor)
}

//answer is 74
//second answer is probably 1796
