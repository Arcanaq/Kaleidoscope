use crate::AppData;
use scraper::{ElementRef, Selector};
use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style;
use xilem::view::{flex_col, flex_row, prose, sized_box, AnyFlexChild, FlexExt};
use xilem::Color;

pub fn heading(element: ElementRef) -> AnyFlexChild<AppData> {
    let header_num = element
        .value()
        .name()
        .to_string()
        .chars()
        .nth(1)
        .unwrap()
        .to_string()
        .parse::<u8>()
        .unwrap();
    prose(element.inner_html())
        .text_size(32.-(header_num as f32-1.)*2.)
        .text_color(Color::BLACK).into_any_flex()
}

pub fn paragraph(element: ElementRef) -> AnyFlexChild<AppData> {
    prose(element.inner_html())
        .text_color(Color::BLACK)
        .into_any_flex()
}

pub fn table(element: ElementRef) -> AnyFlexChild<AppData> {
    let mut widget = Vec::new();
    for tr in element.select(&Selector::parse("tr").unwrap()){
        let mut tr_widget = Vec::new();
        for element in tr.child_elements(){
            tr_widget.push(
                sized_box(prose(element.inner_html()).text_color(Color::BLACK))
                    .width(200.px())
            )
        }
        widget.push(flex_row(tr_widget).border(Color::BLACK, 1.25));
    }
    flex_col(widget).border(Color::BLACK, 1.25)
        .into_any_flex()
}

pub fn image(element: ElementRef) -> AnyFlexChild<AppData>{
    todo!()
}

pub fn empty(element:ElementRef) -> AnyFlexChild<AppData> {
    prose(element.inner_html())
        .into_any_flex()
}