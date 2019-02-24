use blaze_rs::dynamic::*;

#[cfg(test)]
pub fn test_init_shutdown() {
    {
        let options = SpriteBatchOpts {
            max_buckets: 5,
            max_sprites_per_bucket: 100,
            flags: InitFlags::Default,
        };
        let batch = SpriteBatch::new(options.clone());
        assert!(batch.is_ok());
        assert!(batch.unwrap().get_options() == &options);
    }
    {
        let fail = SpriteBatch::new(SpriteBatchOpts {
            max_buckets: 0,
            max_sprites_per_bucket: 0,
            flags: InitFlags::Default,
        });
        assert!(fail.is_err());
    }
}
