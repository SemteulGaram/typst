use crate::library::prelude::*;
use crate::library::text::ParNode;

/// Align a node along the layouting axes.
#[derive(Debug, Hash)]
pub struct AlignNode {
    /// How to align the node horizontally and vertically.
    pub aligns: Spec<Option<Align>>,
    /// The node to be aligned.
    pub child: LayoutNode,
}

#[node]
impl AlignNode {
    fn construct(_: &mut Context, args: &mut Args) -> TypResult<Content> {
        let aligns: Spec<_> = args.find()?.unwrap_or_default();
        let body: LayoutNode = args.expect("body")?;
        Ok(Content::block(body.aligned(aligns)))
    }
}

impl Layout for AlignNode {
    fn layout(
        &self,
        ctx: &mut Context,
        regions: &Regions,
        styles: StyleChain,
    ) -> TypResult<Vec<Arc<Frame>>> {
        // The child only needs to expand along an axis if there's no alignment.
        let mut pod = regions.clone();
        pod.expand &= self.aligns.map_is_none();

        // Align paragraphs inside the child.
        let mut passed = StyleMap::new();
        if let Some(align) = self.aligns.x {
            passed.set(ParNode::ALIGN, align);
        }

        // Layout the child.
        let mut frames = self.child.layout(ctx, &pod, passed.chain(&styles))?;
        for (region, frame) in regions.iter().zip(&mut frames) {
            // Align in the target size. The target size depends on whether we
            // should expand.
            let target = regions.expand.select(region, frame.size);
            let default = Spec::new(Align::Left, Align::Top);
            let aligns = self.aligns.unwrap_or(default);
            Arc::make_mut(frame).resize(target, aligns);
        }

        Ok(frames)
    }
}

dynamic! {
    Align: "alignment",
}

dynamic! {
    Spec<Align>: "2d alignment",
}

castable! {
    Spec<Option<Align>>,
    Expected: "1d or 2d alignment",
    @align: Align => {
        let mut aligns = Spec::default();
        aligns.set(align.axis(), Some(*align));
        aligns
    },
    @aligns: Spec<Align> => aligns.map(Some),
}