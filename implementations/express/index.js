const express = require('express')
const morgan = require('morgan');
const app = express()
const db = require('./queries')
const port = 55505

app.use(morgan('tiny'));

app.get('/color', db.getColor)

app.listen(port, () => {
  console.log(`App running on port ${port}.`)
})
