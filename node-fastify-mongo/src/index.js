"use strict"
const fastify = require('fastify')()
const jwt = require('jsonwebtoken')
const { connectDb } = require('./dbConnection')
const sendMessage = require('./sendMessage')

fastify.post('/', async (req, reply) => {
  try {
    const token = await auth(req);
    if (!token) {
      reply.code(401).send()
    } else {
      const message = await sendMessage(token.appName, req.body);
      return toMessageViewModel(message)
    }
  } catch (err) {
    reply.code(500).send(err.message)
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

const port = 3000;
connectDb().then(() => {
  fastify.listen(port, '0.0.0.0', (_, address) => {
    console.log(`Listening at ${address}`)
  })
})