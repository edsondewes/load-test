"use strict"
const { getDb } = require('./dbConnection')

async function sendMessage(appName, data) {
  const app = await findAppByName(appName)
  if (!app) {
    throw new Error('Invalid app')
  }

  const now = new Date();
  const insertResult = await getDb().collection('messages').insertOne({
    appName,
    from: app.phone,
    to: data.to,
    status: 'stored',
    text: data.text,
    createdAt: now,
    updatedAt: now
  })
  const message = insertResult.ops[0]
  await setInteractionLastMessage(message)

  return message
}

function findAppByName(appName) {
  return getDb().collection('apps').findOne({ name: appName })
}

function setInteractionLastMessage(message) {
  return getDb().collection('interactions')
    .updateOne(
      { appName: message.appName, to: message.to },
      {
        $set: {
          lastMessage: message,
          updatedAt: message.updatedAt
        },
        $setOnInsert: { createdAt: message.createdAt },
      },
      {
        upsert: true
      }
    )
}


module.exports = sendMessage;