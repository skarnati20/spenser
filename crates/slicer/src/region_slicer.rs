use core::slicer::Slicer;
use types::{CardContent, RegionRef, SchemaDiff};

pub struct RegionSlicer;

impl Slicer for RegionSlicer {
    fn name(&self) -> &str {
        "region"
    }

    fn slice(&self, diff: &SchemaDiff) -> Vec<CardContent> {
        let mut cards = Vec::new();

        for file in &diff.files {
            let Some(path) = file.new_path.as_ref().or(file.old_path.as_ref()) else {
                continue;
            };

            for region in &file.regions {
                let body = region
                    .lines
                    .iter()
                    .map(|line| line.content.as_str())
                    .collect::<String>();

                cards.push(CardContent {
                    body,
                    region_ref: RegionRef {
                        path: path.clone(),
                        range: region.range.clone(),
                    },
                });
            }
        }

        cards
    }
}
