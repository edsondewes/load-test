using System;
using System.Threading.Tasks;

namespace MessageTest.Model
{
    public class MessageService
    {
        private readonly MongoDbRepository _repository;

        public MessageService(MongoDbRepository repository)
        {
            _repository = repository;
        }

        public async Task<Message> SendMessage(string appName, string to, string text)
        {
            var app = await _repository.FindAppByName(appName);
            if (app is null)
            {
                throw new Exception("Invalid app");
            }

            var now = DateTime.Now;
            var message = new Message
            {
                AppName = appName,
                From = app.Phone,
                To = to,
                Status = "stored",
                Text = text,
                CreatedAt = now,
                UpdatedAt = now
            };

            await _repository.SaveMessage(message);
            await _repository.SetInteractionLastMessage(message);

            return message;
        }
    }
}