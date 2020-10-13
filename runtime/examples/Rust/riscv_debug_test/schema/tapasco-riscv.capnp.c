#include "tapasco-riscv.capnp.h"
/* AUTO GENERATED - DO NOT EDIT */
#ifdef __GNUC__
# define capnp_unused __attribute__((unused))
# define capnp_use(x) (void) x;
#else
# define capnp_unused
# define capnp_use(x)
#endif


Request_ptr new_Request(struct capn_segment *s) {
	Request_ptr p;
	p.p = capn_new_struct(s, 8, 1);
	return p;
}
Request_list new_Request_list(struct capn_segment *s, int len) {
	Request_list p;
	p.p = capn_new_list(s, len, 8, 1);
	return p;
}
void read_Request(struct Request *s capnp_unused, Request_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	s->request_which = (enum Request_request_which)(int) capn_read16(p.p, 0);
	switch (s->request_which) {
	case Request_request_read:
	case Request_request_write:
		s->request.write.p = capn_getp(p.p, 0, 0);
		break;
	default:
		break;
	}
}
void write_Request(const struct Request *s capnp_unused, Request_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	capn_write16(p.p, 0, s->request_which);
	switch (s->request_which) {
	case Request_request_read:
	case Request_request_write:
		capn_setp(p.p, 0, s->request.write.p);
		break;
	default:
		break;
	}
}
void get_Request(struct Request *s, Request_list l, int i) {
	Request_ptr p;
	p.p = capn_getp(l.p, i, 0);
	read_Request(s, p);
}
void set_Request(const struct Request *s, Request_list l, int i) {
	Request_ptr p;
	p.p = capn_getp(l.p, i, 0);
	write_Request(s, p);
}

ReadReq_ptr new_ReadReq(struct capn_segment *s) {
	ReadReq_ptr p;
	p.p = capn_new_struct(s, 8, 0);
	return p;
}
ReadReq_list new_ReadReq_list(struct capn_segment *s, int len) {
	ReadReq_list p;
	p.p = capn_new_list(s, len, 8, 0);
	return p;
}
void read_ReadReq(struct ReadReq *s capnp_unused, ReadReq_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	s->addr = capn_read16(p.p, 0);
}
void write_ReadReq(const struct ReadReq *s capnp_unused, ReadReq_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	capn_write16(p.p, 0, s->addr);
}
void get_ReadReq(struct ReadReq *s, ReadReq_list l, int i) {
	ReadReq_ptr p;
	p.p = capn_getp(l.p, i, 0);
	read_ReadReq(s, p);
}
void set_ReadReq(const struct ReadReq *s, ReadReq_list l, int i) {
	ReadReq_ptr p;
	p.p = capn_getp(l.p, i, 0);
	write_ReadReq(s, p);
}

ReadRsp_ptr new_ReadRsp(struct capn_segment *s) {
	ReadRsp_ptr p;
	p.p = capn_new_struct(s, 8, 0);
	return p;
}
ReadRsp_list new_ReadRsp_list(struct capn_segment *s, int len) {
	ReadRsp_list p;
	p.p = capn_new_list(s, len, 8, 0);
	return p;
}
void read_ReadRsp(struct ReadRsp *s capnp_unused, ReadRsp_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	s->isRead = (capn_read8(p.p, 0) & 1) != 0;
	s->data = capn_read32(p.p, 4);
}
void write_ReadRsp(const struct ReadRsp *s capnp_unused, ReadRsp_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	capn_write1(p.p, 0, s->isRead != 0);
	capn_write32(p.p, 4, s->data);
}
void get_ReadRsp(struct ReadRsp *s, ReadRsp_list l, int i) {
	ReadRsp_ptr p;
	p.p = capn_getp(l.p, i, 0);
	read_ReadRsp(s, p);
}
void set_ReadRsp(const struct ReadRsp *s, ReadRsp_list l, int i) {
	ReadRsp_ptr p;
	p.p = capn_getp(l.p, i, 0);
	write_ReadRsp(s, p);
}

WriteReq_ptr new_WriteReq(struct capn_segment *s) {
	WriteReq_ptr p;
	p.p = capn_new_struct(s, 8, 0);
	return p;
}
WriteReq_list new_WriteReq_list(struct capn_segment *s, int len) {
	WriteReq_list p;
	p.p = capn_new_list(s, len, 8, 0);
	return p;
}
void read_WriteReq(struct WriteReq *s capnp_unused, WriteReq_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	s->addr = capn_read16(p.p, 0);
	s->data = capn_read32(p.p, 4);
}
void write_WriteReq(const struct WriteReq *s capnp_unused, WriteReq_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	capn_write16(p.p, 0, s->addr);
	capn_write32(p.p, 4, s->data);
}
void get_WriteReq(struct WriteReq *s, WriteReq_list l, int i) {
	WriteReq_ptr p;
	p.p = capn_getp(l.p, i, 0);
	read_WriteReq(s, p);
}
void set_WriteReq(const struct WriteReq *s, WriteReq_list l, int i) {
	WriteReq_ptr p;
	p.p = capn_getp(l.p, i, 0);
	write_WriteReq(s, p);
}

WriteRsp_ptr new_WriteRsp(struct capn_segment *s) {
	WriteRsp_ptr p;
	p.p = capn_new_struct(s, 8, 0);
	return p;
}
WriteRsp_list new_WriteRsp_list(struct capn_segment *s, int len) {
	WriteRsp_list p;
	p.p = capn_new_list(s, len, 8, 0);
	return p;
}
void read_WriteRsp(struct WriteRsp *s capnp_unused, WriteRsp_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	s->isRead = (capn_read8(p.p, 0) & 1) != 0;
	s->success = (capn_read8(p.p, 0) & 2) != 0;
}
void write_WriteRsp(const struct WriteRsp *s capnp_unused, WriteRsp_ptr p) {
	capn_resolve(&p.p);
	capnp_use(s);
	capn_write1(p.p, 0, s->isRead != 0);
	capn_write1(p.p, 1, s->success != 0);
}
void get_WriteRsp(struct WriteRsp *s, WriteRsp_list l, int i) {
	WriteRsp_ptr p;
	p.p = capn_getp(l.p, i, 0);
	read_WriteRsp(s, p);
}
void set_WriteRsp(const struct WriteRsp *s, WriteRsp_list l, int i) {
	WriteRsp_ptr p;
	p.p = capn_getp(l.p, i, 0);
	write_WriteRsp(s, p);
}
