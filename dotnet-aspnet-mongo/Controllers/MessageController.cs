using System;
using System.Security.Claims;
using System.Threading.Tasks;
using MessageTest.Model;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace MessageTest.Controllers
{
    [ApiController]
    [Route("/")]
    public class MessageController : ControllerBase
    {
        private readonly MessageService _service;

        public MessageController(MessageService service)
        {
            _service = service;
        }

        [HttpPost]
        [Authorize]
        public async Task<MessageViewModel> Post([FromBody] SaveMessageRequest model)
        {
            var appName = User.FindFirstValue("app_name");
            var message = await _service.SendMessage(appName, model.To, model.Text);

            return MessageViewModel.ToViewModel(message);
        }
    }

    public class SaveMessageRequest
    {
        public string To { get; set; }
        public string Text { get; set; }
    }

    public class MessageViewModel
    {
        public string MessageId { get; set; }
        public string AppName { get; set; }
        public string Text { get; set; }
        public string To { get; set; }
        public string From { get; set; }
        public string Status { get; set; }
        public DateTime CreatedAt { get; set; }
        public DateTime UpdatedAt { get; set; }

        public static MessageViewModel ToViewModel(Message message) => new MessageViewModel
        {
            MessageId = message.Id,
            AppName = message.AppName,
            Text = message.Text,
            To = message.To,
            From = message.From,
            Status = message.Status,
            CreatedAt = message.CreatedAt,
            UpdatedAt = message.UpdatedAt
        };
    }
}
