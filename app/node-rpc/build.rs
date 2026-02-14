fn main() {
    tonic_prost_build::compile_protos("protobufs/greeter.proto").unwrap();
    tonic_prost_build::compile_protos("protobufs/node.proto").unwrap();
}
