import 'dotenv/config'
import express from "express"
import path from "path"
import models, { sequelize } from './src/models/config'

var app = express()

app.use(express.json())
app.use(express.urlencoded({ extended: false }))
app.use(express.static(path.join(__dirname, 'public')))

app.post('/api/login', async (req, res) => {
  const username = req.body.username ? req.body.username : ''
  const password = req.body.password ? req.body.password : ''
  
  await models.User.findOne({
    where: {
       username: username,
       password: password
    }
    }).then((user) => {
    if (!user) {
      return res.json({"response": "error", "description": "User does not exist"})
    }
    res.json({"response": "ok"})
  })

})

app.post('/api/register', async (req, res) => {
  const username = req.body.username
  const password = req.body.password

  if (username && password)
    await models.User.create(
        {
          username: username,
          password: password
        },
      ).then((item) => {
        return res.json({
          "response" : "ok",
          "item" : item
        })
      }).catch((err) => {
        return res.json({
          "response" : "error",
          "description" : err
        })
      })
  else
      return res.json({"response": "error", "description": "No username or password provided"})
})


// Setting up server and socketserver (same port)
let server = app.listen(process.env.PORT, (err) => {
  console.log("running server on from port::" + process.env.PORT)
})
let io = require('socket.io')(server);

// SOCKET STUFF
io.on('connection', (socket) => {
  console.log('a user connected');

  // chat message is the tag from the library
  socket.on('chat message', (msg) => {
    console.log('message: ' + msg);
  });

  io.emit('some event', { someProperty: 'some value', otherProperty: 'other value' }); // This will emit the event to all connected sockets

});

// DB STUFF
sequelize.sync().then(() => {
  app.listen(process.env.DATABASE_PORT, () => {
    console.log(`App database listening on port ${process.env.DATABASE_PORT}!`)
  })
})

module.exports = app