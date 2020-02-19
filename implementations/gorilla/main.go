package main

func main() {
	a := App{}
	a.Initialize("postgres", "", "test_db")

	a.Run(":8080")
}

