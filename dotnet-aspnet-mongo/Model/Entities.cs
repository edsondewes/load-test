using System;
using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

namespace MessageTest.Model
{
    public class App
    {
        [BsonId]
        [BsonRepresentation(BsonType.ObjectId)]
        public string Id { get; set; }
        public string Name { get; set; }
        public string Phone { get; set; }
    }

    public class Interaction
    {
        [BsonId]
        [BsonRepresentation(BsonType.ObjectId)]
        public string Id { get; set; }
        public string AppName { get; set; }
        public string To { get; set; }
        public Message LastMessage { get; set; }
        public DateTime CreatedAt { get; set; }
        public DateTime UpdatedAt { get; set; }
    }

    public class Message
    {
        [BsonId]
        [BsonRepresentation(BsonType.ObjectId)]
        public string Id { get; set; }
        public string AppName { get; set; }
        public string Text { get; set; }
        public string To { get; set; }
        public string From { get; set; }
        public string Status { get; set; }
        public DateTime CreatedAt { get; set; }
        public DateTime UpdatedAt { get; set; }
    }
}