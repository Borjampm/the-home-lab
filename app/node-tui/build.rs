fn main() {
    tonic_prost_build::compile_protos("../node-rpc/protobufs/node.proto").unwrap();
    tonic_prost_build::compile_protos("../node-rpc/protobufs/greeter.proto").unwrap();
}
