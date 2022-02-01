use serenity::builder::CreateEmbed;
use std::string::ToString;

fn _info_box(text: String) -> CreateEmbed {
    let mut e = CreateEmbed::default();
    e.color((255, 220, 0));
    e.field("**INFO**", text, false);
    e
}

fn _error_box(text: String) -> CreateEmbed {
    let mut e = CreateEmbed::default();
    e.color((255, 0, 0));
    e.field("**ERROR**", text, false);
    e
}

fn _text_box(text: String) -> CreateEmbed {
    let mut e = CreateEmbed::default();
    e.color((0, 255, 0));
    e.field("**OK**", text, false);
    e
}

pub trait CreateEmbedExt {
    fn info_box<T: ToString>(text: T) -> Self;

    fn error_box<T: ToString>(text: T) -> Self;

    fn success_box<T: ToString>(text: T) -> Self;

    fn fields_chunked_fmt<'a, T, F>(
        &mut self,
        content: &'a [T],
        fmt: F,
        title: &str,
        inline: bool,
        count: usize,
    ) -> &mut Self
    where
        F: Fn(&T) -> String;

    fn fields_chunked<'a, T>(
        &mut self,
        content: &'a [T],
        title: &str,
        inline: bool,
        count: usize,
    ) -> &mut Self
    where
        T: ToString;
}

impl CreateEmbedExt for CreateEmbed {
    fn info_box<T: ToString>(text: T) -> Self {
        let text = text.to_string();
        _info_box(text)
    }

    fn error_box<T: ToString>(text: T) -> Self {
        let text = text.to_string();
        _error_box(text)
    }

    fn success_box<T: ToString>(text: T) -> Self {
        let text = text.to_string();
        _text_box(text)
    }

    fn fields_chunked_fmt<'a, T, F>(
        &mut self,
        content: &'a [T],
        fmt: F,
        title: &str,
        inline: bool,
        count: usize,
    ) -> &mut Self
    where
        F: Fn(&T) -> String,
    {
        let chunks = content.chunks(count);
        for c in chunks {
            let field_text = c.iter().map(|t| fmt(t)).collect::<Vec<_>>().join("\n");
            self.field(title, field_text, inline);
        }

        self
    }

    fn fields_chunked<'a, T>(
        &mut self,
        content: &'a [T],
        title: &str,
        inline: bool,
        count: usize,
    ) -> &mut CreateEmbed
    where
        T: ToString,
    {
        CreateEmbed::fields_chunked_fmt(self, content, ToString::to_string, title, inline, count)
    }
}
