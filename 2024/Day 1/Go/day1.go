package main
import (
	"bufio"
	"fmt"
	"os"
	"strings"
  "strconv"
)

func sortArrayAsc( unSorted[]string) []string{

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

func sortArrayDes( ascend[]string) []string{
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

//function to substract same index numbers and return a new array of differences
func subsameindex(first[]string,second[]string)[]string{

  final:= []string{}

  for i:=0 ; i<len(first);i++{
    firstNum, err1 := strconv.Atoi(first[i])//converting the string to int,using strconv 
    secondNum, err2 := strconv.Atoi(second[i])//strconv take two argument

    if err1 != nil || err2 != nil{
      fmt.Println("Couldnot convert to int")
      return final
    }
    result:= firstNum-secondNum
    if result <0{
      result = result * -1
    }
    final = append(final, strconv.Itoa(result))
  }

  return final
}


// function to sum an array
func addInArray(array[]string) int {
  var Sum int

    //converting string to integers by iterating over it
  for _,str:=range array{
    num,err := strconv.Atoi(str) //converting each number(String) to int,before addition 
    if err != nil{
      fmt.Println(err)
    }
    Sum = Sum + num
  }

  return Sum 
}

//main function to execute

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
 
  sortedFirst := sortArrayAsc(firstColumn)
  sortedSecond := sortArrayAsc(secondColumn)

  diff := subsameindex(sortedFirst,sortedSecond)
  finalOutput := addInArray(diff)
  fmt.Println(finalOutput)

	// Print results
	// fmt.Println("First Column:", firstColumn)
	// fmt.Println("Second Column:", secondColumn)
}
