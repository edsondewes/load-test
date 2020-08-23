"use strict"
const MongoClient = require('mongodb').MongoClient;

let client
async function connectDb() {
  client = await MongoClient.connect('mongodb://localhost:27017/test-req', { useUnifiedTopology: true })
}

function getDb() {
  return client.db();
}


module.exports = { connectDb, getDb }