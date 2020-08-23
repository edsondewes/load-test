"use strict"
const { App, Interaction, Message } = require('./dbConnection')

async function sendMessage(appName, data) {
  const app = await findAppByName(appName)
  if (!app) {
    throw new Error('Invalid app')
  }

  const message = await Message.create({
    appName,
    from: app.phone,
    to: data.to,
    status: 'stored',
    text: data.text,
  })
  await setInteractionLastMessage(message)

  return message
}

function findAppByName(appName) {
  return App.findOne({ name: appName })
    .lean()
    .exec()
}

function setInteractionLastMessage(message) {
  return Interaction
    .findOneAndUpdate(
      { appName: message.appName, to: message.to },
      {
        $set: {
          lastMessage: message,
        },
      },
      {
        upsert: true
      }
    )
    .lean()
    .exec()
}


module.exports = sendMessage;