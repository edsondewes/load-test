using System.Threading.Tasks;
using MongoDB.Bson.Serialization.Conventions;
using MongoDB.Driver;

namespace MessageTest.Model
{
    public class MongoDbRepository
    {
        private readonly IMongoCollection<App> _apps;
        private readonly IMongoCollection<Interaction> _interactions;
        private readonly IMongoCollection<Message> _messages;

        public MongoDbRepository()
        {
            var conventionPack = new ConventionPack { new CamelCaseElementNameConvention() };
            ConventionRegistry.Register("camelCase", conventionPack, t => true);

            var client = new MongoClient("mongodb://localhost:27017");
            var database = client.GetDatabase("test-req");

            _apps = database.GetCollection<App>("apps");
            _interactions = database.GetCollection<Interaction>("interactions");
            _messages = database.GetCollection<Message>("messages");
        }

        public Task<App> FindAppByName(string appName)
        {
            return _apps.Find(a => a.Name == appName).FirstOrDefaultAsync();
        }

        public Task SetInteractionLastMessage(Message message)
        {
            var update = Builders<Interaction>.Update
                .Set(i => i.LastMessage, message)
                .Set(i => i.UpdatedAt, message.UpdatedAt)
                .SetOnInsert(i => i.CreatedAt, message.CreatedAt);

            return _interactions.UpdateOneAsync(
                i => i.AppName == message.AppName && i.To == message.To,
                update,
                new UpdateOptions { IsUpsert = true }
                );
        }

        public Task SaveMessage(Message message)
        {
            return _messages.InsertOneAsync(message);
        }
    }
}