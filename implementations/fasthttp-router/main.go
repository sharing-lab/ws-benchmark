package main

import (
	"fmt"
	"log"
	"database/sql"
	"encoding/json"

	"github.com/fasthttp/router"
	"github.com/valyala/fasthttp"
	_ "github.com/lib/pq"
)

var (
	strContentType = []byte("Content-Type")
	strApplicationJSON = []byte("application/json")
)

func doJSONWrite(ctx *fasthttp.RequestCtx, code int, obj interface{}) {
	ctx.Response.Header.SetCanonical(strContentType, strApplicationJSON)
	ctx.Response.SetStatusCode(code)
	start := time.Now()
	if err := json.NewEncoder(ctx).Encode(obj); err != nil {
		elapsed := time.Since(start)
		logrus.Errorfp("", elapsed, err.Error(), obj)
		ctx.Error(err.Error(), fasthttp.StatusInternalServerError)
	}
}

// Index is the index handler
func Index(ctx *fasthttp.RequestCtx, db *sql.DB) {
	//fmt.Fprint(ctx, "Welcome!\n")
	doJSONWrite(ctx, getProducts(db))
}

func main() {
	db, err = sql.Open("postgres", "user=postgres dbname=test_db host=127.0.0.1 sslmode=disable")
	if err != nil {
		log.Fatal(err)
	}

	r := router.New()
	r.GET("/", Index(db))

	log.Fatal(fasthttp.ListenAndServe(":55518", r.Handler))
}


