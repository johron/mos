use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Panel {
    pub id: String,
    pub child: PanelChild,
    pub position: (u16, u16),
    pub size: (u16, u16),
}

impl Panel {
    pub fn new(id: String, child: PanelChild) -> Self {
        Self {
            id,
            child,
            position: (0, 0),
            size: (0, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PanelChild {
    Editor(/*EditorPanel*/),
    Explorer(/*ExplorerPanel*/),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SplitOrientation {
    Horizontal, // stack top/bottom
    Vertical,   // stack left/right
}

#[derive(Debug, Clone)]
pub enum Placement {
    Top,
    Bottom,
    Left,
    Right,
    Replace,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub enum LayoutNode {
    Leaf { panel_id: String },
    Split {
        orientation: SplitOrientation,
        children: Vec<LayoutNode>,
        ratios: Vec<f32>, // same length as children
    },
}

impl LayoutNode {
    // Try to insert new panel relative to target_id. Returns true if inserted.
    pub fn add_relative(&mut self, target_id: &str, new_id: String, placement: Placement) -> bool {
        match self {
            LayoutNode::Leaf { panel_id } if panel_id == target_id => {
                if placement == Placement::Replace {
                    *panel_id = new_id;
                    return true;
                }
                let old = LayoutNode::Leaf {
                    panel_id: panel_id.clone(),
                };
                let new_leaf = LayoutNode::Leaf { panel_id: new_id };
                let (orientation, children) = match placement {
                    Placement::Top => (
                        SplitOrientation::Horizontal,
                        vec![new_leaf, old], // top then old
                    ),
                    Placement::Bottom => (
                        SplitOrientation::Horizontal,
                        vec![old, new_leaf], // old then bottom
                    ),
                    Placement::Left => (
                        SplitOrientation::Vertical,
                        vec![new_leaf, old], // left then old
                    ),
                    Placement::Right => (
                        SplitOrientation::Vertical,
                        vec![old, new_leaf], // old then right
                    ),
                    _ => unreachable!(),
                };
                *self = LayoutNode::Split {
                    orientation,
                    children,
                    ratios: vec![0.5, 0.5],
                };
                return true;
            }
            LayoutNode::Leaf { .. } => false,
            LayoutNode::Split {
                orientation: _,
                children,
                ratios: _,
            } => {
                // try to insert into children (depth-first)
                for child in children.iter_mut() {
                    if child.add_relative(target_id, new_id.clone(), placement.clone()) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

pub struct PanelHandler {
    panels: Vec<Panel>,
    pub root: Option<LayoutNode>,
}

impl PanelHandler {
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
            root: None,
        }
    }

    pub fn find_panel_index(&self, id: &str) -> Option<usize> {
        self.panels.iter().position(|p| p.id == id)
    }

    pub fn add_panel(&mut self, new_panel: Panel, target_id: Option<&str>, placement: Placement) {
        let new_id = new_panel.id.clone();
        self.panels.push(new_panel);

        match (&mut self.root, target_id) {
            (None, _) => {
                // first panel becomes root
                self.root = Some(LayoutNode::Leaf { panel_id: new_id });
            }
            (Some(root), Some(tid)) => {
                // insert relative to target leaf
                if !root.add_relative(tid, new_id, placement) {
                    // if target not found, fallback: append to root as Right
                    let current_root = std::mem::replace(root, LayoutNode::Leaf { panel_id: String::new() });
                    *root = LayoutNode::Split {
                        orientation: SplitOrientation::Vertical,
                        children: vec![current_root, LayoutNode::Leaf { panel_id: new_id }],
                        ratios: vec![0.8, 0.2],
                    };
                }
            }
            (Some(root), None) => {
                // no target: split root by placement
                let current_root = std::mem::replace(root, LayoutNode::Leaf { panel_id: String::new() });
                let (orientation, children) = match placement {
                    Placement::Top => (
                        SplitOrientation::Horizontal,
                        vec![LayoutNode::Leaf { panel_id: new_id }, current_root],
                    ),
                    Placement::Bottom => (
                        SplitOrientation::Horizontal,
                        vec![current_root, LayoutNode::Leaf { panel_id: new_id }],
                    ),
                    Placement::Left => (
                        SplitOrientation::Vertical,
                        vec![LayoutNode::Leaf { panel_id: new_id }, current_root],
                    ),
                    Placement::Right => (
                        SplitOrientation::Vertical,
                        vec![current_root, LayoutNode::Leaf { panel_id: new_id }],
                    ),
                    Placement::Replace => {
                        *root = LayoutNode::Leaf { panel_id: new_id };
                        return;
                    }
                };
                *root = LayoutNode::Split {
                    orientation,
                    children,
                    ratios: vec![0.5, 0.5],
                };
            }
        }
    }

    // Compute rectangle map for each panel id and update panel position/size.
    pub fn compute_layout(&mut self, full: Rect) -> HashMap<String, Rect> {
        let mut map = HashMap::new();
        if let Some(root) = &self.root {
            Self::layout_recursive(root, full, &mut map);
        }
        // update stored panels
        for panel in self.panels.iter_mut() {
            if let Some(r) = map.get(&panel.id) {
                panel.position = (r.x, r.y);
                panel.size = (r.width, r.height);
            }
        }
        map
    }

    fn layout_recursive(node: &LayoutNode, rect: Rect, out: &mut HashMap<String, Rect>) {
        match node {
            LayoutNode::Leaf { panel_id } => {
                out.insert(panel_id.clone(), rect);
            }
            LayoutNode::Split {
                orientation,
                children,
                ratios,
            } => {
                let total: f32 = ratios.iter().sum::<f32>().max(1.0);
                if children.is_empty() {
                    return;
                }
                if *orientation == SplitOrientation::Horizontal {
                    // split vertically (top/bottom)
                    let mut y = rect.y as u32;
                    let width = rect.width;
                    for (i, child) in children.iter().enumerate() {
                        let frac = ratios.get(i).copied().unwrap_or(1.0) / total;
                        let mut h = ((rect.height as f32) * frac).round() as u32;
                        // ensure last child consumes remaining pixels
                        if i == children.len() - 1 {
                            let used: u32 = out
                                .values()
                                .filter(|r| r.x == rect.x && r.width == width)
                                .map(|r| r.height as u32)
                                .sum();
                            // but simpler: compute remaining by rect.height - sum(prev), however out doesn't hold these interim values.
                            // We'll leave rounding errors to last child:
                            h = (rect.y as u32 + rect.height as u32).saturating_sub(y);
                        }
                        let child_rect = Rect {
                            x: rect.x,
                            y: y as u16,
                            width,
                            height: h as u16,
                        };
                        Self::layout_recursive(child, child_rect, out);
                        y = y.saturating_add(h);
                    }
                } else {
                    // Vertical split (left/right)
                    let mut x = rect.x as u32;
                    let height = rect.height;
                    for (i, child) in children.iter().enumerate() {
                        let frac = ratios.get(i).copied().unwrap_or(1.0) / total;
                        let mut w = ((rect.width as f32) * frac).round() as u32;
                        if i == children.len() - 1 {
                            w = (rect.x as u32 + rect.width as u32).saturating_sub(x);
                        }
                        let child_rect = Rect {
                            x: x as u16,
                            y: rect.y,
                            width: w as u16,
                            height,
                        };
                        Self::layout_recursive(child, child_rect, out);
                        x = x.saturating_add(w);
                    }
                }
            }
        }
    }
}