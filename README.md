# slack-geoify.rs

Microservice-based slack bot that provide time and weather information for a given city.

```
Slack API --New Message Notification--> Bot {
  -> Message
  -> NLP (todo)
  -> Command
  -> Message
} --Reply--> Slack API
```

## Components and Responsibilities

- Bot
  - Handle connection to Slack.
  - Handle Service Registrations.
  - Receive messages from Slack.
  - Process Messages.
  - Route Messages to a service.
  - Send replies to Slack
- NLP Processor (Process Messages)
- Time (Get Time)
- Weather (Get Weather)

