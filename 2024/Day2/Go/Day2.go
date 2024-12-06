//failed attempt need to rewrite this!!!.Didnt want to delete the file :(
package main

import (
	"fmt"
	"os"
  "io"
  "strings"
  "strconv"
)

//returns the each line as an array of 2d int arrays,in a file input.txt
func arrayFromFile() [][]int {
  
	file, err := os.Open("input.txt")
  if err != nil{
    fmt.Println("Couldnot open a file",err)
    return nil
  }
  defer file.Close()
  
  var allLevels string

  buffer := make([]byte,1024)
  for{
    n,err := file.Read(buffer)
    if err != nil && err !=io.EOF{
      fmt.Println("Err",err)
      break
    }
    if n == 0 {
      break
    }
    allLevels += string(buffer[:n])//adding n which is a buffer to the allLevels string
  }
  // fmt.Println(allLevels)
  singleLevels := strings.Split(allLevels,"\n")
  var arrayOfLevels [][] int 
  for _,line := range singleLevels{
    fields:= strings.Fields(line)
    //holds current row
    var row[] int

    //convert each field to an interger and strore it to row
    for _,fields := range fields{
      num,err := strconv.Atoi(fields)
      if err != nil{
        fmt.Println("Couldnot convert to string",err)
        continue

      }
      row = append(row, num)
    }
    //this should remove the extra []
    if len(row)> 0{

    arrayOfLevels = append(arrayOfLevels, row)
    }
  }

  return arrayOfLevels
}
//chekck if ascending
func checkArrayAsc(toSortAsc []int) bool {
  //creating atrue deep copy??
  unSorted := append([]int{},toSortAsc...)
  println("ascend value",unSorted)
  //making copy of a copy of a copy of a copy.......
  checkFam:= make([]int,len(unSorted))
  copy(checkFam,unSorted)

	//unoptimal ,try using map or somethingelse
	for i := 0; i < len(unSorted); i++ {
		for j := 0; j < len(unSorted); j++ {
			currentNum := unSorted[i]
			if unSorted[i] < unSorted[j] {
				unSorted[i] = unSorted[j]
				unSorted[j] = currentNum
			}
		}

		// sorted = append(sorted,unSorted[i])
	}
  println("Unsorted:",unSorted)
	for i := 0; i < len(unSorted); i++ {
    fmt.Println("currentiteration in asc",i)

    if(i != len(unSorted)-1){
      fmt.Println("first,second",unSorted[i],unSorted[i+1])
      currentChange := unSorted[i]-unSorted[i+1]
      if(currentChange<0){
        currentChange = currentChange*-1
      }
      fmt.Println("current change:",currentChange)
    if(checkFam[i] == unSorted[i]){
      if(unSorted[i] == unSorted[i+1] && currentChange<1 && currentChange>3){
        return false
      }else{
        continue
      }
      // fmt.Println(checkAscend[i],ascend[i])
    }else{
      // fmt.Println("Out")
      // fmt.Println(checkAscend[i],ascend[i])
      // fmt.Println("Out")
      return false
    }
    }else{
      return true 
    }
  }

  return true

}

//check if descending
func checkArrayDes(ascend []int) bool {
	//unoptimal ,try using map or somethingelse
  checkAscend:= make([]int,len(ascend))
  copy(checkAscend,ascend)
  println("ascend value",ascend)
	for i := 0; i < len(ascend); i++ {
		for j := 0; j < len(ascend); j++ {
			currentNum := ascend[i]
			if ascend[i] > ascend[j] {
				ascend[i] = ascend[j]
				ascend[j] = currentNum
			}
		} // sorted = append(sorted,unSorted[i])
	}


  // fmt.Println("check",checkAscend)
  // fmt.Println("actual",ascend)


	for i := 0; i < len(ascend); i++ {

    if(i != len(ascend)-1){
      currentDiff := ascend[i]-ascend[i+1]
      if(currentDiff<0){
        currentDiff = currentDiff*-1
      }

    if(checkAscend[i] == ascend[i]){
      if(ascend[i] == ascend[i+1] && currentDiff>1 && currentDiff<3){
        return false
      }else{
        continue
      }
      // fmt.Println(checkAscend[i],ascend[i])
    }else{
      // fmt.Println("Out")
      // fmt.Println(checkAscend[i],ascend[i])
      // fmt.Println("Out")
      return false
    }
    }else{
      return true 
    }
  }

  return true
}


func checkAscOrDesc( array[] []int)   {
  //getting all the inner length of the array
    // innerLengths := []int{}
    // for _, innerArray := range array{//use _ if u dont want to use i,this method always takes two parameter and if u dont want to use i,then just use _
    //   innerLengths = append(innerLengths, len(innerArray)) //range of all inner arrays
    // }
  var currentSafe int
  for i:= 0 ; i< len(array); i++{    
    currentFullArray:= array[i]
if(checkArrayDes(currentFullArray) == true  && checkArrayAsc(currentFullArray) == true){
      println("passed \n")
      currentSafe = currentSafe+1
      // fmt.Println("true")
    }else{

      fmt.Println("checkAsc,checkDesc",checkArrayAsc(currentFullArray),checkArrayDes(currentFullArray))
    }
        }
      fmt.Println("currentSafe \n",currentSafe)
}

func check( array[]int)   {
  //getting all the inner length of the array
    // innerLengths := []int{}
    // for _, innerArray := range array{//use _ if u dont want to use i,this method always takes two parameter and if u dont want to use i,then just use _
    //   innerLengths = append(innerLengths, len(innerArray)) //range of all inner arrays
    // }
  var currentSafe int
  for i:= 0 ; i< len(array); i++{    
if(checkArrayDes(array) == true  && checkArrayAsc(array) == true){
      println("passed \n")
      currentSafe = currentSafe+1
      // fmt.Println("true")
    }else{

      fmt.Println("checkAsc,checkDesc",checkArrayAsc(array),checkArrayDes(array))
    }
        }
      fmt.Println("currentSafe \n",currentSafe)
}

func main() {

test :=[]int {7, 6, 4, 2 ,1}

	// firstList := []string{"3","4","2","1","3","3"}

  check(test)
}
