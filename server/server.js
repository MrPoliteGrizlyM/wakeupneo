var express = require('express');
var path = require('path');
// var cookieParser = require('cookie-parser');

var app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: false }));
// app.use(cookieParser());
app.use(express.static(path.join(__dirname, 'public')));

app.get('/api/login', function(req, res) {
	res.send("Hello from a public endpoint! You don't need to be authenticated to see this.");
});

app.post('/api/register', function(req, res) {
  res.send("Hello from a public endpoint! You don't need to be authenticated to see this.");
});


app.listen(8080, function(err) {
  console.log("running server on from port::" + 8080);
});

module.exports = app;
