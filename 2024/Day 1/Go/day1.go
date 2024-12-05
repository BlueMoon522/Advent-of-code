package main
import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func sortArrayAsc( unSorted[]int) []int {

  //unoptimal ,try using map or somethingelse
  for i:= 0 ; i < len(unSorted) ;i++{
  for j:= 0 ; j < len(unSorted) ;j++{
    currentNum := unSorted[i]
if(unSorted[i]< unSorted[j]){
        unSorted[i] = unSorted[j] 
        unSorted[j]= currentNum
      }
      }
    
        // sorted = append(sorted,unSorted[i])
    }

  return unSorted
  }
 //test function

func sortArrayDes( ascend[]int) []int {
  //unoptimal ,try using map or somethingelse
  for i:= 0 ; i < len(ascend) ;i++{
  for j:= 0 ; j < len(ascend) ;j++{
    currentNum := ascend[i]
if(ascend[i]> ascend[j]){
        ascend[i] = ascend[j] 
        ascend[j]= currentNum
      }
      }   // sorted = append(sorted,unSorted[i])
    }

  return ascend
  }


func main()  {
  	// Open the file
	file, err := os.Open("input.txt")//read a file
  //incase error happens
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()//always close a file after opening and viewing it

	// Initialize slices for columns
	var firstColumn, secondColumn []string //making two array of strings to store first and the last column

	// Read file line by line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		// Split each line into words
		words := strings.Fields(scanner.Text())
		if len(words) == 2 {
			firstColumn = append(firstColumn, words[0])
			secondColumn = append(secondColumn, words[1])
		}
	}

	// Check for scanner errors
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
 
  number:= []int{
32, 5, 48, 20, 14, 2, 35, 47, 11, 29, 18, 50, 37, 6, 9, 45, 25, 4, 22, 41, 12, 30, 46, 1, 17, 40, 21, 34, 3, 7, 8, 33, 38, 36, 16, 10, 19, 24, 27, 15, 13, 42, 26, 28, 43, 39, 23, 31, 49, 44}
  num2:= []int{
32, 5, 48, 20, 14, 2, 35, 47, 11, 29, 18, 50, 37, 6, 9, 45, 25, 4, 22, 41, 12, 30, 46, 1, 17, 40, 21, 34, 3, 7, 8, 33, 38, 36, 16, 10, 19, 24, 27, 15, 13, 42, 26, 28, 43, 39, 23, 31, 49, 44}
  sortedFirst := sortArrayDes(number)
  sortedSecond := sortArrayAsc(num2)
  fmt.Println("Descending:",sortedFirst) 
  fmt.Println("Asecending:",sortedSecond)
	// Print results
	// fmt.Println("First Column:", firstColumn)
	// fmt.Println("Second Column:", secondColumn)
}
