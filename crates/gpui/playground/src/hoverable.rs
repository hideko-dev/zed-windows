use crate::{
    element::{Element, Layout},
    layout_context::LayoutContext,
    paint_context::PaintContext,
    style::{Style, StyleHelpers, Styleable},
};
use anyhow::Result;
use gpui::platform::MouseMovedEvent;
use refineable::{CascadeSlot, Refineable, RefinementCascade};
use std::{cell::Cell, rc::Rc};

pub struct Hoverable<E: Styleable> {
    hovered: Rc<Cell<bool>>,
    cascade_slot: CascadeSlot,
    hovered_style: <E::Style as Refineable>::Refinement,
    child: E,
}

pub fn hoverable<E: Styleable>(mut child: E) -> Hoverable<E> {
    Hoverable {
        hovered: Rc::new(Cell::new(false)),
        cascade_slot: child.style_cascade().reserve(),
        hovered_style: Default::default(),
        child,
    }
}

impl<E: Styleable> Styleable for Hoverable<E> {
    type Style = E::Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<Self::Style> {
        self.child.style_cascade()
    }

    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement {
        &mut self.hovered_style
    }
}

impl<V: 'static, E: Element<V> + Styleable> Element<V> for Hoverable<E> {
    type Layout = E::Layout;

    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<Layout<V, Self::Layout>>
    where
        Self: Sized,
    {
        self.child.layout(view, cx)
    }

    fn paint(
        &mut self,
        view: &mut V,
        layout: &mut Layout<V, Self::Layout>,
        cx: &mut PaintContext<V>,
    ) where
        Self: Sized,
    {
        let bounds = layout.bounds(cx);
        let order = layout.order(cx);

        self.hovered.set(bounds.contains_point(cx.mouse_position()));

        let slot = self.cascade_slot;
        let style = self.hovered.get().then_some(self.hovered_style.clone());
        self.style_cascade().set(slot, style);

        let hovered = self.hovered.clone();
        cx.on_event(order, move |view, event: &MouseMovedEvent, cx| {
            if bounds.contains_point(event.position) != hovered.get() {
                cx.repaint();
            }
        });

        self.child.paint(view, layout, cx);
    }
}

impl<E: Styleable<Style = Style>> StyleHelpers for Hoverable<E> {}
