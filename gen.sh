#!/bin/bash

AELF_SPKG="https://github.com/streamingfast/firehose-aelf/releases/download/v0.1.1/aelf-v0.1.1.spkg"
substreams protogen "$AELF_SPKG" --exclude-paths="sf/substreams/rpc,sf/substreams/v1,google/" --output-path="./core/src/pb"