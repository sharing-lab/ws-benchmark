package main

import (
	"github.com/savsgio/atreugo/v10"
)

func main() {
	config := atreugo.Config{
		Addr: "0.0.0.0:55517",
	}
	server := atreugo.New(config)

	server.GET("/", func(ctx *atreugo.RequestCtx) error {
		return ctx.TextResponse("Hello World")
	})

	if err := server.ListenAndServe(); err != nil {
		panic(err)
	}
}
