#[test]
fn test_all_artifacts() {
    use crate::frame_decoder;
    use std::fs;
    use std::fs::File;

    let mut frame_dec = frame_decoder::FrameDecoder::new();

    for file in fs::read_dir("./fuzz/artifacts/fuzz_target_1").unwrap() {
        let file_name = file.unwrap().path();

        let fnstr = file_name.to_str().unwrap().to_owned();
        if !fnstr.contains("/crash-") {
            continue;
        }

        let mut f = File::open(file_name.clone()).unwrap();
        match frame_dec.reset(&mut f) {
            Ok(_) => {
                let _ = frame_dec.decode_blocks(&mut f, frame_decoder::BlockDecodingStrategy::All);
                /* ignore errors. It just should never panic on invalid input */
            }
            Err(_) => {} /* ignore errors. It just should never panic on invalid input */
        }
    }
}
