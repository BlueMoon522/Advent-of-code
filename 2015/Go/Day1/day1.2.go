/*
--- Day 1: Not Quite Lisp ---
Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh out! To save Christmas, he needs you to collect fifty stars by December 25th.

Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Here's an easy puzzle to warm you up.

Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing. He starts on the ground floor (floor 0) and then follows the instructions one character at a time.

An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.

The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.

For example:

(()) and ()() both result in floor 0.
((( and (()(()( both result in floor 3.
))((((( also results in floor 3.
()) and ))( both result in floor -1 (the first basement level).
))) and )())()) both result in floor -3.
To what floor do the instructions take Santa?

Your puzzle answer was 74.

--- Part Two ---
Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.

For example:

) causes him to enter the basement at character position 1.
()()) causes him to enter the basement at character position 5.
What is the position of the character that causes Santa to first enter the basement?

Your puzzle answer was 1795.

Both parts of this puzzle are complete! They provide two gold stars: **
*/
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
