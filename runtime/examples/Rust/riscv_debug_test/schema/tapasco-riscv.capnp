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
    isRead @0 :Bool;
    data @1 :UInt32;
}

struct WriteReq {
    addr @0 :UInt16;
    data @1 :UInt32;
}

struct WriteRsp {
    isRead @0 :Bool;
    success @1 :Bool;
}
