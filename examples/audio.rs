use stream_download::StreamDownload;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let reader = StreamDownload::new_http(
        "https://www.learningcontainer.com/wp-content/uploads/2020/02/Sample-FLAC-File.flac?_=3"
            .parse()
            .unwrap(),
    );

    sink.append(rodio::Decoder::new(reader).unwrap());

    sink.sleep_until_end();
}
