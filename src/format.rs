use crate::misc::DbgFmtFn;
use std::fmt::*;

#[allow(dead_code)]
pub enum FormatType {
    Debug,
    Display,
    Emoji,
    Short,
    Long,
}

pub trait Formattable<T = Self>: Debug
where
    T: Debug,
{
    fn format_emoji(&self, f: &mut Formatter<'_>) -> Result;
    fn format_short(&self, f: &mut Formatter<'_>) -> Result;
    fn format_long(&self, f: &mut Formatter<'_>) -> Result;

    fn format(&self, format_type: &FormatType, f: &mut Formatter<'_>) -> Result {
        match format_type {
            FormatType::Debug => Debug::fmt(self, f),
            FormatType::Display => {
                //Display::fmt(self,f)
                Debug::fmt(self, f)
            }
            FormatType::Emoji => self.format_emoji(f),
            FormatType::Short => self.format_short(f),
            FormatType::Long => self.format_long(f),
        }
    }

    fn format_string(&self, format_type: &FormatType) -> String {
        let format_str = format!("{:?}", DbgFmtFn(|f| self.format(format_type, f)));
        format_str
    }
}
