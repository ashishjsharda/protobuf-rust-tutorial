extern crate protobuf_cpp as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct User {
  inner: ::__pb::__runtime::MessageInner
}

// SAFETY:
// - `User` does not provide shared mutation with its arena.
// - `UserMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena that would conflict with
//   field access is impossible.
unsafe impl Sync for User {}

impl ::__pb::Proxied for User {
  type View<'msg> = UserView<'msg>;
  type Mut<'msg> = UserMut<'msg>;
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct UserView<'msg> {
  msg: ::__pb::__internal::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

#[allow(dead_code)]
impl<'msg> UserView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__internal::RawMessage) -> Self {
    Self { msg, _phantom: std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__internal::RawMessage {
    self.msg
  }

  // name: optional string
  pub fn name(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__user_User_get_name(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }


  // age: optional int32
  pub fn age(self) -> i32 {
    unsafe { __rust_proto_thunk__user_User_get_age(self.raw_msg()) }
  }

  // email: optional string
  pub fn email(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__user_User_get_email(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }


}

// SAFETY:
// - `UserView` does not perform any mutation.
// - While a `UserView` exists, a `UserMut` can't exist to mutate
//   the arena that would conflict with field access.
// - `UserMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for UserView<'_> {}
unsafe impl Send for UserView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for UserView<'msg> {
  type Proxied = User;

  fn as_view(&self) -> ::__pb::View<'msg, User> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, User> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for User {
  type VTable = ::__pb::__internal::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    UserView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    UserMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for User {
  type OptionalVTable = ::__pb::__internal::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for User {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, User>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, User>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer);
      (present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<User> for UserView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, User>)
    where User: 'dst {
    unsafe { __rust_proto_thunk__user_User_copy_from(mutator.inner.msg(), self.msg) };
  }
}
unsafe impl ::__pb::ProxiedInRepeated for User {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__user_User_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__user_User_copy_from(
        __rust_proto_thunk__user_User_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__user_User_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__user_User_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__user_User_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__user_User_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__user_User_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}

#[derive(Debug)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UserMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

#[allow(dead_code)]
impl<'msg> UserMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__internal::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  fn raw_msg(&self) -> ::__pb::__internal::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }


  // name: optional string
  pub fn name(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__user_User_get_name(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  pub fn name_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    static VTABLE: ::__pb::__internal::BytesMutVTable =
      ::__pb::__internal::BytesMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__user_User_get_name,
        __rust_proto_thunk__user_User_set_name,
      );
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          &VTABLE,
        )
      )
    }
  }

  // age: optional int32
  pub fn age(&self) -> i32 {
    unsafe { __rust_proto_thunk__user_User_get_age(self.raw_msg()) }
  }
  pub fn age_mut(&mut self) -> ::__pb::Mut<'_, i32> {
    static VTABLE: ::__pb::__internal::PrimitiveVTable<i32> =
      ::__pb::__internal::PrimitiveVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__user_User_get_age,
        __rust_proto_thunk__user_User_set_age,
      );

      // SAFETY:
      // - The message is valid for the output lifetime.
      // - The vtable is valid for the field.
      // - There is no way to mutate the element for the output
      //   lifetime except through this mutator.
      unsafe {
        ::__pb::PrimitiveMut::from_inner(
          ::__pb::__internal::Private,
          ::__pb::__internal::RawVTableMutator::new(
            ::__pb::__internal::Private,
            self.as_mutator_message_ref(),
            &VTABLE,
          ),
        )
      }
  }

  // email: optional string
  pub fn email(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__user_User_get_email(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  pub fn email_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    static VTABLE: ::__pb::__internal::BytesMutVTable =
      ::__pb::__internal::BytesMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__user_User_get_email,
        __rust_proto_thunk__user_User_set_email,
      );
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          &VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `UserMut` does not perform any shared mutation.
// - `UserMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for UserMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for UserMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, User> {
    UserMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, User> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for UserMut<'msg> {
  type Proxied = User;
  fn as_view(&self) -> ::__pb::View<'_, User> {
    UserView { msg: self.raw_msg(), _phantom: std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, User> where 'msg: 'shorter {
    UserView { msg: self.raw_msg(), _phantom: std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl User {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__user_User_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__internal::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__user_User_serialize(self.raw_msg()) }
  }
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__user_User_deserialize(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }

  pub fn as_view(&self) -> UserView {
    UserView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> UserMut {
    UserMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // name: optional string
  pub fn name(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__user_User_get_name(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  pub fn name_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    static VTABLE: ::__pb::__internal::BytesMutVTable =
      ::__pb::__internal::BytesMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__user_User_get_name,
        __rust_proto_thunk__user_User_set_name,
      );
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          &VTABLE,
        )
      )
    }
  }

  // age: optional int32
  pub fn age(&self) -> i32 {
    unsafe { __rust_proto_thunk__user_User_get_age(self.raw_msg()) }
  }
  pub fn age_mut(&mut self) -> ::__pb::Mut<'_, i32> {
    static VTABLE: ::__pb::__internal::PrimitiveVTable<i32> =
      ::__pb::__internal::PrimitiveVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__user_User_get_age,
        __rust_proto_thunk__user_User_set_age,
      );

      // SAFETY:
      // - The message is valid for the output lifetime.
      // - The vtable is valid for the field.
      // - There is no way to mutate the element for the output
      //   lifetime except through this mutator.
      unsafe {
        ::__pb::PrimitiveMut::from_inner(
          ::__pb::__internal::Private,
          ::__pb::__internal::RawVTableMutator::new(
            ::__pb::__internal::Private,
            self.as_mutator_message_ref(),
            &VTABLE,
          ),
        )
      }
  }

  // email: optional string
  pub fn email(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__user_User_get_email(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  pub fn email_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    static VTABLE: ::__pb::__internal::BytesMutVTable =
      ::__pb::__internal::BytesMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__user_User_get_email,
        __rust_proto_thunk__user_User_set_email,
      );
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          &VTABLE,
        )
      )
    }
  }

}  // impl User

