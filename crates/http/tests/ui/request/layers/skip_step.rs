use http::__typestate::{RequestLayerPipeline, TraceAdded};

fn skip_step_case(pipeline: RequestLayerPipeline<TraceAdded>) {
    let _pipeline = pipeline.add_user_context();
}

fn main() {}
