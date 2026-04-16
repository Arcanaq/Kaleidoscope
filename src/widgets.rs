use crate::Context;
use tl::{HTMLTag, Parser};
use xilem::palette;
use xilem::style::Style;
use xilem::view::{label, AnyFlexChild, FlexExt};

pub(crate) fn to_widget(tag:&HTMLTag, parser:&Parser) ->Option<AnyFlexChild<Context>>{
    match tag.name().as_utf8_str().as_ref(){
        "p"|"h1"|"h2"|"h3"|"h4"|"h5"|"h6"=>Some(
            label(tag.inner_html(parser)).into_any_flex()
        ),
        "a"=>Some(
            label(tag.inner_html(parser)).color(palette::css::LIGHT_BLUE).into_any_flex()
        ),
        unknown => {/*println!("{}", unknown));*/None}
    }
}