impl ::__std::ops::Drop for User {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__user_User_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__user_User_new() -> ::__pb::__internal::RawMessage;
  fn __rust_proto_thunk__user_User_delete(raw_msg: ::__pb::__internal::RawMessage);
  fn __rust_proto_thunk__user_User_serialize(raw_msg: ::__pb::__internal::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__user_User_deserialize(raw_msg: ::__pb::__internal::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__user_User_copy_from(dst: ::__pb::__internal::RawMessage, src: ::__pb::__internal::RawMessage);
  fn __rust_proto_thunk__user_User_repeated_len(raw: ::__pb::__internal::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__user_User_repeated_add(raw: ::__pb::__internal::RawRepeatedField) -> ::__pb::__internal::RawMessage;
  fn __rust_proto_thunk__user_User_repeated_get(raw: ::__pb::__internal::RawRepeatedField, index: usize) -> ::__pb::__internal::RawMessage;
  fn __rust_proto_thunk__user_User_repeated_get_mut(raw: ::__pb::__internal::RawRepeatedField, index: usize) -> ::__pb::__internal::RawMessage;
  fn __rust_proto_thunk__user_User_repeated_clear(raw: ::__pb::__internal::RawRepeatedField);
  fn __rust_proto_thunk__user_User_repeated_copy_from(dst: ::__pb::__internal::RawRepeatedField, src: ::__pb::__internal::RawRepeatedField);

  fn __rust_proto_thunk__user_User_get_name(raw_msg: ::__pb::__internal::RawMessage) -> ::__pb::__internal::PtrAndLen;
  fn __rust_proto_thunk__user_User_set_name(raw_msg: ::__pb::__internal::RawMessage, val: ::__pb::__internal::PtrAndLen);

  fn __rust_proto_thunk__user_User_get_age(raw_msg: ::__pb::__internal::RawMessage) -> i32;
  fn __rust_proto_thunk__user_User_set_age(raw_msg: ::__pb::__internal::RawMessage, val: i32);

  fn __rust_proto_thunk__user_User_get_email(raw_msg: ::__pb::__internal::RawMessage) -> ::__pb::__internal::PtrAndLen;
  fn __rust_proto_thunk__user_User_set_email(raw_msg: ::__pb::__internal::RawMessage, val: ::__pb::__internal::PtrAndLen);


}  // extern "C" for User


impl User {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__internal::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(&mut self) -> ::__pb::__internal::RawMessage {
    self.raw_msg()
  }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Users {
  inner: ::__pb::__runtime::MessageInner
}

// SAFETY:
// - `Users` does not provide shared mutation with its arena.
// - `UsersMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena that would conflict with
//   field access is impossible.
unsafe impl Sync for Users {}

impl ::__pb::Proxied for Users {
  type View<'msg> = UsersView<'msg>;
  type Mut<'msg> = UsersMut<'msg>;
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct UsersView<'msg> {
  msg: ::__pb::__internal::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

#[allow(dead_code)]
impl<'msg> UsersView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__internal::RawMessage) -> Self {
    Self { msg, _phantom: std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__internal::RawMessage {
    self.msg
  }

  // users: repeated message user.User
  pub fn users(self) -> ::__pb::RepeatedView<'msg, crate::User> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        unsafe { __rust_proto_thunk__user_Users_get_users(self.raw_msg()) },
      )
    }
  }

}

// SAFETY:
// - `UsersView` does not perform any mutation.
// - While a `UsersView` exists, a `UsersMut` can't exist to mutate
//   the arena that would conflict with field access.
// - `UsersMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for UsersView<'_> {}
unsafe impl Send for UsersView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for UsersView<'msg> {
  type Proxied = Users;

  fn as_view(&self) -> ::__pb::View<'msg, Users> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Users> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for Users {
  type VTable = ::__pb::__internal::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    UsersView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    UsersMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for Users {
  type OptionalVTable = ::__pb::__internal::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for Users {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, Users>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, Users>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer);
      (present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<Users> for UsersView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Users>)
    where Users: 'dst {
    unsafe { __rust_proto_thunk__user_Users_copy_from(mutator.inner.msg(), self.msg) };
  }
}
unsafe impl ::__pb::ProxiedInRepeated for Users {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__user_Users_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__user_Users_copy_from(
        __rust_proto_thunk__user_Users_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__user_Users_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__user_Users_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__user_Users_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__user_Users_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__user_Users_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}

#[derive(Debug)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UsersMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

#[allow(dead_code)]
impl<'msg> UsersMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__internal::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  fn raw_msg(&self) -> ::__pb::__internal::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }


