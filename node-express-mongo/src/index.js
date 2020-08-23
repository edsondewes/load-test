"use strict"
const express = require('express')
const app = express()
const jwt = require('jsonwebtoken')
const { connectDb } = require('./dbConnection')
const sendMessage = require('./sendMessage')

app.use(express.json())

app.post('/', async (req, res) => {
  try {
    const token = await auth(req);
    if (!token) {
      return res.status(401).send();
    }

    const message = await sendMessage(token.appName, req.body);
    return res.status(200).json(toMessageViewModel(message))
  } catch (err) {
    return res.status(500).send(err.message)
  }
})

function auth(req) {
  if (!req.headers.authorization) {
    return null;
  }

  return new Promise((resolve) => {
    jwt.verify(
      req.headers.authorization.replace('Bearer ', ''),
      'my super secret test key',
      function (_, decoded) {
        resolve({ appName: decoded.app_name })
      });
  });
}

function toMessageViewModel(message) {
  return {
    messageId: message._id,
    appName: message.appName,
    text: message.text,
    to: message.to,
    from: message.from,
    status: message.status,
    createdAt: message.createdAt,
    updatedAt: message.updatedAt,
  }
}

const port = 3000
connectDb().then(() => {
  app.listen(port, () => {
    console.log(`Listening at http://localhost:${port}`)
  })
})