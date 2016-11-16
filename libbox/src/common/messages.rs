// will be serializable in the future
#[derive(Clone, Debug)]
pub enum Message {
    TestMessage(String),
    Quit,
}
