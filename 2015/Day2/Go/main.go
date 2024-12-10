package main

import (
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
)

func getInput() string {

	content, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer content.Close()

	var result strings.Builder

	buffer := make([]byte, 1024)

	for {
		n, err := content.Read(buffer)
		if err != nil && err != io.EOF {
			fmt.Println(err)
			break
		}
		if n == 0 {
			break
		}
		result.Write(buffer[:n])
	}

	return result.String()
}

func solve(l int, w int, h int) (int, int) {

	extra := 0

	if (l*w) <= (w*h) && (l*w) <= (h*l) {
		extra = l * w
	} else if (w*h) <= (l*w) && (w*h) <= (h*l) {
		extra = w * h
	} else {

		extra = h * l
	}
	area := (2 * l * w) + (2 * w * h) + (2 * h * l)

	return area, extra
}

func main() {

	inputs := getInput()
  fmt.Println("Input ",inputs)
	//three arrays for three dimensions
	var l_array, w_array, h_array []int

	//split the strings into rows
	//trimming white space
	rows := strings.Split(strings.TrimSpace(inputs),"\n")

	//process each row
	for _, row := range rows {
		//split into individual numbers
		//split the rows,which now only holds single line,and each num is separated by *
		numbers := strings.Split(row, "*")
  fmt.Println("numbers",numbers)

		//add into individual arrays based on the length
		if len(numbers) == 3 {

			num1, _ := strconv.Atoi(numbers[0])
			num2, _ := strconv.Atoi(numbers[1])
			num3, _ := strconv.Atoi(numbers[2])
			l_array = append(l_array, num1)
			w_array = append(w_array, num2)
			h_array = append(h_array, num3)
		} else {
			fmt.Println("No length of 3")
			return
		}
	}

  fmt.Println(l_array,w_array,h_array)

  total := 0
  for i:= 0 ; i<len(l_array);i++{

    area,extra:=solve(l_array[i],w_array[i],h_array[i])
    fmt.Println("Area,extra",area,extra)

    total += area+extra



  }
  fmt.Println(total)

	// area,extra :=solve(1,1,10)
	// fmt.Println("Area,Extra",area,extra)
	fmt.Println("Hello world")

}
