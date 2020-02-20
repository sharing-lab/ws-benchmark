package main

import (
	"fmt"
	"log"

	"github.com/fasthttp/router"
	"github.com/valyala/fasthttp"
)

// Index is the index handler
func Index(ctx *fasthttp.RequestCtx) {
	fmt.Fprint(ctx, "Welcome!\n")
}

func main() {
	r := router.New()
	r.GET("/", Index)

	log.Fatal(fasthttp.ListenAndServe(":55518", r.Handler))
}


