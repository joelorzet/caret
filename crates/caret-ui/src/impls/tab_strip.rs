#[allow(unused_imports)]
use crate::prelude::*;

impl TabStrip {
    /// As wide as a tab is ever allowed to get, however long its title.
    const MAX: f32 = 220.0;
    const MIN: f32 = 52.0;
    const GAP: f32 = 2.0;
    /// Breathing room inside a tab, before its text.
    pub const PAD: f32 = 10.0;
    /// How far the drawn pill sits inside the tab's rectangle. The layout
    /// has to know, or it hands the title room the pill does not have.
    pub const PILL_INSET: f32 = 2.0;
    pub const RADIUS: f32 = 7.0;
    const NEW_TAB: f32 = 30.0;
    const CLOSE: f32 = 16.0;
    /// Characters of title that must survive beside a close button; below
    /// this the button is dropped rather than left covering the name.
    const CLOSE_NEEDS: u32 = 4;

    /// Whether the tabs are numbered.
    ///
    /// One tab has nothing to be picked out from, so the number in front of
    /// it is a label for a choice that does not exist. They come back as
    /// soon as there is a second tab to switch to.
    pub fn numbered(&self) -> bool {
        self.tabs.len() > 1
    }

    pub fn px(&self, logical: f32) -> u32 {
        (logical * self.scale).round().max(0.0) as u32
    }

    pub fn layout(&self, width: u32) -> StripLayout {
        let gap = self.px(Self::GAP);
        let new_tab_w = self.px(Self::NEW_TAB);
        let close_w = self.px(Self::CLOSE);
        let available = width
            .saturating_sub(self.inset_left)
            .saturating_sub(new_tab_w + gap);

        let widths = self.widths(available, gap, close_w);
        // The room a tab needs before it can also carry a close button:
        // what it is padded by, the button itself, and the title. A title
        // shorter than the limit only has to keep what it has, or a tab
        // called `~` would never get a button it has ample room for.
        let fixed = (self.px(Self::PAD) + self.px(Self::PILL_INSET)) * 2 + close_w;

        let mut tabs = Vec::with_capacity(widths.len());
        let mut closes = Vec::new();
        let mut x = self.inset_left as i32;
        for (index, width) in widths.into_iter().enumerate() {
            let rect = PxRect::new(x, 0, width, self.height);
            let kept = self.tabs.get(index).map_or(0, |tab| {
                tab.columns(index, self.numbered()).min(Self::CLOSE_NEEDS)
            });
            if width >= fixed + kept * self.cell_width {
                let pad = self.px(Self::PAD) / 2 + self.px(Self::PILL_INSET);
                let at = rect.right() - (close_w + pad) as i32;
                closes.push(PxRect::new(at, 0, close_w, self.height).centred(close_w, close_w));
            }
            tabs.push(rect);
            x = rect.right() + gap as i32;
        }

        // The button follows the tabs rather than pinning to the right edge,
        // which is what every other tab bar does and keeps the pointer close.
        let new_tab = PxRect::new(
            x.min(width.saturating_sub(new_tab_w) as i32),
            0,
            new_tab_w,
            self.height,
        );

        StripLayout {
            tabs,
            closes,
            new_tab,
        }
    }

    /// Tabs are as wide as what they hold, up to a limit; they only give that
    /// up, uniformly, once they no longer fit.
    fn widths(&self, available: u32, gap: u32, close_w: u32) -> Vec<u32> {
        let count = self.tabs.len() as u32;
        if count == 0 {
            return Vec::new();
        }

        let fixed = (self.px(Self::PAD) + self.px(Self::PILL_INSET)) * 2 + close_w;
        let natural: Vec<u32> = self
            .tabs
            .iter()
            .enumerate()
            .map(|(index, tab)| {
                (tab.columns(index, self.numbered()) * self.cell_width + fixed)
                    .clamp(self.px(Self::MIN), self.px(Self::MAX))
            })
            .collect();

        let gaps = gap * count.saturating_sub(1);
        if natural.iter().sum::<u32>() + gaps <= available {
            return natural;
        }

        let share = available.saturating_sub(gaps) / count;
        let uniform = share.max(self.px(Self::MIN));
        natural.iter().map(|w| uniform.min(*w)).collect()
    }

    pub fn hit(&self, layout: &StripLayout, x: i32, y: i32) -> StripHit {
        // Close buttons sit inside their tab, so they are tested first.
        for (index, rect) in layout.closes.iter().enumerate() {
            if rect.contains(x, y) {
                return StripHit::Close(index);
            }
        }
        if layout.new_tab.contains(x, y) {
            return StripHit::NewTab;
        }
        for (index, rect) in layout.tabs.iter().enumerate() {
            if rect.contains(x, y) {
                return StripHit::Tab(index);
            }
        }
        StripHit::Background
    }
}
