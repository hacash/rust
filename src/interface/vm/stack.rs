/**

i64  bit 8  byte max                       922_33720368_54775807
i128 bit 16 byte max 1701411_83460469_23173168_73037158_84105727

*/

pub trait StackItem {
    fn read(&self) -> &[u8];
    fn write(&mut self, _: &[u8]) -> Option<Error>;
}

pub trait Stack {

    fn push(&mut self, item: Box<dyn StackItem>) -> Option<Error>;
    fn pop(&mut self) -> Result<Box<dyn StackItem>, Error>;
    fn peek(&self) -> Option<Weak<dyn StackItem>>;

    fn height(&self) -> u16; // max height 65535

}