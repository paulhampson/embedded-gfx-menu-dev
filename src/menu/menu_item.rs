use crate::menu::menu::MenuStyle;
use core::fmt;
use core::fmt::Formatter;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{AnchorX, Point};
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::{DrawTargetExt, Primitive, Size};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle, Triangle};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_graphics::Drawable;
use embedded_layout::View;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuItemType {
    Section,
    Checkbox,
    Selector,
    Submenu,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MenuItem<'a, C>
where
    C: PixelColor,
{
    label: &'static str,
    item_type: MenuItemType,
    highlighted: bool,
    position: Point,
    menu_style: MenuStyle<'a, C>,
}

impl<C> MenuItem<'_, C>
where
    C: PixelColor,
{
    pub const fn new<'a>(
        label: &'static str,
        item_type: MenuItemType,
        menu_style: MenuStyle<'a, C>,
    ) -> MenuItem<'a, C> {
        MenuItem::<'a, C> {
            label,
            item_type,
            highlighted: false,
            position: Point::zero(),
            menu_style,
        }
    }

    pub fn label(&self) -> &str {
        self.label
    }
}

impl<C> fmt::Display for MenuItem<'_, C>
where
    C: PixelColor,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[\"{}\":{:?}]", self.label, self.item_type)
    }
}

impl<C> View for MenuItem<'_, C>
where
    C: PixelColor,
{
    fn translate_impl(&mut self, by: Point) {
        self.position += by;
    }

    fn bounds(&self) -> Rectangle {
        self.menu_style
            .item_character_style
            .measure_string(self.label, Point::zero(), Baseline::Bottom)
            .bounding_box
    }
}

impl<C> Drawable for MenuItem<'_, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        match self.item_type {
            MenuItemType::Submenu => {
                let indicator_vertical_pad = 2u32;
                let indicator_right_pad = 2u32;
                let submenu_indicator_size = Size::new(self.size().height / 2, self.size().height);

                let display_size = display.bounding_box();
                let submenu_indicator_draw_area =
                    display_size.resized_width(submenu_indicator_size.width, AnchorX::Right);
                let mut indicator_display = display.cropped(&submenu_indicator_draw_area);
                let filled_style = PrimitiveStyle::with_fill(self.menu_style.indicator_fill_color);

                Triangle::new(
                    Point::new(0, indicator_vertical_pad as i32),
                    Point::new(
                        0,
                        (submenu_indicator_size.height - indicator_vertical_pad) as i32,
                    ),
                    Point::new(
                        (submenu_indicator_size.width - indicator_right_pad) as i32,
                        (((submenu_indicator_size.height - indicator_vertical_pad * 2) / 2)
                            + indicator_vertical_pad) as i32,
                    ),
                )
                .into_styled(filled_style)
                .draw(&mut indicator_display)?;

                let submenu_label_draw_area = display_size.resized_width(
                    display_size.size().width - submenu_indicator_size.width,
                    AnchorX::Left,
                );
                let mut label_display = display.cropped(&submenu_label_draw_area);

                Text::with_baseline(
                    self.label,
                    self.position,
                    self.menu_style.item_character_style,
                    Baseline::Top,
                )
                .draw(&mut label_display)?;
            }
            MenuItemType::Checkbox => {
                Text::with_baseline(
                    self.label,
                    self.position,
                    self.menu_style.item_character_style,
                    Baseline::Top,
                )
                .draw(display)?;

                Text::with_text_style(
                    "[ ]",
                    Point::new(display.bounding_box().size().width as i32, 0),
                    self.menu_style.item_character_style,
                    TextStyleBuilder::new()
                        .alignment(Alignment::Right)
                        .baseline(Baseline::Top)
                        .build(),
                )
                .draw(display)?;
            }
            _ => {
                Text::with_baseline(
                    self.label,
                    self.position,
                    self.menu_style.item_character_style,
                    Baseline::Top,
                )
                .draw(display)?;
            }
        }

        Ok(())
    }
}
