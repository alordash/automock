use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn normalize_super_paths_in_signature(mut signature: Signature) -> Signature {
    SuperPathsNormalizer.visit_signature_mut(&mut signature);
    return signature;
}

struct SuperPathsNormalizer;

impl VisitMut for SuperPathsNormalizer {
    fn visit_path_mut(&mut self, i: &mut Path) {
        if let Some(super_segment_index) = i.segments.iter().position(|x| x.ident == "super") {
            i.segments
                .insert(super_segment_index, i.segments[super_segment_index].clone());
        }

        visit_mut::visit_path_mut(self, i);
    }
}

pub(crate) fn normalize_super_paths_in_block(mut block: Block) -> Block {
    SuperUsagesNormalizer.visit_block_mut(&mut block);
    return block;
}

struct SuperUsagesNormalizer;

impl VisitMut for SuperUsagesNormalizer {
    fn visit_use_path_mut(&mut self, i: &mut UsePath) {
        if i.ident == "super" {
            let decoy_tree = Box::new(UseTree::Glob(UseGlob {
                star_token: Default::default(),
            }));
            let mut base_tree = core::mem::replace(&mut i.tree, decoy_tree);
            visit_mut::visit_use_tree_mut(self, &mut base_tree);
            *i.tree = UseTree::Path(UsePath {
                ident: i.ident.clone(),
                colon2_token: i.colon2_token,
                tree: base_tree,
            });
            return;
        }

        visit_mut::visit_use_path_mut(self, i);
    }
}
