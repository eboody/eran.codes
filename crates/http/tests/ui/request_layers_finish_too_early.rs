use http::__typestate::{CoreReady, RequestLayerPipeline};

fn finish_too_early(pipeline: RequestLayerPipeline<CoreReady>) {
    let _router = pipeline.finish();
}

fn main() {}
