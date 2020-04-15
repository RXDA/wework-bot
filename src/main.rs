
mod api;
fn main(){
    let message = api::request::SimpleMessage{
        msg_type: "text".to_owned(),
        text: api::request::Text{
            content: "hello world".to_owned(),
        }
    };
    let a = api::request::request(message);
    println!("{}", a)
}
