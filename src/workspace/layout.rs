use crate::app::MosId;
use crate::panel::panel::Panel;
use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::Frame;

pub enum Axis {
    Horizontal,
    Vertical,
}

pub enum Layout {
    Split {
        axis: Axis,
        //ratio: f32,
        children: Vec<Layout>,
    },
    Tabs {
        tabs: Vec<Box<dyn Panel>>,
        active: Option<MosId>,
    },
}

impl Layout {
    pub fn get_active_panel(&self) -> Option<&dyn Panel> {
        match self {
            Layout::Split { children, .. } => {
                for child in children {
                    if let Some(panel) = child.get_active_panel() {
                        return Some(panel);
                    }
                }
                None
            }
            Layout::Tabs { tabs, active } => {
                if let Some(active_id) = active.as_ref() {
                    if let Some(panel) = tabs.iter().find(|p| p.id() == *active_id) {
                        return Some(panel.as_ref());
                    }
                }
                tabs.first().map(|b| b.as_ref())
            }
        }
    }

    pub fn get_active_panel_mut(&mut self) -> Option<&mut (dyn Panel + 'static)> {
        match self {
            Layout::Split { children, .. } => {
                for child in children {
                    if let Some(panel) = child.get_active_panel_mut() {
                        return Some(panel);
                    }
                }
                None
            }

            Layout::Tabs { tabs, active } => {
                // Try active tab first
                if let Some(active_id) = active.as_ref() {
                    if let Some(index) = tabs.iter().position(|p| p.id() == *active_id) {
                        return Some(tabs[index].as_mut());
                    }
                }

                // Fallback to first tab
                tabs.first_mut().map(|b| b.as_mut())
            }
        }
    }



    pub fn render(&self, frame: &mut Frame, area: Rect) {
        match self {
            Layout::Split { axis, children } => {
                let constraints = vec![Constraint::Percentage(100 / children.len() as u16); children.len()];
                let chunks = match axis {
                    Axis::Horizontal => ratatui::layout::Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(constraints)
                        .split(area),
                    Axis::Vertical => ratatui::layout::Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(constraints)
                        .split(area),
                };
                for (child, chunk) in children.iter().zip(chunks.iter()) {
                    child.render(frame, *chunk);
                }
            }
            Layout::Tabs { tabs, active } => {
                //println!("Rendering Tabs layout with {} tabs, active tab id: {:?}", tabs.len(), active);
                if let Some(active_id) = active.as_ref() {
                    if let Some(active_panel) = tabs.iter().find(|panel| panel.id() == *active_id) {
                        active_panel.render(frame, area);
                        return;
                    }
                }

                if let Some(first) = tabs.first() {
                    first.render(frame, area);
                } else {
                    // No tabs to render, maybe render a placeholder or do nothing
                    println!("No tabs to render in Tabs layout");
                }
            }
        }
    }
}

pub enum Anchor {
    Top(Offset),
    Bottom(Offset),
    Left(Offset),
    Right(Offset),
    TopLeft(Offset),
    TopRight(Offset),
    BottomLeft(Offset),
    BottomRight(Offset),
}

pub enum Offset {
    Absolute(i32, i32, i32, i32), // left, top, right, bottom
    Relative(f32, f32, f32, f32), // left, top, right, bottom as percentage of parent size
}

pub struct FloatingPanel {
    pub panel: Box<dyn Panel>,
    pub anchor: Anchor,
}