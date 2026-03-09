use http::__typestate::{RequestLayerPipeline, TraceAdded};

fn skip_step(pipeline: RequestLayerPipeline<TraceAdded>) {
    let _pipeline = pipeline.add_user_context();
}

fn main() {}
