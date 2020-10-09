@0x997b72335b9c6aaf;

struct Request {
    request :union {
        read @0 :ReadReq;
        write @1 :WriteReq;
    }
}

struct ReadReq {
    addr @0 :UInt16;
}

struct ReadRsp {
    data @0 :UInt32;
}

struct WriteReq {
    addr @0 :UInt16;
    data @1 :UInt32;
}