  // users: repeated message user.User
  pub fn users(&self) -> ::__pb::RepeatedView<'_, crate::User> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        unsafe { __rust_proto_thunk__user_Users_get_users(self.raw_msg()) },
      )
    }
  }
  pub fn users_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::User> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__user_Users_get_mut_users(self.raw_msg()),
        ),
      )
    }
  }

}

// SAFETY:
// - `UsersMut` does not perform any shared mutation.
// - `UsersMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for UsersMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for UsersMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, Users> {
    UsersMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, Users> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for UsersMut<'msg> {
  type Proxied = Users;
  fn as_view(&self) -> ::__pb::View<'_, Users> {
    UsersView { msg: self.raw_msg(), _phantom: std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Users> where 'msg: 'shorter {
    UsersView { msg: self.raw_msg(), _phantom: std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl Users {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__user_Users_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__internal::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__user_Users_serialize(self.raw_msg()) }
  }
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__user_Users_deserialize(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }

  pub fn as_view(&self) -> UsersView {
    UsersView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> UsersMut {
    UsersMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // users: repeated message user.User
  pub fn users(&self) -> ::__pb::RepeatedView<'_, crate::User> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        unsafe { __rust_proto_thunk__user_Users_get_users(self.raw_msg()) },
      )
    }
  }
  pub fn users_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::User> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__user_Users_get_mut_users(self.raw_msg()),
        ),
      )
    }
  }

}  // impl Users

impl ::__std::ops::Drop for Users {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__user_Users_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__user_Users_new() -> ::__pb::__internal::RawMessage;
  fn __rust_proto_thunk__user_Users_delete(raw_msg: ::__pb::__internal::RawMessage);
  fn __rust_proto_thunk__user_Users_serialize(raw_msg: ::__pb::__internal::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__user_Users_deserialize(raw_msg: ::__pb::__internal::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__user_Users_copy_from(dst: ::__pb::__internal::RawMessage, src: ::__pb::__internal::RawMessage);
  fn __rust_proto_thunk__user_Users_repeated_len(raw: ::__pb::__internal::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__user_Users_repeated_add(raw: ::__pb::__internal::RawRepeatedField) -> ::__pb::__internal::RawMessage;
  fn __rust_proto_thunk__user_Users_repeated_get(raw: ::__pb::__internal::RawRepeatedField, index: usize) -> ::__pb::__internal::RawMessage;
  fn __rust_proto_thunk__user_Users_repeated_get_mut(raw: ::__pb::__internal::RawRepeatedField, index: usize) -> ::__pb::__internal::RawMessage;
  fn __rust_proto_thunk__user_Users_repeated_clear(raw: ::__pb::__internal::RawRepeatedField);
  fn __rust_proto_thunk__user_Users_repeated_copy_from(dst: ::__pb::__internal::RawRepeatedField, src: ::__pb::__internal::RawRepeatedField);

  fn __rust_proto_thunk__user_Users_clear_users(raw_msg: ::__pb::__internal::RawMessage);
  fn __rust_proto_thunk__user_Users_get_mut_users(raw_msg: ::__pb::__internal::RawMessage) -> ::__pb::__internal::RawRepeatedField;
  fn __rust_proto_thunk__user_Users_get_users(raw_msg: ::__pb::__internal::RawMessage) -> ::__pb::__internal::RawRepeatedField;


}  // extern "C" for Users


impl Users {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__internal::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(&mut self) -> ::__pb::__internal::RawMessage {
    self.raw_msg()
  }
}

