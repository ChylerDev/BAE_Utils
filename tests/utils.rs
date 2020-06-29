#[cfg(test)]
mod tests {
    use bae_utils::*;
    use bae_types::*;

    use std::fs::File;

    const SAMPLE_RATE: usize = 48_000;
    const INV_SAMPLE_RATE: AccurateMath = 1.0 / SAMPLE_RATE as AccurateMath;

    #[test]
    fn test_write_wav() -> Result<(), ()> {
        let mut t = SampleTrack::new();

        for i in 0..16 {
            t.push(Sample((2.0 * std::f32::consts::PI * 440.0 * i as f32 * INV_SAMPLE_RATE as f32).sin()));
        }

        std::fs::create_dir_all(".junk/utils").unwrap();
        WaveWriteOptions::new()
            .bps(24)?
            .r(Math(SAMPLE_RATE as AccurateMath))
            .clip(true)
            .write(
                vec![t],
                &mut File::create(".junk/utils/wavwrite.wav").unwrap(),
            )
            .unwrap();

        Ok(())
    }
}
