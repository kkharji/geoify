// use actix::{
//     io::{SinkWrite, WriteHandler},
//     *,
// };
// use actix_codec::Framed;
// use awc::{
//     error::WsProtocolError,
//     ws::{Codec, Frame, Message},
//     BoxedSocket,
// };

// use bytes::Bytes;
// use futures::stream::{SplitSink, StreamExt};
// use std::time::Duration;

// pub type SlackWSClientWriter = SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>;
// pub struct SlackWSClient(SlackWSClientWriter);

// impl Actor for SlackWSClient {
//     type Context = Context<Self>;

//     fn started(&mut self, ctx: &mut Context<Self>) {
//         tracing::info!("Slack Websocket Connection Established! ctx: {ctx:?}");
//         self.hb(ctx);
//     }

//     fn stopped(&mut self, _: &mut Context<Self>) {
//         tracing::info!("Slack Websocket Disconnected!");

//         // Stop application on disconnect
//         System::current().stop();
//     }
// }

// impl SlackWSClient {
//     pub fn new(writer: SlackWSClientWriter) -> Self {
//         Self(writer)
//     }

//     fn hb(&self, ctx: &mut Context<Self>) {
//         ctx.run_later(Duration::new(1, 0), |act, ctx| {
//             act.0.write(Message::Ping(Bytes::from_static(b""))).unwrap();
//             act.hb(ctx);

//             // client should also check for a timeout here, similar to the
//             // server code
//         });
//     }
// }

// impl WriteHandler<WsProtocolError> for SlackWSClient {}

// /// Handle stdin commands
// // impl Handler<cmdhandler::ClientCommand> for SlackWSClient {
// //     type Result = ();

// //     fn handle(&mut self, cmd: cmdhandler::ClientCommand, _ctx: &mut Context<Self>) {
// //         match cmd.handleSelf(&mut self.1) {
// //             Some(content) => {
// //                 let msg = serverhandlers::ServerMsg::new(&self.1.addr(), content);
// //                 self.0.write(Message::Text(msg.toString())).unwrap();
// //             }
// //             None => {}
// //         }
// //     }
// // }

// /// Handle server websocket messages
// impl StreamHandler<Result<Frame, WsProtocolError>> for SlackWSClient {
//     fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
//         if let Ok(Frame::Text(txt)) = msg {
//             tracing::info!("Handling {txt:?}");
//             // serverhandlers::ServerMsg::fromServer(&txt.to_vec()).handleSelf(&mut self.1);
//         }
//     }

//     fn started(&mut self, _ctx: &mut Context<Self>) {
//         tracing::info!("Connected");
//     }

//     fn finished(&mut self, ctx: &mut Context<Self>) {
//         tracing::info!("Server disconnected");
//         ctx.stop()
//     }
// }

// // impl SlackWSClient {
// //     fn hb(&self, ctx: &mut Context<Self>) {
// //         ctx.run_later(Duration::new(1, 0), |act, ctx| {
// //             act.0.write(Message::Ping(Bytes::from_static(b""))).unwrap();
// //             act.hb(ctx);

// //             // client should also check for a timeout here, similar to the
// //             // server code
// //         });
// //     }
// // }
