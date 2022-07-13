use indicatif::{ProgressBar, ProgressStyle};

pub trait BarPreCreate {
    fn create(val: u64) -> ProgressBar {
        let pb = ProgressBar::new(val);

        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {percent}% [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        pb
    }
}

impl BarPreCreate for ProgressBar {}
