use crate::menu::MenuStyle;
use core::fmt;
use core::fmt::{Debug, Display, Formatter};
use embedded_graphics::draw_target::{DrawTarget, DrawTargetExt};
use embedded_graphics::geometry::{AnchorX, Point, Size};
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle, Triangle};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use embedded_layout::View;

#[derive(PartialEq, Clone, Copy)]
pub struct SubmenuItem<'a, C>
where
    C: PixelColor,
{
    label: &'static str,
    highlighted: bool,
    position: Point,
    menu_style: MenuStyle<'a, C>,
}

impl<C> SubmenuItem<'_, C>
where
    C: PixelColor,
{
    pub const fn new<'a>(label: &'static str, menu_style: MenuStyle<'a, C>) -> SubmenuItem<'a, C> {
        SubmenuItem {
            label,
            highlighted: false,
            position: Point::zero(),
            menu_style,
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }
}

impl<C> Debug for SubmenuItem<'_, C>
where
    C: PixelColor,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[\"{}\":Checkbox]", self.label)
    }
}

impl<C> Display for SubmenuItem<'_, C>
where
    C: PixelColor,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl<C> View for SubmenuItem<'_, C>
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

impl<C> Drawable for SubmenuItem<'_, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
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

        Ok(())
    }
}