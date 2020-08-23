"use strict"
const mongoose = require('mongoose')

const appSchema = new mongoose.Schema(
  {
    name: {
      type: String,
      required: true,
    },
    phone: {
      type: String,
      required: true,
    }
  },
  {
    timestamps: true,
    versionKey: false,
  }
)

const interactionSchema = new mongoose.Schema(
  {
    appName: {
      type: String,
      required: false,
    },
    to: {
      type: String,
      required: true,
    },
    lastMessage: {
      type: Object,
      required: true,
    }
  },
  {
    timestamps: true,
    versionKey: false,
  }
)

const messageSchema = new mongoose.Schema(
  {
    appName: {
      type: String,
      required: true,
    },
    to: {
      type: String,
      required: true,
    },
    from: {
      type: String,
      required: true,
    },
    status: {
      type: String,
      required: true,
    },
    text: {
      type: String,
      required: true
    },
  },
  {
    timestamps: true,
    versionKey: false,
  }
)

const App = mongoose.model('App', appSchema);
const Interaction = mongoose.model('Interaction', interactionSchema);
const Message = mongoose.model('Message', messageSchema);

mongoose.set('useFindAndModify', false);
mongoose.connect('mongodb://localhost:27017/test-req', { useNewUrlParser: true, useUnifiedTopology: true });

module.exports = { App, Interaction, Message }