//! The pair behind [`space::Overlap`]: iced caps a stacked layer at its base's
//! size and takes no negative margin, so the reference's two overlaps are
//! drawn here instead.

use iced::advanced::widget::{Operation, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget, layout, mouse, overlay, renderer};
use iced::{Element, Event, Point, Rectangle, Size, Vector};

use crate::style::{self, Drawn, Length, space};

/// Which of the covering's own edges is pinned against the covered element.
enum Anchored {
    Head { raised: Drawn, shed: Drawn },
    Foot { lifted: Drawn },
}

/// One element drawn over the foot of another.
pub struct Overlapping<'a, Message, Theme, Renderer> {
    covered: Element<'a, Message, Theme, Renderer>,
    covering: Element<'a, Message, Theme, Renderer>,
    anchored: Anchored,
}

impl<'a, Message, Theme, Renderer> Overlapping<'a, Message, Theme, Renderer> {
    /// The covering's own top over the covered's foot, the pair standing as
    /// tall as the taller of the two once the covering has handed back what it
    /// sheds.
    pub fn new(
        covered: Element<'a, Message, Theme, Renderer>,
        covering: Element<'a, Message, Theme, Renderer>,
        overlap: space::Overlap,
    ) -> Overlapping<'a, Message, Theme, Renderer> {
        Overlapping {
            covered,
            covering,
            anchored: Anchored::Head {
                raised: overlap.raised.drawn(),
                shed: overlap.shed.drawn(),
            },
        }
    }

    /// The covering's own foot over the covered's foot, the pair keeping the
    /// covered's own height however tall the covering is.
    pub fn lifted(
        covered: Element<'a, Message, Theme, Renderer>,
        covering: Element<'a, Message, Theme, Renderer>,
        lift: Length,
    ) -> Overlapping<'a, Message, Theme, Renderer> {
        Overlapping {
            covered,
            covering,
            anchored: Anchored::Foot {
                lifted: lift.drawn(),
            },
        }
    }

    fn pair(&self) -> [&Element<'a, Message, Theme, Renderer>; 2] {
        [&self.covered, &self.covering]
    }

    fn pair_mut(&mut self) -> [&mut Element<'a, Message, Theme, Renderer>; 2] {
        [&mut self.covered, &mut self.covering]
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Overlapping<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.pair().map(Tree::new).into()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.pair());
    }

    fn size(&self) -> Size<iced::Length> {
        Size {
            width: iced::Length::Fill,
            height: iced::Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let covered = self
            .covered
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let width = covered.size().width;
        let foot = covered.size().height;

        let covering = self.covering.as_widget_mut().layout(
            &mut tree.children[1],
            renderer,
            &layout::Limits::new(Size::ZERO, Size::new(width, f32::INFINITY)),
        );

        match self.anchored {
            Anchored::Head { raised, shed } => {
                let covering =
                    covering.move_to(Point::new(0.0, (foot - style::drawn(raised)).max(0.0)));
                let below = covering.bounds().y + covering.size().height - style::drawn(shed);
                layout::Node::with_children(
                    Size::new(width, foot.max(below)),
                    vec![covered, covering],
                )
            }
            Anchored::Foot { lifted } => {
                let top = foot - style::drawn(lifted) - covering.size().height;
                let covering = covering.move_to(Point::new(0.0, top));
                layout::Node::with_children(Size::new(width, foot), vec![covered, covering])
            }
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for ((child, tree), layout) in self
                .pair_mut()
                .into_iter()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                child
                    .as_widget_mut()
                    .operate(tree, layout, renderer, operation);
            }
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        for ((child, tree), layout) in self
            .pair_mut()
            .into_iter()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                tree, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.pair()
            .into_iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, tree), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(tree, layout, cursor, viewport, renderer)
            })
            .find(|interaction| *interaction != mouse::Interaction::None)
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, tree), layout) in self
            .pair()
            .into_iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child
                .as_widget()
                .draw(tree, renderer, theme, defaults, layout, cursor, viewport);
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let raised: Vec<overlay::Element<'b, Message, Theme, Renderer>> = self
            .pair_mut()
            .into_iter()
            .zip(&mut tree.children)
            .zip(layout.children())
            .filter_map(|((child, tree), layout)| {
                child
                    .as_widget_mut()
                    .overlay(tree, layout, renderer, viewport, translation)
            })
            .collect();
        (!raised.is_empty()).then(|| overlay::Group::with_children(raised).overlay())
    }
}

impl<'a, Message, Theme, Renderer> From<Overlapping<'a, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(overlapping: Overlapping<'a, Message, Theme, Renderer>) -> Self {
        Self::new(overlapping)
    }
}
