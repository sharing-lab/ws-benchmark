package main

import (
	"database/sql"
)

type color struct {
	ID    int     `json:"id"`
	Code  string  `json:"code"`
	Name  string  `json:"name"`
}

func getProducts(db *sql.DB, start, count int) ([]color, error) {
	rows, err := db.Query(
		"SELECT id, code, name FROM color")

	if err != nil {
		return nil, err
	}

	defer rows.Close()

	colors := []color{}

	for rows.Next() {
		var p color
		if err := rows.Scan(&p.ID, &p.Code, &p.Name); err != nil {
			return nil, err
		}
		colors = append(colors, p)
	}

	return colors, nil
}

