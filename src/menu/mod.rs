#![no_std]

use crate::menu::items::MenuItem;
pub mod items;

use crate::menu::items::checkbox::CheckboxItem;
use crate::menu::items::multi_option::MultiOptionItem;
use crate::menu::items::section::SectionItem;
use crate::menu::items::submenu::SubmenuItem;
use crate::menu::items::MenuItems;
use embedded_graphics::geometry::AnchorY;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Baseline, Text};
use embedded_layout::View;
use trees::Tree;

pub struct Menu<'a, C>
where
    C: PixelColor,
{
    menu_tree_root: Tree<MenuItems<'a, C>>,
    menu_style: MenuStyle<'a, C>,
    menu_state: MenuState,
}

impl<'a, C> Menu<'a, C>
where
    C: PixelColor,
{
    pub fn new(label: &'static str, menu_style: MenuStyle<'a, C>) -> Self {
        Self {
            menu_tree_root: Tree::new(MenuItems::Submenu(SubmenuItem::new(label, menu_style))),
            menu_style,
            menu_state: MenuState::new(),
        }
    }

    /// Add menu item to the menu structure that will be drawn
    pub fn add_item(&mut self, item: MenuItems<'a, C>) {
        self.menu_tree_root.push_back(Tree::new(item));
        self.menu_state
            .update_item_count(self.menu_tree_root.iter().count());
    }

    /// Add checkbox as next item in the menu
    pub fn add_checkbox(&mut self, label: &'static str) {
        self.add_item(MenuItems::Checkbox(CheckboxItem::new(
            label,
            self.menu_style,
        )));
    }

    /// Add selector as next item in the menu
    pub fn add_selector(&mut self, label: &'static str, options: &'a [&'static str]) {
        self.add_item(MenuItems::Selector(MultiOptionItem::new(
            label,
            self.menu_style,
            options,
        )));
    }

    /// Add section (non-selectable item) as next item in the menu
    pub fn add_section(&mut self, label: &'static str) {
        self.add_item(MenuItems::Section(SectionItem::new(label, self.menu_style)));
    }

    /// Add a sub-menu to the menu structure that will be drawn
    pub fn add_submenu(&mut self, submenu: Menu<'a, C>) {
        self.menu_tree_root.push_back(submenu.into());
        self.menu_state
            .update_item_count(self.menu_tree_root.iter().count());
    }

    pub fn navigate_down(&mut self) {
        self.menu_state.move_down();
    }

    pub fn navigate_up(&mut self) {
        self.menu_state.move_up();
    }

    pub fn select_item(&mut self) {}
}

impl<C> Drawable for Menu<'_, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let display_area = display.bounding_box();
        display.clear(self.menu_style.menu_background_color)?;
        let header = self.menu_tree_root.data();
        let header_height = self.menu_style.heading_character_style.line_height();
        Text::with_baseline(
            header.label(),
            Point::zero(),
            self.menu_style.heading_character_style,
            Baseline::Top,
        )
        .draw(display)?;

        let mut remaining_item_area = display_area
            .resized_height(display_area.size().height - header_height, AnchorY::Bottom);

        let menu_iter = self
            .menu_tree_root
            .iter()
            .skip(self.menu_state.highlighted_item());

        for menu_item in menu_iter {
            let item_height = menu_item.data().size().height;
            if item_height > remaining_item_area.size().height {
                break;
            }

            let mut item_display = display.cropped(&remaining_item_area);
            menu_item.data().draw(&mut item_display)?;

            remaining_item_area = remaining_item_area.resized_height(
                remaining_item_area.size().height - item_height,
                AnchorY::Bottom,
            );
        }

        Ok(())
    }
}

impl<'a, C> From<Menu<'a, C>> for Tree<MenuItems<'a, C>>
where
    C: PixelColor,
{
    fn from(menu: Menu<'a, C>) -> Tree<MenuItems<'a, C>> {
        menu.menu_tree_root
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MenuStyle<'a, C> {
    pub(crate) menu_background_color: C,
    pub(crate) heading_character_style: MonoTextStyle<'a, C>,
    pub(crate) item_character_style: MonoTextStyle<'a, C>,
    pub(crate) indicator_fill_color: C,
    pub(crate) highlight_item_color: C,
    pub(crate) highlight_text_style: MonoTextStyle<'a, C>,
}

impl<'a, C> MenuStyle<'a, C>
where
    C: PixelColor,
{
    pub fn new(
        menu_background_color: C,
        heading_character_style: MonoTextStyle<'a, C>,
        item_character_style: MonoTextStyle<'a, C>,
        indicator_fill_color: C,
        highlight_item_color: C,
        highlight_text_style: MonoTextStyle<'a, C>,
    ) -> Self {
        Self {
            menu_background_color,
            heading_character_style,
            item_character_style,
            indicator_fill_color,
            highlight_item_color,
            highlight_text_style,
        }
    }
}

struct MenuState {
    highlighted_item: usize,
    item_count: usize,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            highlighted_item: 0,
            item_count: 0,
        }
    }
    pub fn update_item_count(&mut self, item_count: usize) {
        self.item_count = item_count;
    }
    pub fn move_down(&mut self) {
        self.highlighted_item += 1;
        if self.highlighted_item > self.item_count {
            self.highlighted_item = 0;
        }
    }

    pub fn move_up(&mut self) {
        if self.highlighted_item == 0 {
            self.highlighted_item = self.item_count - 1;
        } else {
            self.highlighted_item -= 1;
        }
    }

    pub fn highlighted_item(&self) -> usize {
        self.highlighted_item
    }
}
