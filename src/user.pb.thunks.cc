#include "user.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// user.User
        // clang-format off
extern "C" {
void* __rust_proto_thunk__user_User_new() { return new ::user::User(); }
void __rust_proto_thunk__user_User_delete(void* ptr) { delete static_cast<::user::User*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__user_User_serialize(::user::User* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__user_User_deserialize(::user::User* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__user_User_copy_from(::user::User* dst, const ::user::User* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__user_User_repeated_len(google::protobuf::RepeatedPtrField<::user::User>* field) {
  return field->size();
}
const ::user::User& __rust_proto_thunk__user_User_repeated_get(
  google::protobuf::RepeatedPtrField<::user::User>* field,
  size_t index) {
  return field->Get(index);
}
::user::User* __rust_proto_thunk__user_User_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::user::User>* field,
  size_t index) {
  return field->Mutable(index);
}
::user::User* __rust_proto_thunk__user_User_repeated_add(google::protobuf::RepeatedPtrField<::user::User>* field) {
  return field->Add();
}
void __rust_proto_thunk__user_User_repeated_clear(google::protobuf::RepeatedPtrField<::user::User>* field) {
  field->Clear();
}
void __rust_proto_thunk__user_User_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::user::User>& dst,
  const google::protobuf::RepeatedPtrField<::user::User>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__user_User_get_name(::user::User* msg) {
  absl::string_view val = msg->name();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__user_User_set_name(::user::User* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_name(absl::string_view(s.ptr, s.len));
}

::int32_t __rust_proto_thunk__user_User_get_age(::user::User* msg) { return msg->age(); }
void __rust_proto_thunk__user_User_set_age(::user::User* msg, ::int32_t val) {
  msg->set_age(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__user_User_get_email(::user::User* msg) {
  absl::string_view val = msg->email();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__user_User_set_email(::user::User* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_email(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on


// user.Users
        // clang-format off
extern "C" {
void* __rust_proto_thunk__user_Users_new() { return new ::user::Users(); }
void __rust_proto_thunk__user_Users_delete(void* ptr) { delete static_cast<::user::Users*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__user_Users_serialize(::user::Users* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__user_Users_deserialize(::user::Users* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__user_Users_copy_from(::user::Users* dst, const ::user::Users* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__user_Users_repeated_len(google::protobuf::RepeatedPtrField<::user::Users>* field) {
  return field->size();
}
const ::user::Users& __rust_proto_thunk__user_Users_repeated_get(
  google::protobuf::RepeatedPtrField<::user::Users>* field,
  size_t index) {
  return field->Get(index);
}
::user::Users* __rust_proto_thunk__user_Users_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::user::Users>* field,
  size_t index) {
  return field->Mutable(index);
}
::user::Users* __rust_proto_thunk__user_Users_repeated_add(google::protobuf::RepeatedPtrField<::user::Users>* field) {
  return field->Add();
}
void __rust_proto_thunk__user_Users_repeated_clear(google::protobuf::RepeatedPtrField<::user::Users>* field) {
  field->Clear();
}
void __rust_proto_thunk__user_Users_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::user::Users>& dst,
  const google::protobuf::RepeatedPtrField<::user::Users>& src) {
  dst = src;
}

void __rust_proto_thunk__user_Users_clear_users(::user::Users* msg) {
  msg->clear_users();
}
google::protobuf::RepeatedPtrField<::user::User>* __rust_proto_thunk__user_Users_get_mut_users(::user::Users* msg) {
  return msg->mutable_users();
}
const google::protobuf::RepeatedPtrField<::user::User>* __rust_proto_thunk__user_Users_get_users(
    const ::user::Users* msg) {
  return &msg->users();
}


}  // extern "C"
// clang-format on


