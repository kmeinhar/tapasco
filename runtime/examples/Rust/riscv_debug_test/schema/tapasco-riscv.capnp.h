#ifndef CAPN_997B72335B9C6AAF
#define CAPN_997B72335B9C6AAF
/* AUTO GENERATED - DO NOT EDIT */
#include <capnp_c.h>

#if CAPN_VERSION != 1
#error "version mismatch between capnp_c.h and generated code"
#endif

#ifndef capnp_nowarn
# ifdef __GNUC__
#  define capnp_nowarn __extension__
# else
#  define capnp_nowarn
# endif
#endif


#ifdef __cplusplus
extern "C" {
#endif

struct Request;
struct ReadReq;
struct ReadRsp;
struct WriteReq;
struct WriteRsp;

typedef struct {capn_ptr p;} Request_ptr;
typedef struct {capn_ptr p;} ReadReq_ptr;
typedef struct {capn_ptr p;} ReadRsp_ptr;
typedef struct {capn_ptr p;} WriteReq_ptr;
typedef struct {capn_ptr p;} WriteRsp_ptr;

typedef struct {capn_ptr p;} Request_list;
typedef struct {capn_ptr p;} ReadReq_list;
typedef struct {capn_ptr p;} ReadRsp_list;
typedef struct {capn_ptr p;} WriteReq_list;
typedef struct {capn_ptr p;} WriteRsp_list;
enum Request_request_which {
	Request_request_read = 0,
	Request_request_write = 1
};

struct Request {
	enum Request_request_which request_which;
	capnp_nowarn union {
		ReadReq_ptr read;
		WriteReq_ptr write;
	} request;
};

static const size_t Request_word_count = 1;

static const size_t Request_pointer_count = 1;

static const size_t Request_struct_bytes_count = 16;

struct ReadReq {
	uint16_t addr;
};

static const size_t ReadReq_word_count = 1;

static const size_t ReadReq_pointer_count = 0;

static const size_t ReadReq_struct_bytes_count = 8;

struct ReadRsp {
	unsigned isRead : 1;
	uint32_t data;
};

static const size_t ReadRsp_word_count = 1;

static const size_t ReadRsp_pointer_count = 0;

static const size_t ReadRsp_struct_bytes_count = 8;

struct WriteReq {
	uint16_t addr;
	uint32_t data;
};

static const size_t WriteReq_word_count = 1;

static const size_t WriteReq_pointer_count = 0;

static const size_t WriteReq_struct_bytes_count = 8;

struct WriteRsp {
	unsigned isRead : 1;
	unsigned success : 1;
};

static const size_t WriteRsp_word_count = 1;

static const size_t WriteRsp_pointer_count = 0;

static const size_t WriteRsp_struct_bytes_count = 8;

Request_ptr new_Request(struct capn_segment*);
ReadReq_ptr new_ReadReq(struct capn_segment*);
ReadRsp_ptr new_ReadRsp(struct capn_segment*);
WriteReq_ptr new_WriteReq(struct capn_segment*);
WriteRsp_ptr new_WriteRsp(struct capn_segment*);

Request_list new_Request_list(struct capn_segment*, int len);
ReadReq_list new_ReadReq_list(struct capn_segment*, int len);
ReadRsp_list new_ReadRsp_list(struct capn_segment*, int len);
WriteReq_list new_WriteReq_list(struct capn_segment*, int len);
WriteRsp_list new_WriteRsp_list(struct capn_segment*, int len);

void read_Request(struct Request*, Request_ptr);
void read_ReadReq(struct ReadReq*, ReadReq_ptr);
void read_ReadRsp(struct ReadRsp*, ReadRsp_ptr);
void read_WriteReq(struct WriteReq*, WriteReq_ptr);
void read_WriteRsp(struct WriteRsp*, WriteRsp_ptr);

void write_Request(const struct Request*, Request_ptr);
void write_ReadReq(const struct ReadReq*, ReadReq_ptr);
void write_ReadRsp(const struct ReadRsp*, ReadRsp_ptr);
void write_WriteReq(const struct WriteReq*, WriteReq_ptr);
void write_WriteRsp(const struct WriteRsp*, WriteRsp_ptr);

void get_Request(struct Request*, Request_list, int i);
void get_ReadReq(struct ReadReq*, ReadReq_list, int i);
void get_ReadRsp(struct ReadRsp*, ReadRsp_list, int i);
void get_WriteReq(struct WriteReq*, WriteReq_list, int i);
void get_WriteRsp(struct WriteRsp*, WriteRsp_list, int i);

void set_Request(const struct Request*, Request_list, int i);
void set_ReadReq(const struct ReadReq*, ReadReq_list, int i);
void set_ReadRsp(const struct ReadRsp*, ReadRsp_list, int i);
void set_WriteReq(const struct WriteReq*, WriteReq_list, int i);
void set_WriteRsp(const struct WriteRsp*, WriteRsp_list, int i);

#ifdef __cplusplus
}
#endif
#endif
