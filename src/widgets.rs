use crate::Context;
use tl::{HTMLTag, Parser};
use xilem::{palette, Color};
use xilem::style::{Padding, Style};
use xilem::view::{button, inline_prose, label, sized_box, text_button, AnyFlexChild, FlexExt};

pub(crate) fn to_widget(tag:&HTMLTag, parser:&Parser) ->Option<AnyFlexChild<Context>>{
    match tag.name().as_utf8_str().as_ref(){
        "p"|"h1"|"h2"|"h3"|"h4"|"h5"|"h6"=>Some(
            inline_prose(tag.inner_text(parser)).into_any_flex()
        ),
        "a"=>Some(
            button(label(tag.inner_text(parser)).color(palette::css::LIGHT_BLUE), |cx:&mut Context|{

            }).into_any_flex()
        ),
        "code"=>Some(
            sized_box(label(tag.inner_text(parser)))
                .padding(Padding::all(10.))
                .background_color(Color::from_rgba8(50, 50, 50, 100))
                .corner_radius(5.)
                .into_any_flex()
        ),
        "hr"=>Some(
            label("___________________________________________________________________________").into_any_flex()
        ),
        unknown => {println!("{}", unknown);None}
    }
}