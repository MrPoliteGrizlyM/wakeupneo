import 'dotenv/config'
import express from "express"
import path from "path"
import models, { sequelize } from './src/entities/config'

var app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(express.static(path.join(__dirname, 'public')));

app.get('/api/login', (req, res) => {
  const username = req.body.username ? req.body.username : '';
  const password = req.body.password ? req.body.password : '';
  
  models.User.findOne({
    where: {
       username: username,
       password: password
    }
    }).then((user) => {
    if (!user) {
       res.json({"response": "error", "description": "User does not exist"})
    }
    res.json({"response": "ok"});
  });;

});

app.post('/api/register', async (req, res) => {
  const username = req.body.username;
  const password = req.body.password;

  if (username && password) {
    await models.User.create(
        {
          username: username,
          password: password
        },
      ).then((item) => {
        res.json({
          "response" : "ok",
          "item" : item
        });
      }).catch((err) => {
        res.json({
          "response" : "error",
          "description" : err
        });
      });
  }
  return res.json({"response": "error", "description": "No username or password provided"})
});


app.listen(process.env.PORT, (err) => {
  console.log("running server on from port::" + process.env.PORT);
});

sequelize.sync().then(() => {
  app.listen(process.env.DATABASE_PORT, () => {
    console.log(`App database listening on port ${process.env.DATABASE_PORT}!`)
  });
});

module.exports = app;
